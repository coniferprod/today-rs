use crate::events::{Event, MonthDay, Category};

pub struct EventFilter {
    month_day: Option<MonthDay>,
    description_contains: Option<String>,
    category_matches: Option<Category>,
}

impl EventFilter {
    pub fn month_day(&self) -> Option<MonthDay> {
        self.month_day.clone()
    }

    pub fn description_contains(&self) -> Option<String> {
        self.description_contains.clone()
    }

    pub fn category_matches(&self) -> Option<Category> {
        self.category_matches.clone()
    }    

    pub fn accepts(&self, event: &Event) -> bool {
        if let Some(month_day) = &self.month_day {
            if event.month_day() != *month_day {
                return false;
            }
        }

        if let Some(ref text) = self.description_contains {
            if !event.description().to_lowercase().contains(text) {
                return false;
            }
        }

        if let Some(ref filter_cat) = self.category_matches {
            if event.category().primary() != filter_cat.primary() {
                return false;
            }

            if let Some(ref sec) = filter_cat.secondary() {
                if event.category().secondary().as_ref() != Some(sec) {
                    return false;
                }
            }
        }

        true
    }
}

pub struct FilterBuilder {
    month_day: Option<MonthDay>,
    description_contains: Option<String>,
    category_matches: Option<Category>,
}

impl FilterBuilder {
    pub fn new() -> Self {
        Self {
            month_day: None,
            description_contains: None,
            category_matches: None,
        }
    }

    pub fn month_day(mut self, month_day: MonthDay) -> Self {
        self.month_day = Some(month_day);
        self
    }

    pub fn description_contains(mut self, text: impl Into<String>) -> Self {
        self.description_contains = Some(text.into().to_lowercase());
        self
    }

    pub fn category_matches(mut self, category: &Category) -> Self {
        self.category_matches = Some(category.clone());
        self
    }

    pub fn build(self) -> EventFilter {
        EventFilter {
            month_day: self.month_day,
            description_contains: self.description_contains,
            category_matches: self.category_matches,
        }
    }    
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

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
            .category_matches(&rust_category)
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
            .description_contains("Rust".to_string())
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
        let contains = (
            filter.month_day(),
            filter.category_matches(),
            filter.description_contains());
        assert_eq!(contains, (None, None, None));
    }

    #[test]
    fn build_filter_month_day_only() {
        let filter = FilterBuilder::new()
            .month_day(MonthDay::new(3, 17))
            .build();
        let contains = (
            filter.month_day(),
            filter.category_matches(),
            filter.description_contains());
        assert_eq!(contains, (Some(MonthDay::new(3, 17)), None, None));
    }
}
