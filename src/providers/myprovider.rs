use crate::EventProvider;
use crate::events::Event;

pub struct MyProvider {
    name: String
}

impl MyProvider {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string() }
    }
}

impl EventProvider for MyProvider {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn get_events(&self, events: &mut Vec<Event>) {
        // pass
    }
}