use bitflags::{bitflags, bitflags_match};
use crate::events::{Event, MonthDay, Category};

bitflags! {
    #[derive(Debug, Clone, PartialEq, Default)]
    pub struct FilterFlags: u32 {
        const NULL      = 0b00000000;
        const MONTH_DAY = 0b00000001;
        const CATEGORY  = 0b00000010;
        const TEXT      = 0b00000100;
    }
}

#[derive(Debug)]
pub struct EventFilter {
    flags: FilterFlags,
    month_day: MonthDay,
    category: Option<Category>,
    pattern: Option<String>,
}

impl EventFilter {
    pub fn accepts(&self, event: &Event) -> bool {
        bitflags_match!(self.flags, {
            FilterFlags::MONTH_DAY => {
                event.month_day() == self.month_day 
            },
            FilterFlags::CATEGORY => {
                if let Some(category) = &self.category {
                    event.category() == *category
                } else {
                    false
                }
            },
            FilterFlags::MONTH_DAY | FilterFlags::CATEGORY => {
                if let Some(category) = &self.category {
                    event.month_day() == self.month_day
                        && event.category() == *category
                } else {
                    false
                }
            },
            FilterFlags::TEXT => {
                if let Some(pattern) = &self.pattern {
                    event.description().contains(pattern)
                } else {
                    false
                }
            },
            FilterFlags::MONTH_DAY | FilterFlags::CATEGORY | FilterFlags::TEXT => {
                if let (Some(category), Some(pattern)) = (&self.category, &self.pattern) {
                    event.month_day() == self.month_day
                        && event.category() == *category
                        && event.description().contains(pattern)
                } else {
                    false
                }
            },
            FilterFlags::CATEGORY | FilterFlags::TEXT => {
                if let (Some(category), Some(pattern)) = (&self.category, &self.pattern) {
                    // in this case we don't care about the month-day
                    event.category() == *category
                        && event.description().contains(pattern)
                } else {
                    false
                }
            },
            FilterFlags::NULL => true,
            _ => false,
        })
    }

    pub fn flags(&self) -> FilterFlags {
        self.flags.clone()
    }

    pub fn month_day(&self) -> Option<MonthDay> {
        Some(self.month_day.clone())
    }

    pub fn category(&self) -> Option<Category> {
        self.category.clone()
    }

    pub fn pattern(&self) -> Option<String> {
        self.pattern.clone()
    }
}

pub struct FilterBuilder {
    flags: FilterFlags,
    month_day: MonthDay,
    category: Option<Category>,
    pattern: Option<String>,
}

impl FilterBuilder {
    pub fn new() -> FilterBuilder {
        FilterBuilder {
            flags: FilterFlags::NULL,
            month_day: MonthDay::new(1, 1),
            category: None,
            pattern: None,
        }
    }

    pub fn flags(mut self, flags: FilterFlags) -> FilterBuilder {
        self.flags = flags;
        self
    }

    pub fn month_day(mut self, month_day: MonthDay) -> FilterBuilder {
        self.month_day = month_day;
        self.flags.insert(FilterFlags::MONTH_DAY);
        self
    }

    pub fn category(mut self, category: Category) -> FilterBuilder {
        self.category = Some(category);
        self.flags.insert(FilterFlags::CATEGORY);
        self
    }

    pub fn pattern(mut self, pattern: String) -> FilterBuilder {
        self.pattern = Some(pattern);
        self.flags.insert(FilterFlags::TEXT);
        self
    }

    pub fn build(self) -> EventFilter {
        EventFilter {
            flags: self.flags,
            month_day: self.month_day,
            category: self.category,
            pattern: self.pattern,
        }
    }
}

// Implement event filter as a set:

/*
use std::collections::HashSet;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum Condition {
    Null,  // all pass filter
    Date(MonthDay),
    Category(Category),
    Text(String),
}

#[derive(Debug)]
pub struct EventFilter {
    conditions: HashSet<Condition>,
}

impl EventFilter {
    pub fn accepts(&self, event: &Event) -> bool {
        let mut results: Vec<bool> = Vec::new();

        for condition in &self.conditions {
            match condition {
                Condition::Null => results.push(true),
                Condition::Date(month_day) =>
                    results.push(event.month_day() == *month_day),
                Condition::Category(category) =>
                    results.push(event.category() == *category),
                Condition::Text(text) =>
                    results.push(event.description().contains(*&text))
            }
        }

        results.iter().all(|&x| x)
    }

    pub fn has_condition(self, condition: &Condition) -> bool {
        self.conditions.contains(condition)
    }
}

pub struct FilterBuilder {
    conditions: HashSet<Condition>,
}

impl FilterBuilder {
    pub fn new() -> FilterBuilder {
        let conditions: HashSet<Condition> = 
            vec![Condition::Null].into_iter().collect();

        FilterBuilder {
            conditions
        }
    }

    pub fn month_day(mut self, month_day: MonthDay) -> FilterBuilder {
        let condition = Condition::Date(month_day);
        if !self.conditions.contains(&condition) {
            self.conditions.insert(condition);
        }
        self
    }

    pub fn category(mut self, category: Category) -> FilterBuilder {
        let condition = Condition::Category(category);
        if !self.conditions.contains(&condition) {
            self.conditions.insert(condition);
        }
        self
    }

    pub fn description(mut self, text: String) -> FilterBuilder {
        let condition = Condition::Text(text);
        if !self.conditions.contains(&condition) {
            self.conditions.insert(condition);
        }
        self
    }

    pub fn build(self) -> EventFilter {
        EventFilter {
            conditions: self.conditions,
        }
    }
}
 */
