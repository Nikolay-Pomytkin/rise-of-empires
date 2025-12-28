//! Deterministic random number generator
//!
//! Uses Xoshiro128++ for fast, deterministic random numbers.
//! The RNG is seeded once at simulation start and produces
//! identical sequences given the same seed.

use bevy_ecs::prelude::*;
use rand::Rng;
use rand_core::SeedableRng;

#[cfg(feature = "deterministic_rng")]
use rand_xoshiro::Xoshiro128PlusPlus;

/// Deterministic RNG resource for the simulation
#[derive(Resource)]
pub struct SimRng {
    #[cfg(feature = "deterministic_rng")]
    rng: Xoshiro128PlusPlus,

    #[cfg(not(feature = "deterministic_rng"))]
    rng: rand::rngs::StdRng,

    seed: u64,
}

impl SimRng {
    pub fn new(seed: u64) -> Self {
        Self {
            #[cfg(feature = "deterministic_rng")]
            rng: Xoshiro128PlusPlus::seed_from_u64(seed),

            #[cfg(not(feature = "deterministic_rng"))]
            rng: rand::rngs::StdRng::seed_from_u64(seed),

            seed,
        }
    }

    /// Reset the RNG to its initial state
    pub fn reset(&mut self) {
        #[cfg(feature = "deterministic_rng")]
        {
            self.rng = Xoshiro128PlusPlus::seed_from_u64(self.seed);
        }

        #[cfg(not(feature = "deterministic_rng"))]
        {
            self.rng = rand::rngs::StdRng::seed_from_u64(self.seed);
        }
    }

    /// Get the seed used to initialize this RNG
    pub fn seed(&self) -> u64 {
        self.seed
    }

    /// Get the current seed (for saving game state)
    pub fn current_seed(&self) -> u64 {
        self.seed
    }

    /// Set a new seed and reinitialize the RNG
    pub fn set_seed(&mut self, seed: u64) {
        self.seed = seed;
        self.reset();
    }

    /// Generate a random f32 in [0, 1)
    pub fn random(&mut self) -> f32 {
        self.rng.random()
    }

    /// Generate a random f32 in [min, max)
    pub fn range_f32(&mut self, min: f32, max: f32) -> f32 {
        self.rng.random_range(min..max)
    }

    /// Generate a random i32 in [min, max)
    pub fn range_i32(&mut self, min: i32, max: i32) -> i32 {
        self.rng.random_range(min..max)
    }

    /// Generate a random u32 in [min, max)
    pub fn range_u32(&mut self, min: u32, max: u32) -> u32 {
        self.rng.random_range(min..max)
    }

    /// Generate a random u64
    pub fn next_u64(&mut self) -> u64 {
        self.rng.random()
    }

    /// Generate a random bool with given probability
    pub fn chance(&mut self, probability: f32) -> bool {
        self.random() < probability
    }
}

impl Default for SimRng {
    fn default() -> Self {
        Self::new(12345)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_determinism() {
        let mut rng1 = SimRng::new(42);
        let mut rng2 = SimRng::new(42);

        for _ in 0..1000 {
            assert_eq!(rng1.next_u64(), rng2.next_u64());
        }
    }

    #[test]
    fn test_different_seeds() {
        let mut rng1 = SimRng::new(42);
        let mut rng2 = SimRng::new(43);

        // Very unlikely to be equal with different seeds
        let seq1: Vec<u64> = (0..10).map(|_| rng1.next_u64()).collect();
        let seq2: Vec<u64> = (0..10).map(|_| rng2.next_u64()).collect();

        assert_ne!(seq1, seq2);
    }

    #[test]
    fn test_reset() {
        let mut rng = SimRng::new(42);

        let first_run: Vec<u64> = (0..10).map(|_| rng.next_u64()).collect();

        rng.reset();

        let second_run: Vec<u64> = (0..10).map(|_| rng.next_u64()).collect();

        assert_eq!(first_run, second_run);
    }
}
