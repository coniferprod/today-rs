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

fn ordinal_suffix(n: usize) -> String {
    match n % 10 {
        1 => "st".to_string(),
        2 => "nd".to_string(),
        3 => "rd".to_string(),
        _ => "th".to_string(),
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
        let fib_ordinal = test_fib(day_count as u32);
        if fib_ordinal > 0 {
            message.push_str(
                &format!(" That's the {}{} Fibonacci number!", 
                    fib_ordinal, ordinal_suffix(fib_ordinal)));
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

// Check if `n` is a Fibonacci number under 50,000.
fn test_fib(n: u32) -> usize {
    let numbers = fib_under_50k();
    //println!("{:#?}", numbers);
    for (ordinal, number) in numbers.iter().enumerate() {
        if n == *number {
            return ordinal;
        }
    }
    0
}

fn fib_under_50k() -> Vec<u32> {
    let mut result = vec![0, 1];

    let mut a = result[1];
    let mut b = result[0];
    let mut count = 2;

    while count < 25 {
        let temp = a + b;
        b = a;
        a = temp;
        result.push(temp);
        count += 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::birthday::{fib_under_50k, make_message};

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

    #[test]
    fn make_message_fib_prime() {
        assert_eq!(
            make_message(28_657),
            "You are 28657 days old. That's a prime number! That's the 23rd Fibonacci number!");
    }

    #[test]
    fn make_message_fib() {
        assert_eq!(
            make_message(17_711),
            "You are 17711 days old. That's the 22nd Fibonacci number!");
    }

    #[test]
    fn make_message_fib_palindrome() {
        assert_eq!(
            make_message(55),
            "You are 55 days old. That's a palindrome! That's the 10th Fibonacci number!");
    }

    #[test]
    fn make_fib() {
        assert_eq!(
            fib_under_50k(),
            vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610,
                987, 1597, 2584, 4181, 6765, 10946, 17711, 28657, 46368]
        );
    }
}
