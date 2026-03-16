use std::path::{Path, PathBuf};
use std::collections::HashMap;

use sqlite::{Connection, State};
use chrono::NaiveDate;
use bitflags::bitflags_match;
use log;

use crate::events::{Event, Category};
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


    fn get_categories(&self, connection: &Connection) -> HashMap<i64, Category> {
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

        category_map
    }

    fn find_category_id(&self, category: &Category, connection: &Connection) -> Option<i64> {
        // Get the categories currently in the database.
        let category_map = self.get_categories(connection);

        // Use a brute force method where you iterate the hash map keys,
        // and stop if one of the values matches the category.
        for (id, cat) in &category_map {
            if *cat == *category { // found it!
                return Some(*id);
            }
        }

        None
    }
}

impl EventProvider for SQLiteProvider {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn get_events(&self, filter: &EventFilter, events: &mut Vec<Event>) {
        let connection = Connection::open(self.path.clone()).unwrap();
        let category_map = self.get_categories(&connection);

        let where_clause = make_where_clause(filter, &category_map);
        let mut event_query: String = "SELECT event_date, event_description, category_id FROM event".to_string();
        event_query.push(' ');
        event_query.push_str(&where_clause);

        log::info!("SQLite database query: \"{}\"", event_query);

        let mut statement = connection.prepare(event_query).unwrap();
        while let Ok(State::Row) = statement.next() {
            let date_string = statement.read::<String, _>("event_date").unwrap();
            let date = NaiveDate::parse_from_str(&date_string, "%F").unwrap();
            let description = statement.read::<String, _>("event_description").unwrap();
            let category_id = statement.read::<i64, _>("category_id").unwrap();
            let category = category_map.get(&category_id).unwrap();
            events.push(Event::new_singular(date, description.to_string(), category.clone()));
        }
    }

    fn add_event(&self, event: &Event) -> Result<(), EventProviderError> {
        let connection = Connection::open(self.path.clone()).unwrap();

        // Find out if the category of the event is already there.
        match self.find_category_id(&event.category(), &connection) {
            Some(category_id) => {
                // Found it, now insert the event to the database with this category ID:
                let event_date_str = format!("{:04}-{}", event.year(), event.month_day());
                let insert_query = format!("INSERT INTO event (event_date, event_description, category_id) VALUES ('{}', '{}', {})", 
                    event_date_str, event.description(), category_id);
                let connection = Connection::open(self.path.clone()).unwrap();
                log::info!("Found existing category, about to run query: '{}'", insert_query);
                connection.execute(insert_query).unwrap();
            },
            None => {
                // Add a new category first, then add the event
                let category = event.category();
                let primary_str = category.primary();
                let secondary_str = match category.secondary() {
                    Some(secondary) => format!("'{}'", secondary),
                    None => "NULL".to_string()
                };
                let insert_category_query = format!("INSERT INTO category (primary_name, secondary_name) VALUES ('{}', {})",
                    primary_str, secondary_str);
                let connection = Connection::open(self.path.clone()).unwrap();
                log::info!("Existing category not found, about to run query '{}'", insert_category_query);
                connection.execute(insert_category_query).unwrap();

                // Looks like the sqlite crate does not have a way of getting the ID
                // of the last inserted row, so we need to fetch the categories again
                // and look for the newly inserted row...
                match self.find_category_id(&category, &connection) {
                    Some(category_id) => {
                        // We have a category ID, let's insert the event:
                        let event_date_str = format!("{:04}-{}", event.year(), event.month_day());
                        let insert_event_query = format!("INSERT INTO event (event_date, event_description, category_id) VALUES ('{}', '{}', '{}')", 
                            event_date_str, event.description(), category_id);
                        log::info!("Existing category found, about to run query '{}'", insert_event_query);
                        connection.execute(insert_event_query).unwrap();
                    },
                    None => {
                        eprintln!("Unable to find inserted category!");
                        return Err(EventProviderError::OperationFailed);
                    }
                }

                // The database connection should be automatically dropped right about here...
                return Ok(());
            }
        }

        Err(EventProviderError::OperationNotSupported)
    }

    // Override the default implementation from the trait:
    fn is_add_supported(&self) -> bool { true }
}

fn make_date_part(filter: &EventFilter) -> String {
    let md = format!("{:02}-{:02}", filter.month_day().month(), filter.month_day().day());
    format!("strftime('%m-%d', event_date) = '{}'", md)
}

