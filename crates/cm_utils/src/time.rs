//! Time utilities.

use chrono::{Datelike, NaiveDate, Weekday};

/// Get the current season year from a date.
/// Season runs from July to June, so dates before July belong to previous season.
pub fn season_year(date: NaiveDate) -> i32 {
    if date.month() >= 7 {
        date.year()
    } else {
        date.year() - 1
    }
}

/// Check if a date is a weekend.
pub fn is_weekend(date: NaiveDate) -> bool {
    matches!(date.weekday(), Weekday::Sat | Weekday::Sun)
}

/// Get days until next occurrence of a weekday.
pub fn days_until_weekday(from: NaiveDate, target: Weekday) -> i64 {
    let current = from.weekday().num_days_from_monday() as i64;
    let target = target.num_days_from_monday() as i64;
    let diff = target - current;
    if diff <= 0 {
        diff + 7
    } else {
        diff
    }
}

/// Format date as display string.
pub fn format_date(date: NaiveDate) -> String {
    date.format("%d %b %Y").to_string()
}

/// Parse date from string (YYYY-MM-DD).
pub fn parse_date(s: &str) -> Option<NaiveDate> {
    NaiveDate::parse_from_str(s, "%Y-%m-%d").ok()
}
