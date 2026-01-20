mod birthday;
mod events;
mod providers;
mod filters;

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
use crate::filters::{EventFilter, FilterBuilder};

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

    let filter: EventFilter = FilterBuilder::new().build();

    let today: NaiveDate = Local::now().date_naive();
    let filter: EventFilter = FilterBuilder::new()
        .month_day(MonthDay::new(today.month(), today.day()))
        .build();

    let mut count = 0;
    for provider in providers {
        provider.get_events(&filter, &mut events);  // polymorphism!
        let new_count = events.len();
        println!(
            "Got {} events from provider '{}'", 
            new_count - count,
            provider.name());
        count = new_count;
    }

    // Now we only have events that have already been filtered for today,
    // but we exclude "test/fake":
    println!("\nEvents for today (no test/fake category):");
    let test_fake_category = Category::new("test", "fake");
    for event in &events {
        if event.category() == test_fake_category {
            continue;
        }
        println!("{}", event);
    }

    Ok(())
}
