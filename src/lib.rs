//! # The library crate
//! 
//! This crate defines the program configuration items and
//! has the `run` function to actually run the program.

mod birthday;
pub mod events;
pub mod providers;
pub mod filters;

use std::error::Error;
use std::path::Path;
use serde::Deserialize;
use crate::events::{Event, EventKind, Category};
use crate::providers::{
    EventProvider, 
    textfile::TextFileProvider,
    csvfile::CSVFileProvider,
    sqlite::SQLiteProvider,
    web::WebProvider,
};
use crate::filters::EventFilter;

#[derive(Deserialize, Debug)]
pub struct ProviderConfig {
    pub name: String,
    kind: String,
    resource: String,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub providers: Vec::<ProviderConfig>,
}

pub fn create_providers(config: &Config, config_path: &Path) -> Vec::<Box<dyn EventProvider>> {
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
            },
            _ => {
                eprintln!("Unable to make provider: {:?}", cfg);
            }
        }
    }

    /*
    let test_provider = SimpleProvider::new("test");
    providers.push(Box::new(test_provider));
    */

    providers
}

pub fn run(config: &Config, config_path: &Path, filter: &EventFilter)
        -> Result<(), Box<dyn Error>> {
    birthday::handle_birthday();

    let mut events: Vec<Event> = Vec::new();

    let providers = create_providers(config, config_path);

    let mut count = 0;
    for provider in providers {
        provider.get_events(&filter, &mut events);  // polymorphism!
        let new_count = events.len();
        log::info!(
            "Got {} events from provider '{}'", 
            new_count - count,
            provider.name());
        count = new_count;
    }

    let test_fake_category = Category::new("test", "fake");

    let mut singular_events: Vec<&Event> = Vec::new();
    let mut annual_events: Vec<&Event> = Vec::new();
    for event in &events {
        // Filter out "test/fake" events
        /*
        if event.category() == test_fake_category {
            continue;
        }
        */
        
        match event.kind() {
            EventKind::Singular(_) => singular_events.push(event),
            EventKind::Annual(_) | EventKind::RuleBased(_) =>
                annual_events.push(event)
        }
    }

    if singular_events.len() > 0 {
        singular_events.sort_by(|a, b| a.year().cmp(&b.year()));
        singular_events.reverse();
        println!("On this day in history ({}):", singular_events.len());
        for event in singular_events {
            println!("{}", event);
        }
    }

    if annual_events.len() > 0 {
        println!("\nObserved today ({}):", annual_events.len());
        for event in annual_events {
            println!("{} ({})", event.description(), event.category());
        }
    }

    // Now we only have events that have already been filtered for today,
    // but we exclude "test/fake":
    /*
    println!("\nEvents for today (no test/fake category):");
    let test_fake_category = Category::new("test", "fake");
    for event in &events {
        if event.category() == test_fake_category {
            continue;
        }
        println!("{}", event);
    }
     */

    Ok(())
}

pub fn add_event(config: &Config, config_path: &Path, provider_name: &str, event: &Event) {
    let providers = create_providers(config, config_path);

    // Find provider by name
    let mut provider: Option<&dyn EventProvider> = None;
    for p in &providers {
        if p.name() == provider_name {
            provider = Some(p.as_ref());
            break;
        }
    }

    match provider {
        Some(p) => {
            if p.is_add_supported() {
                let _ = p.add_event(event);
            } else {
                println!("Adding events is not supported for provider '{}'", p.name());
            }
        },
        None => {
            eprintln!("Unknown event provider '{}'", provider_name);
        }
    }
}
