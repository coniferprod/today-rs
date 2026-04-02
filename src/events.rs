use std::fmt;
use std::str::FromStr;

use chrono::{
    NaiveDate, 
    Datelike, 
    Weekday as ChronoWeekday, 
    Local, 
    Month
};
use strum_macros::EnumString;

/*
The chrono::Weekday does not implement PartialOrd or Ord, 
citing the reason as: "The order of the days of week 
depends on the context."
 */

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, EnumString)]
#[strum(ascii_case_insensitive)]
pub enum Weekday {
    Monday = 0,
    Tuesday = 1,
    Wednesday = 2,
    Thursday = 3,
    Friday = 4,
    Saturday = 5,
    Sunday = 6,
}

impl Weekday {
    pub fn as_chrono_weekday(&self) -> ChronoWeekday {
        match *self {
            Weekday::Monday => ChronoWeekday::Mon,
            Weekday::Tuesday => ChronoWeekday::Tue,
            Weekday::Wednesday => ChronoWeekday::Wed,
            Weekday::Thursday => ChronoWeekday::Thu,
            Weekday::Friday => ChronoWeekday::Fri,
            Weekday::Saturday => ChronoWeekday::Sat,
            Weekday::Sunday => ChronoWeekday::Sun,
        }
    }
}

#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
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

// Display trait implemented for construcing an SQLite date
// when adding events with SQLiteProvider:
impl fmt::Display for MonthDay {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}-{:02}", self.month, self.day)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
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

    // Accessor functions added due to adding events in SQLiteProvider
    pub fn primary(&self) -> String {
        self.primary.clone()
    }

    pub fn secondary(&self) -> Option<String> {
        self.secondary.clone()
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

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum EventKind {
    Singular(NaiveDate),
    Annual(MonthDay),
    RuleBased(Rule),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, EnumString)]
#[strum(ascii_case_insensitive)]
pub enum Ordinal {
    First = 1,
    Second = 2,
    Third = 3,
    Fourth = 4,
    Last = 5,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Rule {
    ordinal: Ordinal,
    weekday: Weekday,
    month: Month,
}

/*/
impl FromStr for Rule {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<&str> = input.to_lowercase().split_whitespace().collect();
        if tokens.len() != 4 {
            return Err("invalid rule format".into());
        }

        let ordinal = match tokens[0].as_str() {
            "first" => Ordinal::First,
            "second" => Ordinal::Second,
            "third" => Ordinal::Third,
            "fourth" => Ordinal::Fourth,
            "last" => Ordinal::Last,
            _ => return Err("invalid ordinal").into()
        };

    }
}
*/

impl Rule {
    // Parse a rule of the following format:
    // first|second|third|fourth|fifth|last <weekday> in <month>
    //   weekday: Monday|Tuesday|Wednesday|Thursday|Friday|Saturday|Sunday
    //   month: January|February|March| ... |November|December
    pub fn parse(rule_string: &str) -> Option<Self> {
        let parts: Vec<String> = rule_string.to_lowercase().split_whitespace()
            .map(str::to_string).collect();

        // After splitting on whitespace, there must be exactly four parts.
        if parts.len() != 4 {
            eprintln!("invalid rule: {}", rule_string);
            return None;
        }

        let ordinal = match Ordinal::from_str(&parts[0]) {
            Ok(ord) => ord,
            Err(e) => {
                eprintln!("{}", e);
                return None;
            }
        };

        let weekday = match Weekday::from_str(&parts[1]) {
            Ok(wd) => wd,
            Err(e) => {
                eprintln!("{}", e);
                return None;
            }
        };

        if parts[2] != "in" && parts[2] != "of" {
            eprintln!("rule should specify `in` or `of`");
            return None;
        }

        let month = match parts[3].parse::<Month>() {
            Ok(m) => m,
            Err(e) => {
                eprintln!("{}", e);
                return None;
            }
        };

        Some(Self { ordinal, weekday, month })
    }

    pub fn month_day(&self) -> Option<MonthDay> {
        if let Some(date) = self.resolve_date(self.year()) {
            Some(MonthDay { month: date.month(), day: date.day() })
        } else {
            None
        }
    }

    pub fn year(&self) -> i32 {
        Local::now().year()
    }

