//! Wage calculations (stub).

use cm_core::economy::Money;

/// Calculate total weekly wages.
pub fn calculate_weekly_wages(player_wages: &[Money]) -> Money {
    player_wages.iter().fold(Money::ZERO, |acc, w| acc + *w)
}
