use std::path::{Path, PathBuf};
use std::collections::HashMap;

use sqlite::{Connection, State};
use chrono::{NaiveDate, Datelike, Local};

use crate::events::{Event, Category, MonthDay};
use crate::providers::EventProvider;

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
}

impl EventProvider for SQLiteProvider {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn get_events(&self, events: &mut Vec<Event>) {
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

        //let where_clause = self.make_where_clause(filter, &category_map);

        let event_query: String = "SELECT event_date, event_description, category_id FROM event".to_string();
        //event_query.push(' ');
        //event_query.push_str(&where_clause);

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
}
