use std::collections::HashSet;

use crate::events::{Event, MonthDay, Category};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FilterOption {
    MonthDay(MonthDay),
    Category(Category),
    Text(String),
}

pub struct EventFilter {
    options: HashSet<FilterOption>,
}

impl EventFilter {
    pub fn new() -> Self {
        Self {
            options: HashSet::new(),
        }
    }

    pub fn accepts(&self, event: &Event) -> bool {
        // If the option set is empty, this is an all-pass filter.
        if self.options.is_empty() {
            return false;
        }

        // Collect the results from various options into a vector.
        let mut results: Vec<bool> = Vec::new();

        for option in self.options.iter() {
            let result = match option {
                FilterOption::MonthDay(month_day) => {
                    *month_day == event.month_day()
                },
                FilterOption::Category(category) => {
                    *category == event.category()
                },
                FilterOption::Text(text) => {
                    event.description().contains(text)
                }
            };
            results.push(result);
        }

        // If the results vector contains only true values,
        // the event will be accepted, otherwise rejected.
        results.iter().all(|&option| option)
    }

    pub fn contains_month_day(&self) -> bool {
        self.options.iter().any(|option| matches!(option, &FilterOption::MonthDay(_)))    
    }

    pub fn contains_category(&self) -> bool {
        self.options.iter().any(|option| matches!(option, &FilterOption::Category(_)))
    }

    pub fn contains_text(&self) -> bool {
        self.options.iter().any(|option| matches!(option, &FilterOption::Text(_)))
    }

    pub fn month_day(&self) -> Option<MonthDay> {
        for option in self.options.iter() {
            match option {
                FilterOption::MonthDay(month_day) => return Some(month_day.clone()),
                _ => (),
            }
        }

        // All checked, not found
        None
    }

    pub fn category(&self) -> Option<Category> {
        for option in self.options.iter() {
            match option {
                FilterOption::Category(category) => return Some(category.clone()),
                _ => (),
            }
        }
        None
    }

    pub fn text(&self) -> Option<String> {
        for option in self.options.iter() {
            match option {
                FilterOption::Text(text) => return Some(text.clone()),
                _ => (),
            }
        }
        None
    }
}

pub struct FilterBuilder {
    options: HashSet<FilterOption>,
}

impl FilterBuilder {
    pub fn new() -> FilterBuilder {
        FilterBuilder {
            options: HashSet::new(),
        }
    }

    pub fn month_day(mut self, month_day: MonthDay) -> FilterBuilder {
        self.options.insert(FilterOption::MonthDay(month_day));
        self
    }

    pub fn category(mut self, category: Category) -> FilterBuilder {
        self.options.insert(FilterOption::Category(category));
        self
    }

    pub fn text(mut self, text: String) -> FilterBuilder {
        self.options.insert(FilterOption::Text(text));
        self
    }

    pub fn build(self) -> EventFilter {
        EventFilter {
            options: self.options,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{NaiveDate, Local, Datelike};

    #[test]
    fn filter_contains_month_day() {
        let today: NaiveDate = Local::now().date_naive();
        let md: MonthDay = MonthDay::new(today.month(), today.day());

        let filter = FilterBuilder::new()
            .month_day(md)
            .build();
        assert!(filter.contains_month_day());
    }
}
