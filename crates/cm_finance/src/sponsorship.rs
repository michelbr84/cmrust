//! Sponsorship (stub).

use cm_core::economy::Money;

/// Calculate annual sponsorship based on reputation.
pub fn calculate_sponsorship(reputation: u8) -> Money {
    let base = 1_000_000i64;
    let multiplier = reputation as i64 / 10;
    Money::from_major(base * multiplier)
}
