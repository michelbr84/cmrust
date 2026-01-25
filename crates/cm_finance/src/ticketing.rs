//! Ticketing revenue (stub).

use cm_core::economy::Money;

/// Calculate matchday revenue.
pub fn calculate_matchday_revenue(attendance: u32, ticket_price: Money) -> Money {
    Money::from_minor(attendance as i64 * ticket_price.minor())
}
