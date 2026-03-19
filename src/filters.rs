use std::collections::HashSet;

use crate::events::{Event, MonthDay, Category};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FilterOption {
    MonthDay(MonthDay),
    Category(Category),
    Text(String),
}

#[derive(Debug)]
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
            return true;
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

        // If the results vector contains only `true` values,
        // all the options match, and the event will be accepted, 
        // otherwise it will be rejected by the filter.
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

    #[test]
    fn filter_accepts_month_day() {
        let event = Event::new_singular(
            NaiveDate::from_ymd_opt(2026, 3, 17).unwrap(), 
            "Test event for March 17".to_string(), 
            Category::from_primary("test"));
        let filter = FilterBuilder::new()
            .month_day(MonthDay::new(3, 17))
            .build();
        assert!(filter.accepts(&event));
    }

    #[test]
    fn filter_accepts_category() {
        let rust_category = Category::new("programming", "rust");
        let event = Event::new_singular(
            NaiveDate::from_ymd_opt(2026, 3, 5).unwrap(), 
            "Rust 1.94.0 released".to_string(), 
            rust_category.clone());
        let filter = FilterBuilder::new()
            .category(rust_category.clone())
            .build();
        assert!(filter.accepts(&event));
    }

    #[test]
    fn filter_accepts_text() {
        let rust_category = Category::new("programming", "rust");
        let event = Event::new_singular(
            NaiveDate::from_ymd_opt(2026, 3, 5).unwrap(), 
            "Rust 1.94.0 released".to_string(), 
            rust_category.clone());
        let filter = FilterBuilder::new()
            .text("Rust".to_string())
            .build();
        assert!(filter.accepts(&event));
    }

    #[test]
    fn filter_accepts_anything() {
        let rust_category = Category::new("programming", "rust");
        let event = Event::new_singular(
            NaiveDate::from_ymd_opt(2026, 3, 5).unwrap(), 
            "Rust 1.94.0 released".to_string(), 
            rust_category.clone());
        let filter = FilterBuilder::new()
            .build();
        assert!(filter.accepts(&event));
    }

    #[test]
    fn build_filter_no_options() {
        let filter = FilterBuilder::new()
            .build();
        let contains = [
            filter.contains_month_day(),
            filter.contains_category(),
            filter.contains_text()
        ];
        assert_eq!(contains, [false, false, false]);
    }

    #[test]
    fn build_filter_month_day_only() {
        let filter = FilterBuilder::new()
            .month_day(MonthDay::new(3, 17))
            .build();
        let contains = [
            filter.contains_month_day(),
            filter.contains_category(),
            filter.contains_text()
        ];
        assert_eq!(contains, [true, false, false]);
    }
}
