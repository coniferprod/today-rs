use chrono::NaiveDate;
use reqwest::{blocking::Client, blocking::Response};
use serde::Deserialize;

use crate::events::{Event, Category};
use crate::providers::EventProvider;
use crate::providers::EventProviderError;
use crate::filters::EventFilter;

pub struct WebProvider {
    name: String,
    url: String,
}

#[derive(Deserialize, Debug)]
struct JSONEvent {
    category: String,
    date: String,
    description: String,
}

impl WebProvider {
    pub fn new(name: &str, url: &str) -> Self {
        Self { 
            name: name.to_string(),
            url: url.to_string()
        }
    }
}

impl EventProvider for WebProvider {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn get_events(&self, filter: &EventFilter, events: &mut Vec<Event>) {
        let month_day = filter.month_day();

        let date_parameter = format!(
            "date={:02}-{:02}", 
            month_day.month(), 
            month_day.day());

        let url = format!("{}?{}", &self.url, date_parameter);
        log::info!("web URL = {}", &url);

        let client = Client::new();
        let request = client.get(&url).send();

        let response: Response;
        if request.is_err() {
            eprintln!("Error while retrieving data: {:#?}", request.err());
            return;
        } else {
            response = request.ok().unwrap();
        }

        let json_events = response.json::<Vec<JSONEvent>>().unwrap();
        //println!("Got {} events from JSON", json_events.len());
        //println!("body = {:?}", response.text().unwrap());
        //eprintln!("JSON = {:?}", json);

        for json_event in json_events {
            let date = NaiveDate::parse_from_str(&json_event.date, "%F").unwrap();            
            let category = Category::from_str(&json_event.category);
            let event = Event::new_singular(date, json_event.description, category);
            if filter.accepts(&event) {
                events.push(event);
            }
        }
    }

    fn add_event(&self, event: &Event) -> Result<(), EventProviderError> {
        Err(EventProviderError::OperationNotSupported)
    }    
}
