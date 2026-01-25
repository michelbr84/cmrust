//! Tactics influence on match.

use crate::model::TeamStrength;
use cm_core::world::{Formation, Mentality, Tactics};

/// Apply tactics modifiers to team strength.
pub fn apply_tactics_modifiers(base: &TeamStrength, tactics: &Tactics) -> TeamStrength {
    let mut modified = base.clone();

    // Mentality affects attack/defense balance
    match tactics.mentality {
        Mentality::Defensive => {
            modified.attack = modified.attack.saturating_sub(10);
            modified.defense = modified.defense.saturating_add(10).min(100);
        }
        Mentality::Cautious => {
            modified.attack = modified.attack.saturating_sub(5);
            modified.defense = modified.defense.saturating_add(5).min(100);
        }
        Mentality::Balanced => {}
        Mentality::Attacking => {
            modified.attack = modified.attack.saturating_add(5).min(100);
            modified.defense = modified.defense.saturating_sub(5);
        }
        Mentality::AllOutAttack => {
            modified.attack = modified.attack.saturating_add(10).min(100);
            modified.defense = modified.defense.saturating_sub(10);
        }
    }

    // Pressing affects midfield
    if tactics.pressing > 70 {
        modified.midfield = modified.midfield.saturating_add(5).min(100);
    }

    modified
}

/// Get formation attacking bonus.
pub fn formation_attack_bonus(formation: Formation) -> i8 {
    match formation {
        Formation::F433 | Formation::F343 => 5,
        Formation::F4231 | Formation::F3412 => 3,
        Formation::F442 => 0,
        Formation::F451 | Formation::F4141 => -3,
        Formation::F532 | Formation::F352 => -5,
        _ => 0,
    }
}

/// Get formation defensive bonus.
pub fn formation_defense_bonus(formation: Formation) -> i8 {
    match formation {
        Formation::F532 => 5,
        Formation::F352 | Formation::F451 => 3,
        Formation::F442 => 0,
        Formation::F433 | Formation::F4231 => -3,
        Formation::F343 => -5,
        _ => 0,
    }
}
