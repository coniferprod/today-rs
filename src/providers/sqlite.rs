use std::path::{Path, PathBuf};
use std::collections::HashMap;

use sqlite::{Connection, State};
use chrono::{NaiveDate, Datelike, Local};
use bitflags::bitflags_match;

use crate::events::{Event, Category, MonthDay};
use crate::providers::EventProvider;
use crate::providers::EventProviderError;
use crate::filters::{EventFilter, FilterFlags};

pub struct SQLiteProvider {
    name: String,
    path: PathBuf,
}

impl SQLiteProvider {
    pub fn new(name: &str, path: &Path) -> Self {
        Self { 
            name: name.to_string(),
            path: path.to_path_buf() 
        }
    }

    fn make_date_part(&self, filter: &EventFilter) -> String {
        if let Some(month_day) = filter.month_day() {
            let md = format!("{:02}-{:02}", month_day.month(), month_day.day());
            format!("strftime('%m-%d', event_date) = '{}'", md)
        } else {
            "".to_string()
        }
    }

    fn make_category_part(&self, filter: &EventFilter, category_map: &HashMap<i64, Category>) -> String {
        if let Some(filter_category) = filter.category() {
            let mut filter_category_id: Option<i64> = None;

            // Brute force search for maching category:
            //eprintln!("Looking for categories in map...");
            for (category_id, category) in category_map {
                //eprintln!("{}: {}", category_id, category);
                if *category == filter_category {
                    filter_category_id = Some(*category_id);
                    //eprintln!("Found it!");
                    break;
                }
            }

            match filter_category_id {
                Some(id) => format!("category_id = {}", id),
                None => "".to_string(),
            }
        } else {
            "".to_string()
        }
    }

    fn make_text_part(&self, filter: &EventFilter) -> String {
        if let Some(pattern) = filter.pattern() {
            format!("event_description LIKE '%{}%'", pattern)
        } else {
            "".to_string()
        }
    }

    fn make_where_clause(&self, filter: &EventFilter, category_map: &HashMap<i64, Category>) -> String {
        let condition = 
            bitflags_match!(filter.flags(), {
                FilterFlags::MONTH_DAY => {
                    self.make_date_part(filter)
                },
                FilterFlags::CATEGORY => {
                    self.make_category_part(filter, category_map)
                },
                FilterFlags::MONTH_DAY | FilterFlags::CATEGORY => {
                    format!("{} AND {}", self.make_date_part(filter), self.make_category_part(filter, category_map))
                },
                FilterFlags::TEXT => {
                    self.make_text_part(filter)
                },
                FilterFlags::MONTH_DAY | FilterFlags::CATEGORY | FilterFlags::TEXT => {
                    format!(
                        "{} AND {} AND {}", 
                        self.make_date_part(filter), 
                        self.make_category_part(filter, category_map),
                        self.make_text_part(filter))
                },
                _ => "".to_string(),
            }).to_string();

        let mut result = "WHERE ".to_string();
        result.push_str(&condition);
        result
    }
}

impl EventProvider for SQLiteProvider {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn get_events(&self, filter: &EventFilter, events: &mut Vec<Event>) {
        let connection = Connection::open(self.path.clone()).unwrap();

        let mut category_map: HashMap<i64, Category> = HashMap::new();

        let category_query = "SELECT category_id, primary_name, secondary_name FROM category";
        let mut statement = connection.prepare(category_query).unwrap();
        while let Ok(State::Row) = statement.next() {
            let category_id = statement.read::<i64, _>("category_id").unwrap();
            let primary = statement.read::<String, _>("primary_name").unwrap();
            let secondary = statement.read::<Option<String>, _>("secondary_name").unwrap();

            let category = match secondary {
                Some(sec) => Category::new(&primary, &sec),
                None => Category::from_primary(&primary),
            };
            category_map.insert(category_id, category);
        }

        /*
        println!("Categories from the SQLite database:");
        for (key, value) in &category_map {
            println!("{key}: {value}");    
        }
         */

        let where_clause = self.make_where_clause(filter, &category_map);
        let mut event_query: String = "SELECT event_date, event_description, category_id FROM event".to_string();
        event_query.push(' ');
        event_query.push_str(&where_clause);

        eprintln!("SQLite database query: \"{}\"", event_query);

        for row in connection
            .prepare(event_query)
            .unwrap()
            .into_iter()
            .map(|row| row.unwrap()) 
        {
     
            let date = NaiveDate::parse_from_str(row.read::<&str, _>("event_date"), "%F").unwrap();
            let description = row.read::<&str, _>("event_description");
            let category_id = row.read::<i64, _>("category_id");
            let category = category_map.get(&category_id).unwrap();
            events.push(Event::new_singular(date, description.to_string(), category.clone()));
        }
    }

    fn add_event(&self, event: &Event) -> Result<(), EventProviderError> {
        Err(EventProviderError::OperationNotSupported)
    }
}
