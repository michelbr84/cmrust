//! Contract clauses (stub).

use cm_core::economy::Money;
use serde::{Deserialize, Serialize};

/// Clause types.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Clause {
    ReleaseClause(Money),
    SellOnPercent(u8),
    MatchFee(Money),
    BuyBackClause { amount: Money, expires_years: u8 },
}
