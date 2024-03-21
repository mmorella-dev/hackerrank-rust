// https://www.hackerrank.com/challenges/day-of-the-programmer/problem?isFullScreen=true

use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/// a naive 0-indexed date format
struct Date {
    year: i32,
    month: usize,
    day: usize,
}

/// returns the formatted date of the 256th day of the given year
/// using Julian dates until 31 Jan, 1918  after which the date becomes
/// 14 Feb, 1918 in the Gregorian calendar.
fn day_of_programmer(year: i32) -> String {
    let date = get_nth_day(year, 255);
    format_date(date)
}

fn get_nth_day(year: i32, n: usize) -> Date {
    let mut day = n;
    for (month, &days) in days_in_months(year).iter().enumerate() {
        if day < days {
            // hack: correctly specify dates in February 1918.
            if year == 1918 && month == 1 {
                day += 13;
            }
            return Date { year, month, day };
        }
        day -= days;
    }
    panic!("range error")
}

fn is_leap_year(year: i32) -> bool {
    match year {
        1700..=1917 => {
            divides(4, year) // julian
        }
        1918..=2800 => {
            divides(400, year) || divides(4, year) && !divides(100, year) // gregorian
        }
        _ => panic!("range error"),
    }
}

fn days_in_february(year: i32) -> usize {
    let mut days = 28;
    if year == 1918 {
        days -= 13 // transition year. february is 13 days shorter.
    }
    if is_leap_year(year) {
        days += 1;
    }
    days
}

fn days_in_months(year: i32) -> [usize; 12] {
    [
        31, // jan
        days_in_february(year),
        31, // mar
        30, // apr
        31, // may
        30, // jun
        31, // july
        31, // aug
        30, // sep
        31, // oct
        30, // nov
        31, // dec
    ]
}

fn format_date(d: Date) -> String {
    format!("{:02}.{:02}.{:04}", d.day + 1, d.month + 1, d.year)
}

/// returns whether a|b; i.e. whether b is divisible by a.
fn divides(a: i32, b: i32) -> bool {
    b % a == 0
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let year = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    let result = day_of_programmer(year);

    writeln!(&mut fptr, "{}", result).ok();
}

#[cfg(test)]
mod test {
    use crate::{day_of_programmer, format_date, get_nth_day};

    #[test]
    fn sample_input_0() {
        assert_eq!(day_of_programmer(2017), "13.09.2017".to_string());
    }
    #[test]
    fn sample_input_1() {
        assert_eq!(day_of_programmer(2016), "12.09.2016".to_string());
    }
    #[test]
    fn sample_input_2() {
        assert_eq!(day_of_programmer(1800), "12.09.1800".to_string());
    }

    #[test]
    fn test_calendar_swap() {
        assert_eq!(format_date(get_nth_day(1918, 30)), "31.01.1918".to_string());
        assert_eq!(format_date(get_nth_day(1918, 31)), "14.02.1918".to_string());
    }
}