    pub fn resolve_date(&self, year: i32) -> Option<NaiveDate> {
        if self.ordinal == Ordinal::Last {
            last_weekday_in_month(year, self.month, self.weekday)
        } else {
            nth_weekday_in_month(year, self.month, self.weekday, self.ordinal)
        }
        /*
        match self.ordinal {
            Ordinal::
            Ordinal::First => nth_weekday_in_month(year, self.month, self.weekday, 1),
            Ordinal::Second => nth_weekday_in_month(year, self.month, self.weekday, 2),
            Ordinal::Third => nth_weekday_in_month(year, self.month, self.weekday, 3),
            Ordinal::Fourth => nth_weekday_in_month(year, self.month, self.weekday, 4),
            Ordinal::Last => last_weekday_in_month(year, self.month, self.weekday),
        };
        None
         */
    }
}

//fn nth_weekday_in_month(year: i32, month: Month, weekday: Weekday, n: u32)
fn nth_weekday_in_month(year: i32, month: Month, weekday: Weekday, ordinal: Ordinal)
        -> Option<NaiveDate> {
    let mut count = 0;

    for day in 1..=31 {
        if let Some(date) = NaiveDate::from_ymd_opt(year, month.number_from_month(), day) {
            if date.weekday() == weekday.as_chrono_weekday() {
                count += 1;
                if count == ordinal as i32 {
                    return Some(date);
                }
            }
        }
    }

    None
}

fn last_weekday_in_month(year: i32, month: Month, weekday: Weekday)
        -> Option<NaiveDate> {
    for day in (1..=31).rev() {
        if let Some(date) = NaiveDate::from_ymd_opt(year, month.number_from_month(), day) {
            if date.weekday() == weekday.as_chrono_weekday() {
                return Some(date);
            }
        }
    }

    None
}


#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
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

    pub fn new_rule_based(rule: Rule, description: String, category: Category) -> Self {
        Event {
            kind: EventKind::RuleBased(rule),
            description, 
            category
        }
    }

    pub fn year(&self) -> i32 {
        match &self.kind {
            EventKind::Singular(date) => date.year(),
            EventKind::Annual(_month_day) => {
                let today: NaiveDate = Local::now().date_naive();
                today.year()
            },
            EventKind::RuleBased(rule) => rule.year(),
        }
    }

    pub fn month_day(&self) -> MonthDay {
        match &self.kind {
            EventKind::Singular(date) => MonthDay::new(date.month(), date.day()),
            EventKind::Annual(month_day) => month_day.clone(),
            EventKind::RuleBased(rule) => {
                match rule.month_day() {
                    Some(month_day) => month_day,
                    None => panic!("invalid month day resolved from rule"),
                }
            }
        }
    } 

    pub fn category(&self) -> Category {
        self.category.clone()
    }

    pub fn description(&self) -> String {
        self.description.clone()
    }

    pub fn kind(&self) -> EventKind {
        self.kind.clone()
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

#[cfg(test)]
mod tests {
    use crate::events::{Rule, last_weekday_in_month, nth_weekday_in_month, Ordinal, Weekday};
    use chrono::{NaiveDate, Month};

    #[test]
    fn rejects_invalid_ordinal() {
        assert_eq!(Rule::parse("sixth sunday in may"), None);
    }

    #[test]
    fn rejects_invalid_weekday() {
        assert_eq!(Rule::parse("first bloomsday in june"), None);
    }

    #[test]
    fn rejects_invalid_month() {
        assert_eq!(Rule::parse("first tuesday in remember"), None);
    }

    #[test]
    fn valid_date_from_rule() {
        let rule = Rule::parse("second tuesday in may").unwrap();
        assert_eq!(Some(rule.resolve_date(2026)), Some(NaiveDate::from_ymd_opt(2026, 5, 12)));
    }

    #[test]
    fn last_weekday() {
        // last Monday in January 2026 should be 2026-01-26
        let target = NaiveDate::from_ymd_opt(2026, 1, 26);
        assert_eq!(
            Some(last_weekday_in_month(2026, Month::January, Weekday::Monday)),
            Some(target)
        );
    }

    #[test]
    fn first_weekday() {
        // first Friday in May 2026 should be 2026-05-01
        let target = NaiveDate::from_ymd_opt(2026, 5, 1);
        assert_eq!(
            Some(nth_weekday_in_month(2026, Month::May, Weekday::Friday, Ordinal::First)),
            Some(target)
        );
    }
}
