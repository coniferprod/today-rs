use std::fmt;

use chrono::{NaiveDate, Datelike, Local, Weekday};

#[derive(Debug, PartialEq, Clone)]
pub struct MonthDay {
    month: u32,
    day: u32,
}

impl MonthDay {
    pub fn new(month: u32, day: u32) -> Self {
        Self { month, day }
    }

    pub fn from_str(s: &str) -> Self {
        assert!(s.len() == 4);
        let month_string = &s[..2];
        let month = month_string.parse().unwrap();
        let day: u32 = s[2..].parse().unwrap();
        MonthDay { month, day }
    }

    pub fn month(&self) -> u32 { self.month }
    pub fn day(&self) -> u32 { self.day }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Category {
    primary: String,
    secondary: Option<String>,
}

impl Category {
    pub fn new(primary: &str, secondary: &str) -> Self {
        Self {
            primary: primary.to_string(),
            secondary: Some(secondary.to_string())
        }
    }

    pub fn from_primary(primary: &str) -> Self {
        Self {
            primary: primary.to_string(),
            secondary: None
        }
    }

    pub fn from_str(s: &str) -> Category {
        let parts: Vec<&str> = s.split("/").collect();
        if parts.len() < 2 {
            Category { primary: parts[0].to_string(), secondary: None }
        } else {
            Category { primary: parts[0].to_string(), secondary: Some(parts[1].to_string()) }
        }
    }
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.secondary {
            Some(sec) => write!(f, "{}/{}", self.primary, sec),
            None => write!(f, "{}", self.primary),
        }
    }
}

#[derive(Debug)]
pub enum EventKind {
    Singular(NaiveDate),
    Annual(MonthDay),
}

#[derive(Debug)]
pub struct Event {
    kind: EventKind,
    description: String,
    category: Category,
}

impl Event {
    pub fn new_singular(date: NaiveDate, description: String, category: Category) -> Self {
        Event { 
            kind: EventKind::Singular(date),
            description, 
            category
        }
    }

    pub fn new_annual(month_day: MonthDay, description: String, category: Category) -> Self {
        Event {
            kind: EventKind::Annual(month_day),
            description,
            category
        }
    }

    pub fn year(&self) -> i32 {
        let today: NaiveDate = Local::now().date_naive();
        match &self.kind {
            EventKind::Singular(date) => date.year(),
            EventKind::Annual(_month_day) => today.year()
        }
    }

    pub fn month_day(&self) -> MonthDay {
        match &self.kind {
            EventKind::Singular(date) => 
                MonthDay { month: date.month(), day: date.day() },
            EventKind::Annual(month_day) => 
                MonthDay { month: month_day.month, day: month_day.day },
        }
    } 

    pub fn category(&self) -> Category {
        self.category.clone()
    }

    pub fn description(&self) -> String {
        self.description.clone()
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {} ({})",
            self.year(),
            self.description,
            self.category)
    }
}
