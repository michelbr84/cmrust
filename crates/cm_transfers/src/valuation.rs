//! Player valuation.

use cm_core::economy::Money;
use cm_core::world::Player;

/// Calculate player market value.
pub fn calculate_value(player: &Player, date: chrono::NaiveDate) -> Money {
    let age = player.age_on(date);
    let ability = player.overall_rating();

    // Base value from ability
    let base = match ability {
        90..=100 => 50_000_000i64,
        80..=89 => 20_000_000,
        70..=79 => 5_000_000,
        60..=69 => 1_000_000,
        50..=59 => 500_000,
        _ => 100_000,
    };

    // Age modifier
    let age_mod = match age {
        16..=21 => 1.5,
        22..=27 => 1.2,
        28..=30 => 1.0,
        31..=33 => 0.6,
        _ => 0.3,
    };

    // Potential modifier
    let pot_mod = 1.0 + ((player.potential as f64 - ability as f64) / 100.0);

    Money::from_major((base as f64 * age_mod * pot_mod) as i64)
}
