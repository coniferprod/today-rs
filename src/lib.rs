mod birthday;
mod events;
mod providers;

use std::error::Error;
use std::path::{Path, PathBuf};
use chrono::{NaiveDate, Local, Datelike};
use crate::events::{Event, Category, MonthDay};
use crate::providers::{EventProvider, SimpleProvider};
use crate::providers::textfile::TextFileProvider;

pub fn run() -> Result<(), Box<dyn Error>> {
    birthday::handle_birthday();

    let mut events: Vec<Event> = Vec::new();

    let test_provider = SimpleProvider::new("test");
    test_provider.get_events(&mut events);
    
    let text_file_provider = TextFileProvider::new(
        "programming", Path::new("programming.txt"));
    text_file_provider.get_events(&mut events);

    let today: NaiveDate = Local::now().date_naive();
    let today_month_day = MonthDay::new(today.month(), today.day());

    for event in events {
        if today_month_day == event.month_day() {
            println!("{}", event);
        }
    }

    Ok(())
}