fn make_category_part(filter: &EventFilter, category_map: &HashMap<i64, Category>) -> String {
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

fn make_text_part(filter: &EventFilter) -> String {
    if let Some(text) = filter.text() {
        format!("event_description LIKE '%{}%'", text)
    } else {
        "".to_string()
    }
}

fn make_where_clause(filter: &EventFilter, category_map: &HashMap<i64, Category>) -> String {
    let condition = 
        bitflags_match!(filter.flags(), {
            FilterFlags::CATEGORY => {
                make_category_part(filter, category_map)
            },
            FilterFlags::TEXT => {
                make_text_part(filter)
            },
            FilterFlags::CATEGORY | FilterFlags::TEXT => {
                format!(
                    "{} AND {}", 
                    make_category_part(filter, category_map),
                    make_text_part(filter))
            },
            _ => "".to_string(),
        }).to_string();

    let date_part = make_date_part(filter);
    let mut result = format!("WHERE {}", date_part);
    if condition != "" {
        result.push_str(&format!(" AND {}", condition));
    }
    result
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::{events::Category, filters::FilterBuilder};
    use crate::events::MonthDay;
    use chrono::{NaiveDate, Datelike};

    // Creates an in-memory SQLite database with some tables,
    // then inserts one category (id=1, primary=test, secondary=NULL)
    // and one event.
    fn create_memory_db() -> sqlite::Connection {
        let connection = sqlite::open(":memory:").unwrap();

        let query = "
CREATE TABLE IF NOT EXISTS event(
    event_id INTEGER PRIMARY KEY,
    event_date DATE NOT NULL,
    event_description TEXT NOT NULL,
    category_id INTEGER NOT NULL,
    FOREIGN KEY (category_id) REFERENCES category(category_id));
CREATE TABLE IF NOT EXISTS category(
    category_id INTEGER PRIMARY KEY,
    primary_name TEXT NOT NULL,
    secondary_name TEXT);
INSERT INTO category VALUES (1, 'test', NULL);
INSERT INTO event (event_date, event_description, category_id)
    VALUES ('2026-03-07', 'Unit test for SQLiteProvider', 1);
";

        connection.execute(query).unwrap();
        connection
    }

    #[test]
    fn get_categories() -> Result<(), String> {
        let connection = create_memory_db();
        let category_query = "SELECT category_id, primary_name, secondary_name FROM category";
        let mut statement = connection.prepare(category_query).unwrap();

        if let Ok(sqlite::State::Row) = statement.next() {
            let actual = (
                statement.read::<i64, _>("category_id").unwrap(),
                statement.read::<String, _>("primary_name").unwrap(),
                statement.read::<Option<String>, _>("secondary_name").unwrap());
            let expected = (1, "test".to_string(), None);
            assert_eq!(expected, actual);
            Ok(())
        } else {
            Err("Unable to get category from database".to_string())
        }
    }

    #[test]
    fn get_events() -> Result<(), String> {
        let connection = create_memory_db();
        let event_query = "SELECT event_date, event_description, category_id FROM event".to_string();
        let mut statement = connection.prepare(event_query).unwrap();
        if let Ok(sqlite::State::Row) = statement.next() {
            let expected = ("2026-03-07".to_string(), "Unit test for SQLiteProvider".to_string(), 1);
            let actual = (
                statement.read::<String, _>("event_date").unwrap(),
                statement.read::<String, _>("event_description").unwrap(),
                statement.read::<i64, _>("category_id").unwrap());
            assert_eq!(expected, actual);
            Ok(())
        } else {
            Err("Unable to retrieve event from database".to_string())
        }
    }

    #[test]
    fn make_date_part() {
        let today = NaiveDate::from_ymd_opt(2026, 3, 7).unwrap();
        let filter = FilterBuilder::new(MonthDay::new(today.month(), today.day()))
            .build();        
        let date_part = crate::providers::sqlite::make_date_part(&filter);
        assert_eq!(date_part, "strftime('%m-%d', event_date) = '03-07'")
    }

    #[test]
    fn make_category_part_nonempty() {
        let mut category_map: HashMap<i64, Category> = HashMap::new();
        category_map.insert(1, Category::from_primary("test"));
        let today = NaiveDate::from_ymd_opt(2026, 3, 7).unwrap();
        let filter = FilterBuilder::new(MonthDay::new(today.month(), today.day()))
            .category(Category::from_primary("test"))
            .build();
        let category_part = crate::providers::sqlite::make_category_part(&filter, &category_map);
        assert_eq!(category_part, "category_id = 1");
    }

    #[test]
    fn make_category_part_empty() {
        let category_map: HashMap<i64, Category> = HashMap::new();
        let today = NaiveDate::from_ymd_opt(2026, 3, 7).unwrap();
        let filter = FilterBuilder::new(MonthDay::new(today.month(), today.day()))
            .build();
        let category_part = crate::providers::sqlite::make_category_part(&filter, &category_map);
        assert_eq!(category_part, "");
    }
}
