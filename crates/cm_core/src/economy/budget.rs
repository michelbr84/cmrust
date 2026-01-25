//! Budget management for clubs.

use super::Money;
use serde::{Deserialize, Serialize};

/// Club budget allocation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Budget {
    /// Total balance.
    pub balance: Money,
    /// Transfer budget remaining.
    pub transfer_budget: Money,
    /// Wage budget (weekly).
    pub wage_budget: Money,
    /// Current weekly wage bill.
    pub wage_bill: Money,
}

impl Budget {
    /// Create a new budget.
    pub fn new(balance: Money, transfer_budget: Money, wage_budget: Money) -> Self {
        Self {
            balance,
            transfer_budget,
            wage_budget,
            wage_bill: Money::ZERO,
        }
    }

    /// Available transfer funds.
    pub fn available_for_transfers(&self) -> Money {
        self.transfer_budget
    }

    /// Available wage room (weekly).
    pub fn available_wage_room(&self) -> Money {
        self.wage_budget - self.wage_bill
    }

    /// Check if can afford a transfer.
    pub fn can_afford_transfer(&self, amount: Money) -> bool {
        amount <= self.transfer_budget
    }

    /// Check if can afford a wage.
    pub fn can_afford_wage(&self, weekly_wage: Money) -> bool {
        self.wage_bill + weekly_wage <= self.wage_budget
    }

    /// Spend on transfer.
    pub fn spend_transfer(&mut self, amount: Money) {
        self.transfer_budget -= amount;
        self.balance -= amount;
    }

    /// Receive transfer income.
    pub fn receive_transfer(&mut self, amount: Money) {
        self.transfer_budget += amount;
        self.balance += amount;
    }

    /// Add to wage bill.
    pub fn add_wage(&mut self, weekly_wage: Money) {
        self.wage_bill += weekly_wage;
    }

    /// Remove from wage bill.
    pub fn remove_wage(&mut self, weekly_wage: Money) {
        self.wage_bill -= weekly_wage;
    }

    /// Process weekly wage payment.
    pub fn pay_weekly_wages(&mut self) {
        self.balance -= self.wage_bill;
    }
}

impl Default for Budget {
    fn default() -> Self {
        Self::new(Money::ZERO, Money::ZERO, Money::ZERO)
    }
}
