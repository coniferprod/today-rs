use std::path::{Path, PathBuf};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufRead, BufWriter, Write};
use std::fmt;
use std::thread::current;

use chrono::{NaiveDate, Local, Datelike};
use xml::reader::{EventReader, XmlEvent};
use log;

use crate::EventProvider;
use crate::events::{Event, Category, MonthDay, EventKind};
use crate::filters::EventFilter;
use crate::providers::EventProviderError;

pub struct XMLFileProvider {
    name: String,
    path: PathBuf,
}

impl XMLFileProvider {
    pub fn new(name: &str, path: &Path) -> Self {
        Self { name: name.to_string(), path: path.to_path_buf() }
    }
}

impl EventProvider for XMLFileProvider {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn get_events(&self, filter: &EventFilter, events: &mut Vec<Event>) {
        let file = File::open(self.path.clone()).expect("path to text file");
        let file = BufReader::new(file);
        let parser = EventReader::new(file);

        let mut depth = 0;

        let mut xml_events: Vec<Event> = Vec::new();

        let mut current_event: Event;
        let mut current_date: Option<NaiveDate> = None;

        let mut content = String::new();
        for e in parser {
            let mut current_description: Option<String> = None;
            let mut current_category: Option<Category> = None;
    
            match e {
                Ok(XmlEvent::StartElement { name, .. }) => {
                    match name.local_name.as_str() {
                        "events" => {
                            xml_events.clear();
                        },
                        "event" => {
                            current_description = None;
                            //content.clear();
                        },
                        "date" => {
                            current_date = None;
                            //content.clear();
                        },
                        "description" => {
                            current_description = Some(String::new());
                            //content.clear();
                        },
                        "category" => {
                            current_category = None;
                            //content.clear();
                        },
                        _ => {}
                    }

                    println!("{:spaces$}+{name}", "", spaces = depth * 2);
                    depth += 1;
                }

                Ok(XmlEvent::EndElement { name }) => {
                    match name.local_name.as_str() {
                        "events" => {
                            return;
                        },
                        "event" => {
                            current_event = Event::new_singular(
                                current_date.unwrap(), 
                                current_description.unwrap(), 
                                current_category.unwrap());
                            xml_events.push(current_event);
                        },
                        "date" => {
                            log::debug!("Parsing date element, content = '{}'", &content);
                            current_date = Some(NaiveDate::parse_from_str(&content, "%F").expect("invalid date"));
                        },
                        "description" => {
                            current_description = Some(content);
                        },
                        "category" => {
                            current_category = Some(Category::from_str(&content));
                        },
                        _ => ()
                    }

                    depth -= 1;
                    println!("{:spaces$}-{name}", "", spaces = depth * 2);
                }

                Ok(XmlEvent::Characters(s)) => {
                    log::debug!("Characters: '{}'", s);
                    content.push_str(&s);
                }

                Err(e) => {
                    eprintln!("Error: {e}");
                    break;
                }

                _ => {}
            }
        }
    }

    fn is_add_supported(&self) -> bool { false }

    fn add_event(&self, event: &Event) -> Result<(), EventProviderError> {
        Err(EventProviderError::OperationNotSupported)
    }
}
