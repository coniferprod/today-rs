use bitflags::{bitflags, bitflags_match};

use crate::events::{Event, MonthDay, Category};

bitflags! {
    #[derive(Debug, Clone, PartialEq, Default)]
    pub struct FilterFlags: u32 {
        const NULL      = 0b00000000;
        const CATEGORY  = 0b00000001;
        const TEXT      = 0b00000010;
    }
}

#[derive(Debug)]
pub struct EventFilter {
    flags: FilterFlags,
    month_day: MonthDay,
    category: Option<Category>,
    text: Option<String>,
}

impl EventFilter {
    pub fn accepts(&self, event: &Event) -> bool {
        // If the day does not match, don't accept the event.
        if event.month_day() != self.month_day {
            return false;
        }

        // Now we know that the month-day matches;
        // check any additional conditions.
        bitflags_match!(self.flags, {
            FilterFlags::CATEGORY => {
                if let Some(category) = &self.category {
                    event.category() == *category
                } else {
                    false
                }
            },
            FilterFlags::TEXT => {
                if let Some(text) = &self.text {
                    event.description().contains(text)
                } else {
                    false
                }
            },
            FilterFlags::CATEGORY | FilterFlags::TEXT => {
                if let (Some(category), Some(text)) = (&self.category, &self.text) {
                    event.category() == *category
                        && event.description().contains(text)
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

    pub fn month_day(&self) -> MonthDay {
        self.month_day.clone()
    }

    pub fn category(&self) -> Option<Category> {
        self.category.clone()
    }

    pub fn text(&self) -> Option<String> {
        self.text.clone()
    }
}

pub struct FilterBuilder {
    flags: FilterFlags,
    month_day: MonthDay,
    category: Option<Category>,
    text: Option<String>,
}

impl FilterBuilder {
    pub fn new(month_day: MonthDay) -> FilterBuilder {
        FilterBuilder {
            flags: FilterFlags::NULL,
            month_day,
            category: None,
            text: None,
        }
    }

    pub fn flags(mut self, flags: FilterFlags) -> FilterBuilder {
        self.flags = flags;
        self
    }

    pub fn category(mut self, category: Category) -> FilterBuilder {
        self.category = Some(category);
        self.flags.insert(FilterFlags::CATEGORY);
        self
    }

    pub fn text(mut self, text: String) -> FilterBuilder {
        self.text = Some(text);
        self.flags.insert(FilterFlags::TEXT);
        self
    }

    pub fn build(self) -> EventFilter {
        EventFilter {
            flags: self.flags,
            month_day: self.month_day,
            category: self.category,
            text: self.text,
        }
    }
}
