mod birthday;
mod events;
mod providers;

use std::error::Error;
use std::path::{Path, PathBuf};
use chrono::{NaiveDate, Local, Datelike};
use serde::Deserialize;
use crate::events::{Event, Category, MonthDay};
use crate::providers::{EventProvider, SimpleProvider};
use crate::providers::textfile::TextFileProvider;
use crate::providers::csvfile::CSVFileProvider;
use crate::providers::sqlite::SQLiteProvider;
use crate::providers::web::WebProvider;

#[derive(Deserialize, Debug)]
pub struct ProviderConfig {
    name: String,
    kind: String,
    resource: String,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    providers: Vec::<ProviderConfig>,
}

pub fn run(config: &Config, config_path: &Path) -> Result<(), Box<dyn Error>> {
    birthday::handle_birthday();

    // Try to create all the event providers specified in `config`.
    // Put them in a vector of trait objects.
    let mut providers: Vec::<Box<dyn EventProvider>> = Vec::new();
    for cfg in config.providers.iter() {
        let path = config_path.join(&cfg.resource);
        match cfg.kind.as_str() {
            "text" => {
                let provider = TextFileProvider::new(&cfg.name, &path);
                providers.push(Box::new(provider));
            },
            "csv" => {
                let provider = CSVFileProvider::new(&cfg.name, &path);
                providers.push(Box::new(provider));
            },
            "sqlite" => {
                let provider = SQLiteProvider::new(&cfg.name, &path);
                providers.push(Box::new(provider));
            },
            "web" => {
                let provider = WebProvider::new(&cfg.name, &cfg.resource);
                providers.push(Box::new(provider));
            }
            _ => {
                eprintln!("Unable to make provider: {:?}", cfg);
            }
        }
    }

    let test_provider = SimpleProvider::new("test");
    providers.push(Box::new(test_provider));

    let mut events: Vec<Event> = Vec::new();

    // Collect all events from all providers
    let mut count = 0;
    for provider in providers {
        provider.get_events(&mut events);  // polymorphism!
        let new_count = events.len();
        println!(
            "Got {} events from provider '{}'", 
            new_count - count,
            provider.name());
        count = new_count;
    }

    let today: NaiveDate = Local::now().date_naive();
    let today_month_day = MonthDay::new(today.month(), today.day());

    println!("\nEvents for today:");
    for event in &events {
        if today_month_day == event.month_day() {
            println!("{}", event);
        }
    }

    /*
    println!("\nAll events from all providers:");
    for event in &events {
        println!("{}", event);
    }
    */

    Ok(())
}
