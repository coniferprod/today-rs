use chrono::{
    Datelike, 
    NaiveDate,
    DateTime,
    Utc,
};

#[derive(Debug)]
struct MonthDay {
    month: u32,
    day: u32,
}

#[derive(Debug)]
struct Category {
    primary: String,
    secondary: String
}

impl Category {
    fn parse_from_str(category_str: &str) -> Category {
        let parts: Vec<&str> = category_str.split("/").collect();
        Category { primary: parts[0].to_string(), secondary: parts[1].to_string() }
    }
}

#[derive(Debug)]
enum Event {
    Singular { date: NaiveDate, description: String, category: Category },
    Annual { month_day: MonthDay, description: String, category: Category },
}

impl Event {
    fn new(date_str: &str, description_str: &str, category_str: &str) -> Event {
        let category = Category::parse_from_str(category_str);
        if date_str.starts_with("--") {
            let mut full_date_str = "2025".to_string();
            full_date_str.push_str(&date_str[1..]);
            let full_date = NaiveDate::parse_from_str(full_date_str.as_str(), "%Y-%m-%d").unwrap();
            let date = NaiveDate::from_ymd_opt(full_date.year(), full_date.month(), full_date.day()).unwrap();
            let month_day = MonthDay { month: date.month(), day: date.day() };
            Event::Annual { month_day, description: description_str.to_string(), category }
        } else {
            let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d").unwrap();
            Event::Singular { date, description: description_str.to_string(), category }
        }
    }
}

fn main() {
    let events = vec![
        Event::new("2001-04-07", "NASA launches the 2001 Mars Odyssey orbiter", "space/nasa"),
        Event::new("--04-01", "All Fools' Day", "observance/society"),
        Event::new("1973-04-06", "Launch of Pioneer 11 spacecraft", "space/nasa"),
    ];

    let now = chrono::Utc::now();
    for event in events {
        if is_same_day(&event, &now) {
            match event {
                Event::Singular { date, description, category } => {
                    println!("{}: {} ({}/{})", date.year(), description, category.primary, category.secondary);
                },
                Event::Annual { month_day: _, description, category } => {
                    println!("{} ({}/{})", description, category.primary, category.secondary);
                }
            }
        }
    }
}

fn is_same_day(event: &Event, date: &DateTime<Utc>) -> bool {
    let month: u32;
    let day: u32;
    match event {
        Event::Singular { date, description: _, category: _ } => {
            month = date.month();
            day = date.day();
        },
        Event::Annual { month_day, description: _, category: _ } => {
            month = month_day.month;
            day = month_day.day;
        }
    }

    day == date.day() && month == date.month()
}
