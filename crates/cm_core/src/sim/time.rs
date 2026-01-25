//! Game date and time management.

use chrono::{Datelike, NaiveDate, Weekday};
use serde::{Deserialize, Serialize};

/// Game date wrapper with football-specific utilities.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct GameDate(pub NaiveDate);

impl GameDate {
    /// Create a new game date.
    pub fn new(year: i32, month: u32, day: u32) -> Self {
        Self(NaiveDate::from_ymd_opt(year, month, day).unwrap_or_else(|| {
            NaiveDate::from_ymd_opt(2001, 7, 1).unwrap()
        }))
    }

    /// Get the inner NaiveDate.
    pub fn date(&self) -> NaiveDate {
        self.0
    }

    /// Advance by one day.
    pub fn advance_day(&mut self) {
        self.0 = self.0.succ_opt().unwrap_or(self.0);
    }

    /// Advance by n days.
    pub fn advance_days(&mut self, n: u32) {
        for _ in 0..n {
            self.advance_day();
        }
    }

    /// Get the current season year.
    /// Season runs July-June, so dates before July belong to previous season.
    pub fn season_year(&self) -> i32 {
        if self.0.month() >= 7 {
            self.0.year()
        } else {
            self.0.year() - 1
        }
    }

    /// Get the season string (e.g., "2001-02").
    pub fn season_string(&self) -> String {
        let start = self.season_year();
        let end = (start + 1) % 100;
        format!("{}-{:02}", start, end)
    }

    /// Check if it's a weekend.
    pub fn is_weekend(&self) -> bool {
        matches!(self.0.weekday(), Weekday::Sat | Weekday::Sun)
    }

    /// Check if it's a Saturday (typical match day).
    pub fn is_saturday(&self) -> bool {
        self.0.weekday() == Weekday::Sat
    }

    /// Check if it's the first day of the month.
    pub fn is_first_of_month(&self) -> bool {
        self.0.day() == 1
    }

    /// Get the day of week.
    pub fn weekday(&self) -> Weekday {
        self.0.weekday()
    }

    /// Get year.
    pub fn year(&self) -> i32 {
        self.0.year()
    }

    /// Get month.
    pub fn month(&self) -> u32 {
        self.0.month()
    }

    /// Get day.
    pub fn day(&self) -> u32 {
        self.0.day()
    }

    /// Parse from string.
    pub fn parse(s: &str) -> Option<Self> {
        NaiveDate::parse_from_str(s, "%Y-%m-%d")
            .ok()
            .map(Self)
    }
}

impl Default for GameDate {
    fn default() -> Self {
        Self::new(2001, 7, 1)
    }
}

impl std::fmt::Display for GameDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.format("%d %b %Y"))
    }
}

impl From<NaiveDate> for GameDate {
    fn from(date: NaiveDate) -> Self {
        Self(date)
    }
}
