//! ⛧-Doom-Slayer-⛧
//! §B4: Deterministic RNG Stack v2 - Domain-isolated stream generator.

use super::domain::RandomDomain;
use crate::seed::Seed;
use rand::Rng;
use rand::RngCore;
use rand::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;

const FNV_OFFSET_BASIS: u64 = 0xcbf29ce484222325;
const FNV_PRIME: u64 = 0x100000001b3;

/// FNV-1a 64-bit hash for domain name separation.
fn fnv1a_64(data: &[u8]) -> u64 {
    let mut hash = FNV_OFFSET_BASIS;
    for &byte in data {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    hash
}

/// SplitMix64 finalizer/avalanche step.
fn splitmix64_next(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9e3779b97f4a7c15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
    z ^ (z >> 31)
}

/// §B4.1: Domain separation seed expansion.
/// Uses domain name hash instead of sequential index to prevent correlation.
fn domain_seed(universe_seed: u64, domain: RandomDomain, salt: u64) -> u64 {
    let mut h = universe_seed;
    h ^= fnv1a_64(domain.as_str().as_bytes());
    h ^= salt.rotate_left(1); // rotate-mixed salt
    splitmix64_next(&mut h)
}

/// §B4.2: Stream generator using xoshiro256++.
/// Provides high-quality, long-period random sequences per domain.
#[derive(Debug, Clone)]
pub struct RandomStream {
    rng: Xoshiro256PlusPlus,
}

impl RandomStream {
    /// Creates a new stream with a default salt of 0.
    pub fn new(seed: Seed, domain: RandomDomain) -> Self {
        Self::with_salt(seed, domain, 0)
    }

    /// Creates a new stream with a specific salt for sub-domain separation.
    pub fn with_salt(seed: Seed, domain: RandomDomain, salt: u64) -> Self {
        let base = domain_seed(seed.value(), domain, salt);
        let mut state = base;
        let mut seed_bytes = [0u8; 32];

        // Four chained SplitMix64 rounds seed the four 64-bit words of xoshiro's state
        for i in 0..4 {
            let val = splitmix64_next(&mut state);
            seed_bytes[i * 8..(i + 1) * 8].copy_from_slice(&val.to_le_bytes());
        }

        Self {
            rng: Xoshiro256PlusPlus::from_seed(seed_bytes),
        }
    }

    /// Returns the next pseudo-random u64.
    pub fn next_u64(&mut self) -> u64 {
        self.rng.next_u64()
    }

    /// Returns the next pseudo-random f64 in [0, 1).
    pub fn next_f64(&mut self) -> f64 {
        self.rng.gen()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        /// §B4.3: Determinism invariant.
        /// same Seed + same Domain + same Salt ⇒ same Stream
        #[test]
        fn determinism_invariant(seed: u64, salt: u64) {
            let mut s1 = RandomStream::with_salt(Seed::new(seed), RandomDomain::Demons, salt);
            let mut s2 = RandomStream::with_salt(Seed::new(seed), RandomDomain::Demons, salt);
            for _ in 0..100 {
                prop_assert_eq!(s1.next_u64(), s2.next_u64());
            }
        }

        /// §B4.3: Isolation invariant.
        /// same Seed + different Domain ⇒ statistically independent streams
        #[test]
        fn isolation_invariant(seed: u64, salt: u64) {
            let mut s_geom = RandomStream::with_salt(Seed::new(seed), RandomDomain::Geometry, salt);
            let mut s_demon = RandomStream::with_salt(Seed::new(seed), RandomDomain::Demons, salt);

            let mut geom_vals = Vec::new();
            let mut demon_vals = Vec::new();
            for _ in 0..50 {
                geom_vals.push(s_geom.next_u64());
                demon_vals.push(s_demon.next_u64());
            }
            prop_assert_ne!(geom_vals, demon_vals);
        }
    }
}
