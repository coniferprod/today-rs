//! # providers
//! 
//! The `providers` module defines the `EventProvider` trait
//! and collects all the modules for the event providers that 
//! implement the trait.

use chrono::{NaiveDate, Local};
use crate::events::{Event, Category};
use crate::filters::EventFilter;

pub mod textfile;
pub mod csvfile;
pub mod sqlite;
pub mod web;
pub mod xmlfile;

pub trait EventProvider {
    fn name(&self) -> String;
    fn get_events(&self, filter: &EventFilter, events: &mut Vec<Event>);
    fn is_add_supported(&self) -> bool { false }
    fn add_event(&self, event: &Event) -> Result<(), EventProviderError>;
}

pub enum EventProviderError {
    OperationNotSupported,
    OperationFailed,
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

    fn get_events(&self, _filter: &EventFilter, events: &mut Vec<Event>) {
        let today: NaiveDate = Local::now().date_naive();

        let test_event = Event::new_singular(
            today, 
            String::from("Test event for today"), 
            Category::from_primary("test")
        );
        events.push(test_event);
    }

    fn add_event(&self, event: &Event) -> Result<(), EventProviderError> {
        Err(EventProviderError::OperationNotSupported)
    }
}
