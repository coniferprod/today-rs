use std::fs::OpenOptions;
use std::io::BufWriter;
use std::path::{Path, PathBuf};
use chrono::{NaiveDate, Local, Datelike};
use csv::ReaderBuilder;

use crate::EventProvider;
use crate::providers::EventProviderError;
use crate::events::{Event, EventKind, Category, MonthDay, Rule};
use crate::filters::EventFilter;

pub struct CSVFileProvider {
    name: String,
    path: PathBuf,
}

impl CSVFileProvider {
    pub fn new(name: &str, path: &Path) -> Self {
        Self { name: name.to_string(), path: path.to_path_buf() }
    }
}

impl EventProvider for CSVFileProvider {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn get_events(&self, filter: &EventFilter, events: &mut Vec<Event>) {
        let mut reader = ReaderBuilder::new()
            .has_headers(false)
            .from_path(self.path.clone()).expect("existing CSV file");

        for result in reader.records() {
            let record = result.unwrap();

            let mut date_string = record[0].to_string();
            let description = record[1].to_string();
            let category_string = record[2].to_string();

            let event: Event;
            let category = Category::from_str(&category_string);

            // Check if the date string starts with a letter:
            let is_rule_based = date_string.chars().next().unwrap().is_alphabetic();
            if is_rule_based {
                event = Event::new_rule_based(
                    Rule::parse(&date_string).unwrap(), 
                    description, 
                    category);
                if filter.accepts(&event) {
                    events.push(event);
                }
                continue;
            }

            let is_yearless = date_string.starts_with("--");
            if is_yearless {
                let today: NaiveDate = Local::now().date_naive();
                let year_string = format!("{:04}-", today.year());
                date_string = date_string.replace("--", &year_string);
            }
            
            match NaiveDate::parse_from_str(&date_string, "%F") {
                Ok(date) => {
                    if is_yearless {
                        event = Event::new_annual(
                            MonthDay::new(date.month(), date.day()),
                            description.clone(), 
                            category);
                    } else {
                        event = Event::new_singular( 
                            date, 
                            description.clone(), 
                            category);
                    }
                    if filter.accepts(&event) {
                        events.push(event);
                    }
                },
                Err(_) => {
                    eprintln!("Invalid timestamp '{}'", date_string);
                }
            }
        }
    }

    fn is_add_supported(&self) -> bool { true }

    fn add_event(&self, event: &Event) -> Result<(), EventProviderError> {
        if !self.is_add_supported() {
            return Err(super::EventProviderError::OperationNotSupported);
        }

        let file = OpenOptions::new()
            .append(true)
            .open(self.path.clone())
            .expect("path to text file for writing");

        let writer = BufWriter::new(file);
        let mut csv_writer = csv::Writer::from_writer(writer);

        let date_string = match event.kind() {
            EventKind::Singular(date) => {
                date.format("%Y-%m-%d").to_string()
            },
            _ => {
                return Err(EventProviderError::OperationNotSupported);
            }
        };

        csv_writer.write_record([
            date_string, 
            event.description(), 
            format!("{}", event.category())
        ]).unwrap();
        
        csv_writer.flush().unwrap();

        Ok(())
    }

    fn kind(&self) -> String { String::from("CSV") }
}
