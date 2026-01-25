//! Wage type for player/staff salaries.

use super::Money;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Wage frequency.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WageFrequency {
    Weekly,
    Monthly,
    Yearly,
}

/// Wage with frequency.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Wage {
    pub amount: Money,
    pub frequency: WageFrequency,
}

impl Wage {
    /// Create a weekly wage.
    pub fn weekly(amount: Money) -> Self {
        Self {
            amount,
            frequency: WageFrequency::Weekly,
        }
    }

    /// Create a monthly wage.
    pub fn monthly(amount: Money) -> Self {
        Self {
            amount,
            frequency: WageFrequency::Monthly,
        }
    }

    /// Create a yearly wage.
    pub fn yearly(amount: Money) -> Self {
        Self {
            amount,
            frequency: WageFrequency::Yearly,
        }
    }

    /// Convert to weekly wage.
    pub fn as_weekly(&self) -> Money {
        match self.frequency {
            WageFrequency::Weekly => self.amount,
            WageFrequency::Monthly => self.amount.multiply(12.0 / 52.0),
            WageFrequency::Yearly => self.amount.multiply(1.0 / 52.0),
        }
    }

    /// Convert to monthly wage.
    pub fn as_monthly(&self) -> Money {
        match self.frequency {
            WageFrequency::Weekly => self.amount.multiply(52.0 / 12.0),
            WageFrequency::Monthly => self.amount,
            WageFrequency::Yearly => self.amount.multiply(1.0 / 12.0),
        }
    }

    /// Convert to yearly wage.
    pub fn as_yearly(&self) -> Money {
        match self.frequency {
            WageFrequency::Weekly => self.amount.multiply(52.0),
            WageFrequency::Monthly => self.amount.multiply(12.0),
            WageFrequency::Yearly => self.amount,
        }
    }
}

impl fmt::Display for Wage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let suffix = match self.frequency {
            WageFrequency::Weekly => "p/w",
            WageFrequency::Monthly => "p/m",
            WageFrequency::Yearly => "p/a",
        };
        write!(f, "{} {}", self.amount, suffix)
    }
}

impl Default for Wage {
    fn default() -> Self {
        Self::weekly(Money::ZERO)
    }
}
