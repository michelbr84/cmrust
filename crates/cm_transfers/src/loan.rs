//! Loan system (stub).

use chrono::NaiveDate;
use cm_core::economy::Money;
use cm_core::ids::{ClubId, PlayerId};
use serde::{Deserialize, Serialize};

/// Loan agreement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Loan {
    pub player_id: PlayerId,
    pub parent_club: ClubId,
    pub loan_club: ClubId,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub wage_contribution: u8, // percentage
    pub loan_fee: Money,
    pub option_to_buy: Option<Money>,
}
