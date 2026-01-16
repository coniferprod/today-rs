mod birthday;
mod events;

use std::error::Error;
use chrono::{NaiveDate, Local, Datelike};
use crate::events::{Event, Category, MonthDay};

pub fn run() -> Result<(), Box<dyn Error>> {
    birthday::handle_birthday();

    let mut events = vec![
        Event::new_singular(
            NaiveDate::from_ymd_opt(2025, 12, 11).expect("valid date"),
            String::from("Rust 1.92.0 released"),
            Category::new("programming", "rust")),

        Event::new_singular(
            NaiveDate::from_ymd_opt(1996, 1, 23).expect("valid date"),
            String::from("JDK 1.0 released"),
            Category::new("programming", "java")),

        Event::new_singular(
            NaiveDate::from_ymd_opt(2008, 12, 3).expect("valid date"),
            String::from("Python 3.0 released"),
            Category::new("programming", "python")),
            
        Event::new_singular(
            NaiveDate::from_ymd_opt(2015, 5, 15).expect("valid date"),
            String::from("Rust 1.0.0 released"),
            Category::new("programming", "rust")),

        Event::new_singular(
            NaiveDate::from_ymd_opt(2025, 9, 16).expect("valid date"),
            String::from("Java 25 released"),
            Category::new("programming", "java")),

        Event::new_singular(
            NaiveDate::from_ymd_opt(2025, 10, 7).expect("valid date"),
            String::from("Python 3.14 released"),
            Category::new("programming", "python")),

            Event::new_singular(
            NaiveDate::from_ymd_opt(2025, 12, 11).expect("valid date"),
            String::from("Rust 1.92.0 released"),
            Category::new("programming", "rust")),            
    ];

    let today: NaiveDate = Local::now().date_naive();
    let today_month_day = MonthDay::new(today.month(), today.day());

    let test_event = Event::new_singular(
        today, 
        String::from("Test event for today"), 
        Category::from_primary("test")
    );
    events.push(test_event);

    for event in events {
        if today_month_day == event.month_day() {
            println!("{}", event);
        }
    }

    Ok(())
}
