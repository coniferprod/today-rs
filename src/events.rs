use std::fmt;
use std::str::FromStr;

use chrono::{NaiveDate, Datelike, Local, Weekday, Month};
use strum_macros::EnumString;

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
    RuleBased(Rule),
}

#[derive(Debug, Clone, PartialEq, Eq, EnumString)]
pub enum Ordinal {
    First,
    Second,
    Third,
    Fourth,
    Last,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Rule {
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
    pub fn parse_opt(rule_string: &str) -> Option<Self> {
        // Parse a rule of the following format:
        // first|second|third|fourth|fifth|last <weekday> in <month>
        //   weekday: Monday|Tuesday|Wednesday|Thursday|Friday|Saturday|Sunday
        //   month: January|February|March| ... |November|December
        let parts: Vec<String> = rule_string.to_lowercase().split_whitespace()
            .map(str::to_string).collect();

        if parts.len() != 4 {
            eprintln!("invalid rule: {}", rule_string);
            return None;
        }

        /*
        let ordinal_string = parts[0];
        let weekday_string = parts[1];
        let month_string = parts[3];
        */

        /*
        let ordinals_str = vec!["first", "second", "third", "fourth", "fifth", "last"];
        let ordinals: Vec<_> = ordinals_str.into_iter().map(str::to_string).collect();
        if !ordinals.contains(&ordinal_string) {
            eprintln!("unrecognized ordinal {}", ordinal_string);
            return None;
        }
        let ordinal = ordinals.iter().position(|s| *s == ordinal_string);
         */
        let ordinal = match Ordinal::from_str(&parts[0]) {
            Ok(ord) => ord,
            Err(e) => {
                eprintln!("{}", e);
                return None;
            }
        };

        /*
        let weekdays_str = vec![
            "monday", "tuesday", "wednesday", "thursday", 
            "friday", "saturday", "sunday"
        ];
        let weekdays: Vec<_> = weekdays_str.into_iter().map(str::to_string).collect();
        if !weekdays.contains(&weekday_string) {
            eprintln!("unrecognized weekday {}", weekday_string);
            return None;
        }
        */

        let weekday = match parts[1].parse::<Weekday>() {
            Ok(wd) => wd,
            Err(e) => {
                eprintln!("{}", e);
                return None;
            }
        };

        /*
        let months_str = vec![
            "january", "february", "march", "april", "may", "june", 
            "july", "august", "september", "october", "november", "december"];
        let months: Vec<_> = months_str.into_iter().map(str::to_string).collect();
        if !months.contains(&month_string) {
            eprintln!("unknown month {}", month_string);
            return None;
        }

        let month_number = months.iter().position(|s| *s == month_string);
         */

        let month = match parts[2].parse::<Month>() {
            Ok(m) => m,
            Err(e) => {
                eprintln!("{}", e);
                return None;
            }
        };

        Some(Self { ordinal, weekday, month })
    }

    pub fn month_day(&self) -> Option<MonthDay> {
        match self.resolve_date() {
            Ok(date) => Some(MonthDay { month: date.month(), day: date.day() }),
            Err(e) => {
                eprintln!("{}", e);
                None
            }
        }
    }

    pub fn year(&self) -> i32 {
        Local::now().year()
    }

    pub fn resolve_date(&self) -> Result<NaiveDate, String> {
        let year = Local::now().year();

        match self.ordinal {
            Ordinal::First => nth_weekday_in_month(year, self.month, self.weekday, 1),
            Ordinal::Second => nth_weekday_in_month(year, self.month, self.weekday, 2),
            Ordinal::Third => nth_weekday_in_month(year, self.month, self.weekday, 3),
            Ordinal::Fourth => nth_weekday_in_month(year, self.month, self.weekday, 4),
            Ordinal::Last => last_weekday_in_month(year, self.month, self.weekday),
        }
    }
}

fn nth_weekday_in_month(year: i32, month: Month, weekday: Weekday, n: u32)
        -> Result<NaiveDate, String> {
    let mut count = 0;

    for day in 1..=31 {
        if let Some(date) = NaiveDate::from_ymd_opt(year, month.number_from_month(), day) {
            if date.weekday() == weekday {
                count += 1;
                if count == n {
                    return Ok(date);
                }
            }
        }
    }

    Err("No such weekday occurrence in month".into())
}

fn last_weekday_in_month(year: i32, month: Month, weekday: Weekday)
        -> Result<NaiveDate, String> {
    for day in (1..=31).rev() {
        if let Some(date) = NaiveDate::from_ymd_opt(year, month.number_from_month(), day) {
            if date.weekday() == weekday {
                return Ok(date);
            }
        }
    }

    Err("No matching weekday found".into())
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

    pub fn new_rule_based(rule: Rule, description: String, category: Category) -> Self {
        Event {
            kind: EventKind::RuleBased(rule),
            description, 
            category
        }
    }

    pub fn year(&self) -> i32 {
        let today: NaiveDate = Local::now().date_naive();
        match &self.kind {
            EventKind::Singular(date) => date.year(),
            EventKind::Annual(_month_day) => today.year(),
            EventKind::RuleBased(rule) => rule.year(),
        }
    }

    pub fn month_day(&self) -> MonthDay {
        match &self.kind {
            EventKind::Singular(date) => 
                MonthDay { month: date.month(), day: date.day() },
            EventKind::Annual(month_day) => 
                MonthDay { month: month_day.month, day: month_day.day },
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
    use crate::events::Rule;
    use chrono::NaiveDate;

    #[test]
    fn rejects_invalid_ordinal() {
        assert_eq!(Rule::parse_opt("sixth sunday in may"), None);
    }

    #[test]
    fn rejects_invalid_weekday() {
        assert_eq!(Rule::parse_opt("first bloomsday in june"), None);
    }

    #[test]
    fn rejects_invalid_month() {
        assert_eq!(Rule::parse_opt("first tuesday in remember"), None);
    }

    #[test]
    fn valid_date_from_rule() {
        // TODO: Rethink / rewrite this test

        let rule = Rule::parse_opt("second tuesday in may");
        match rule {
            Some(r) => {
                match r.resolve_date() {
                    Ok(date) => {
                        println!("resolved date = {}", date);
                        assert_eq!(date, NaiveDate::from_ymd_opt(2026, 5, 12).unwrap());
                    },
                    Err(e) => {
                        eprintln!("{}", e);
                        assert_ne!(0, 0);
                    }          
                }
            },
            None => {
                assert_ne!(0, 0);
            }
        }
    }
}
