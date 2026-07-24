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
        } else if is_prime(day_count as u32) {
            message.push_str(" That's a prime number!");
        }
        if is_palindrome(day_count as u32) {
            message.push_str(" That's a palindrome!");
        }
    } else if day_count < 0 {
        message.push_str("Are you from the future?");
    } else {  // must be zero
        message.push_str("Looks like you're new here.");
    }

    message
}

// Check if `n` is a prime number.
fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }

    if n == 2 {
        return true;
    }

    if n % 2 == 0 {
        return false;
    }

    let mut d = 3;
    while d * d <= n {
        if n % d == 0 {
            return false;
        }
        d += 2;
    }

    true
}

// Check if `n` is a palindrome number.
fn is_palindrome(n: u32) -> bool {
    let s = n.to_string();
    s.chars().eq(s.chars().rev())
}

#[cfg(test)]
mod tests {
    use crate::birthday::make_message;

    #[test]
    fn make_message_normal() {
        assert_eq!(
            make_message(12345_i64),
            "You are 12345 days old.");
    }

    #[test]
    fn make_message_normal_nice() {
        assert_eq!(
            make_message(10000_i64),
            "You are 10000 days old. That's a nice, round number!");
    }

    #[test]
    fn make_message_newborn() {
        assert_eq!(
            make_message(0),
            "Looks like you're new here.");
    }

    #[test]
    fn make_message_future() {
        assert_eq!(
            make_message(-1),
            "Are you from the future?");
    }

    #[test]
    fn make_message_prime() {
        assert_eq!(
            make_message(10_007),
            "You are 10007 days old. That's a prime number!");
    }

    #[test]
    fn make_message_palindrome_prime() {
        assert_eq!(
            make_message(11_311),
            "You are 11311 days old. That's a prime number! That's a palindrome!");
    }

    #[test]
    fn make_message_palindrome() {
        assert_eq!(
            make_message(22_522),
            "You are 22522 days old. That's a palindrome!");
    }
}
