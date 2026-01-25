//! Seeded RNG for deterministic simulation.

use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use serde::{Deserialize, Serialize};

/// Simulation RNG wrapper for reproducible results.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimRng {
    seed: u64,
    #[serde(skip)]
    rng: Option<StdRng>,
}

impl SimRng {
    /// Create with a specific seed.
    pub fn new(seed: u64) -> Self {
        Self {
            seed,
            rng: Some(StdRng::seed_from_u64(seed)),
        }
    }

    /// Create from entropy.
    pub fn from_entropy() -> Self {
        let mut rng = StdRng::from_entropy();
        let seed = rng.gen();
        Self {
            seed,
            rng: Some(rng),
        }
    }

    /// Get the seed.
    pub fn seed(&self) -> u64 {
        self.seed
    }

    /// Ensure RNG is initialized (needed after deserialization).
    fn ensure_init(&mut self) {
        if self.rng.is_none() {
            self.rng = Some(StdRng::seed_from_u64(self.seed));
        }
    }

    /// Generate a random float in [0, 1).
    pub fn random(&mut self) -> f32 {
        self.ensure_init();
        self.rng.as_mut().unwrap().gen()
    }

    /// Generate a random integer in range [min, max).
    pub fn range(&mut self, min: i32, max: i32) -> i32 {
        self.ensure_init();
        self.rng.as_mut().unwrap().gen_range(min..max)
    }

    /// Roll a percentage chance (0-100).
    pub fn roll(&mut self, percent: u8) -> bool {
        self.range(0, 100) < percent as i32
    }

    /// Pick a random index for a slice of given length.
    pub fn index(&mut self, len: usize) -> usize {
        if len == 0 {
            0
        } else {
            self.range(0, len as i32) as usize
        }
    }
}

impl Default for SimRng {
    fn default() -> Self {
        Self::from_entropy()
    }
}
