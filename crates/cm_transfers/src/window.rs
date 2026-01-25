//! Transfer window system (stub).

use chrono::{Datelike, NaiveDate};

/// Check if transfer window is open.
pub fn is_window_open(date: NaiveDate) -> bool {
    let month = date.month();
    // Summer: June-August, Winter: January
    matches!(month, 1 | 6 | 7 | 8)
}

/// Get next window opening.
pub fn next_window_date(from: NaiveDate) -> NaiveDate {
    // Simplified: returns next January 1 or June 1
    let year = from.year();
    if from.month() < 6 {
        NaiveDate::from_ymd_opt(year, 6, 1).unwrap()
    } else if from.month() < 9 {
        from
    } else {
        NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap()
    }
}
