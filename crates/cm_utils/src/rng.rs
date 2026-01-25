//! Random number generation utilities.

use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

/// Create a seeded RNG for deterministic simulation.
pub fn seeded_rng(seed: u64) -> ChaCha8Rng {
    ChaCha8Rng::seed_from_u64(seed)
}

/// Create an entropy-based RNG.
pub fn entropy_rng() -> ChaCha8Rng {
    ChaCha8Rng::from_entropy()
}

/// Generate a random value in range [min, max).
pub fn random_range<R: Rng>(rng: &mut R, min: i32, max: i32) -> i32 {
    rng.gen_range(min..max)
}

/// Generate a random float in range [0.0, 1.0).
pub fn random_float<R: Rng>(rng: &mut R) -> f32 {
    rng.gen()
}

/// Roll a percentage chance (0-100).
pub fn roll_chance<R: Rng>(rng: &mut R, percent: u8) -> bool {
    rng.gen_range(0..100) < percent
}

/// Pick a random element from a slice.
pub fn pick_random<'a, T, R: Rng>(rng: &mut R, items: &'a [T]) -> Option<&'a T> {
    if items.is_empty() {
        None
    } else {
        Some(&items[rng.gen_range(0..items.len())])
    }
}
