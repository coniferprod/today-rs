use std::path::{Path, PathBuf};

use log;

use crate::events::Event;
use crate::filters::EventFilter;
use crate::ProviderConfig;
use crate::providers::{
    EventProvider,
    textfile::TextFileProvider,
    csvfile::CSVFileProvider,
    sqlite::SQLiteProvider,
    web::WebProvider
};

pub struct EventManager {
    config_path: PathBuf,
    providers: Vec::<Box<dyn EventProvider>>,
}

impl EventManager {
    pub fn new(config_path: &Path) -> Self {
        log::debug!("Making new EventManager, config_path = '{:?}'", config_path.to_str());
        Self {
            config_path: config_path.to_path_buf(),
            providers: Vec::new(),
        }
    }

    pub fn add_provider(&mut self, config: &ProviderConfig) -> bool {
        let path = self.config_path.join(&config.resource);
        match config.kind.as_str() {
            "text" => {
                let provider = TextFileProvider::new(&config.name, &path);
                self.providers.push(Box::new(provider));
                true
            },
            "csv" => {
                let provider = CSVFileProvider::new(&config.name, &path);
                self.providers.push(Box::new(provider));
                true
            },
            "sqlite" => {
                let provider = SQLiteProvider::new(&config.name, &path);
                self.providers.push(Box::new(provider));
                true
            },
            "web" => {
                let provider = WebProvider::new(&config.name, &config.resource);
                self.providers.push(Box::new(provider));
                true
            },
            _ => {
                eprintln!("Unable to make provider: {:?}", config);
                false
            }
        }
    }

    pub fn get_providers(&self) -> Vec<(String, bool)> {
        let mut result: Vec<(String, bool)> = Vec::new();

        for provider in &self.providers {
            result.push((provider.name(), provider.is_add_supported()));
        }

        result
    }

    /// Gets all events from all event providers that match the filter,
    /// and adds them to the supplied vector.
    pub fn get_events(&self, filter: &EventFilter) -> Vec<Event> {
        let mut result: Vec<Event> = Vec::new();

        let mut count = 0;

        for provider in &self.providers {
            provider.get_events(&filter, &mut result);
            let new_count = result.len();
            log::info!(
                "Got {} events from provider '{}'", 
                new_count - count,
                provider.name());
            count = new_count;
        }

        result
    }
}
