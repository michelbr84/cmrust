//! Prize money (stub).

use cm_core::economy::Money;

/// Get league position prize money.
pub fn league_prize(position: u8, league_tier: u8) -> Money {
    let base = match league_tier {
        1 => 10_000_000i64,
        2 => 1_000_000,
        _ => 100_000,
    };
    let position_mult = (21 - position.min(20)) as i64;
    Money::from_major(base * position_mult / 10)
}
