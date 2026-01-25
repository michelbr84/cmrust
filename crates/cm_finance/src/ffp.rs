//! FFP rules (stub).

use cm_core::economy::Money;

/// Check FFP compliance.
pub fn check_ffp_compliance(income: Money, expenses: Money) -> bool {
    income >= expenses.multiply(0.9) // Allow 10% loss
}
