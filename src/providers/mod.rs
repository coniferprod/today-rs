use chrono::{NaiveDate, Local};
use crate::events::{Event, Category};

pub mod textfile;

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
