//! Debt management (stub).

use cm_core::economy::Money;

/// Calculate interest payment.
pub fn calculate_interest(debt: Money, rate: f32) -> Money {
    Money::from_minor((debt.minor() as f64 * rate as f64) as i64)
}
