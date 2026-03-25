use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::BufReader;

use chrono::{NaiveDate, Local};
use xml::reader::{EventReader, XmlEvent};
use log;

use crate::EventProvider;
use crate::events::{Event, Category};
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

        let mut current_date: NaiveDate = Local::now().date_naive();
        let mut current_description: String = String::new();
        let mut current_category: Category = Category::from_primary("test");
        let mut category_string = String::new();

        let mut content = String::new();
        for e in parser {
            match e {
                Ok(XmlEvent::StartElement { name, .. }) => {
                    content.clear();
                    match name.local_name.as_str() {
                        "description" => current_description.clear(),
                        "category" => category_string.clear(),
                        _ => {}
                    }
                }

                Ok(XmlEvent::EndElement { name }) => {
                    match name.local_name.as_str() {
                        "events" => return,
                        "event" => {
                            let event = Event::new_singular(
                                current_date, 
                                current_description.clone(), 
                                current_category.clone());
                            log::debug!("New event: {}", &event);
                            if filter.accepts(&event) {
                                events.push(event);
                            } else {
                                log::debug!("Filter rejects event");
                            }
                        },
                        "date" => {
                            log::debug!("end element 'date': content={}", &content);
                            current_date = NaiveDate::parse_from_str(&content, "%F")
                                .expect("invalid date");
                            content.clear();
                        },
                        "description" => {
                            current_description = content.clone();
                            content.clear();
                        },
                        "category" => {
                            log::debug!("end category, category_string = '{}'", &category_string);
                            current_category = Category::from_str(&category_string);
                        },
                        "primary" => {
                            log::debug!("end element 'primary' content = '{}'", &content);
                            category_string.push_str(&content);
                        },
                        "secondary" => {
                            log::debug!("end element 'secondary' content = '{}'", &content);
                            category_string.push('/');
                            category_string.push_str(&content);
                        },
                        _ => ()
                    }
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

    fn add_event(&self, _event: &Event) -> Result<(), EventProviderError> {
        Err(EventProviderError::OperationNotSupported)
    }
}
