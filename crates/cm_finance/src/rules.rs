//! Financial rules (stub).

/// Financial rules configuration.
pub struct FinancialRules {
    pub tax_rate: f32,
    pub agent_fee_cap: f32,
    pub minimum_wage: i64,
}

impl Default for FinancialRules {
    fn default() -> Self {
        Self {
            tax_rate: 0.20,
            agent_fee_cap: 0.10,
            minimum_wage: 500,
        }
    }
}
