use std::env;
use chrono::{Local, NaiveDate, Datelike};

pub fn handle_birthday() {
    const NAME: &str = "BIRTHDATE";
    let value = env::var(NAME);
    if !value.is_ok() {
        return;
    }

    let value = value.unwrap();

    match NaiveDate::parse_from_str(&value, "%F") {
        Ok(birthdate) => {
            let mut result = String::new();

            let today: NaiveDate = Local::now().date_naive();
            if birthdate.month() == today.month() && birthdate.day() == today.day() {
                result.push_str("Happy birthday! ");
            }

            // Compute the difference between the birthdate and today.
            // Show a message depending on the sign of the result.
            let diff = today.signed_duration_since(birthdate);
            let day_count = diff.num_days();
            result.push_str(&make_message(day_count));
            println!("{}\n", result);
        },
        Err(_) => {
            eprintln!("Error in the '{}' environment variable: \
                '{}' is not a valid date.", NAME, value);
        }
    }
}

fn make_message(day_count: i64) -> String {
    let mut message = String::new();

    if day_count > 0 {
        message.push_str(&format!("You are {} days old.", day_count));
        if day_count % 1000 == 0 {
            message.push_str(" That's a nice, round number!");
        }
    } else if day_count < 0 {
        message.push_str("Are you from the future?");
    } else {  // must be zero
        message.push_str("Looks like you're new here.");
    }

    message
}
