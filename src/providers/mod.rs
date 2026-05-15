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
    fn kind(&self) -> String;
    fn is_active(&self) -> bool;
}

pub enum EventProviderError {
    OperationNotSupported,
    OperationFailed,
}
