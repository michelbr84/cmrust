//! Contract entity.

use chrono::{Datelike, NaiveDate};
use serde::{Deserialize, Serialize};
use crate::economy::{Money, Wage};
use crate::ids::ContractId;

/// A player or staff contract.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contract {
    pub id: ContractId,
    pub wage: Wage,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub signing_fee: Money,
    pub release_clause: Option<Money>,
    pub loyalty_bonus: Money,
    pub appearance_bonus: Money,
    pub goal_bonus: Money,
}

impl Contract {
    /// Create a new contract.
    pub fn new(
        wage: Wage,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Self {
        Self {
            id: ContractId::new(uuid::Uuid::new_v4().to_string()),
            wage,
            start_date,
            end_date,
            signing_fee: Money::ZERO,
            release_clause: None,
            loyalty_bonus: Money::ZERO,
            appearance_bonus: Money::ZERO,
            goal_bonus: Money::ZERO,
        }
    }

    /// Check if contract is active on date.
    pub fn is_active(&self, date: NaiveDate) -> bool {
        date >= self.start_date && date <= self.end_date
    }

    /// Check if contract is expiring soon (within 6 months).
    pub fn is_expiring_soon(&self, date: NaiveDate) -> bool {
        let months_remaining = (self.end_date.year() - date.year()) * 12
            + (self.end_date.month() as i32 - date.month() as i32);
        months_remaining <= 6 && months_remaining > 0
    }

    /// Get years remaining.
    pub fn years_remaining(&self, date: NaiveDate) -> f32 {
        let days = (self.end_date - date).num_days();
        days as f32 / 365.0
    }
}
