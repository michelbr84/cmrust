//! Tactics AI (stub).

use cm_core::world::{Formation, Mentality, Tactics};

/// Select formation based on squad.
pub fn recommend_formation(_attack_strength: u8, _defense_strength: u8) -> Formation {
    Formation::F442
}

/// Adjust mentality for situation.
pub fn adjust_mentality(current_score_diff: i8, minutes_remaining: u8) -> Mentality {
    if current_score_diff < 0 && minutes_remaining < 30 {
        Mentality::Attacking
    } else if current_score_diff > 1 {
        Mentality::Defensive
    } else {
        Mentality::Balanced
    }
}
