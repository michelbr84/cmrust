//! Injury tracking.

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Types of injuries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InjuryType {
    Hamstring,
    Groin,
    Knee,
    Ankle,
    Calf,
    Thigh,
    Back,
    Shoulder,
    Concussion,
    Illness,
    Other,
}

impl InjuryType {
    /// Get display name.
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Hamstring => "Hamstring",
            Self::Groin => "Groin",
            Self::Knee => "Knee",
            Self::Ankle => "Ankle",
            Self::Calf => "Calf",
            Self::Thigh => "Thigh",
            Self::Back => "Back",
            Self::Shoulder => "Shoulder",
            Self::Concussion => "Concussion",
            Self::Illness => "Illness",
            Self::Other => "Other",
        }
    }
}

/// An injury.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Injury {
    pub injury_type: InjuryType,
    pub start_date: NaiveDate,
    pub expected_return: NaiveDate,
    pub severity: u8, // 1-10
}

impl Injury {
    /// Create a new injury.
    pub fn new(injury_type: InjuryType, start_date: NaiveDate, days_out: u16) -> Self {
        let expected_return = start_date + chrono::Duration::days(days_out as i64);
        let severity = match days_out {
            0..=7 => 2,
            8..=14 => 4,
            15..=30 => 6,
            31..=60 => 8,
            _ => 10,
        };
        Self {
            injury_type,
            start_date,
            expected_return,
            severity,
        }
    }

    /// Check if injury is healed.
    pub fn is_healed(&self, current_date: NaiveDate) -> bool {
        current_date >= self.expected_return
    }

    /// Get days remaining.
    pub fn days_remaining(&self, current_date: NaiveDate) -> i64 {
        (self.expected_return - current_date).num_days().max(0)
    }
}
