use chrono::{NaiveDate, Local};
use crate::events::{Event, Category};

pub trait EventProvider {
    fn name(&self) -> String;
    fn get_events(&self, events: &mut Vec<Event>);
}

pub struct SimpleProvider {
    name: String,
}

impl SimpleProvider {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string() }
    }
}

impl EventProvider for SimpleProvider {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn get_events(&self, events: &mut Vec<Event>) {
        let today: NaiveDate = Local::now().date_naive();

        let test_event = Event::new_singular(
            today, 
            String::from("Test event for today"), 
            Category::from_primary("test")
        );
        events.push(test_event);
    }
}

pub mod textfile {
    use std::path::{Path, PathBuf};
    use std::fs::File;
    use std::io::{BufReader, BufRead};
    use std::fmt;

    use chrono::NaiveDate;

    use crate::EventProvider;
    use crate::events::{Event, Category};

    enum ReadingState {
        Date,
        Description,
        Category,
        Separator,
    }

    impl fmt::Display for ReadingState {
        fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
            write!(f, "{}", match self {
                ReadingState::Date => "DATE",
                ReadingState::Description => "DESCRIPTION",
                ReadingState::Category => "CATEGORY",
                ReadingState::Separator => "SEPARATOR",
            })
        }
    }

    pub struct TextFileProvider {
        name: String,
        path: PathBuf,
    }

    impl TextFileProvider {
        pub fn new(name: &str, path: &Path) -> Self {
            Self { name: name.to_string(), path: path.to_path_buf() }
        }
    }

    impl EventProvider for TextFileProvider {
        fn name(&self) -> String {
            self.name.clone()
        }

        fn get_events(&self, events: &mut Vec<Event>) {
            let f = File::open(self.path.clone()).expect("path to text file");
            let reader = BufReader::new(f);
            let mut state = ReadingState::Date;
            let mut date_string = String::new();
            let mut description = String::new();
            let mut category_string = String::new();

            for line_result in reader.lines() {
                let line = line_result.expect("read line");
                match state {
                    ReadingState::Date => {
                        date_string = line;
                        state = ReadingState::Description;
                    },
                    ReadingState::Description => {
                        description = line;
                        state = ReadingState::Category;
                    },
                    ReadingState::Category => {
                        category_string = line;
                        state = ReadingState::Separator;
                    },
                    ReadingState::Separator => {
                        match NaiveDate::parse_from_str(&date_string, "%F") {
                            Ok(date) => {
                                let category = Category::from_str(&category_string);
                                let event = Event::new_singular(date, description.clone(), category);
                                events.push(event);
                            },
                            Err(_) => {
                               eprintln!("Invalid timestamp '{}'", date_string);
                            }
                        }
                        state = ReadingState::Date;
                    },
                }
            }
        }
    }
}