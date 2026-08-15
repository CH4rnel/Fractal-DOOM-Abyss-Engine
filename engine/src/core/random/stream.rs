// ⛧-Doom-Slayer-⛧

use super::domain::RandomDomain;
use crate::core::seed::Seed;

/// Deterministic pseudo-random stream.
///
/// A stream is derived from the universe seed and a specific
/// subsystem domain. Each domain therefore owns an independent
/// deterministic sequence.
///
/// This generator is intended for procedural generation and
/// gameplay systems. It is NOT suitable for cryptography,
/// authentication, or security-sensitive operations.
#[derive(Debug, Clone)]
pub struct RandomStream {
    state: u64,
}

impl RandomStream {
    /// Creates a deterministic stream for the specified universe domain.
    pub fn new(seed: Seed, domain: RandomDomain) -> Self {
        Self {
            state: mix_seed(seed, domain),
        }
    }

    /// Advances the stream and returns the next 64-bit value.
    ///
    /// Uses the SplitMix64 output transformation, providing a fast,
    /// deterministic stream with good statistical properties for
    /// procedural generation.
    pub fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9E37_79B9_7F4A_7C15);

        let mut value = self.state;

        value = (value ^ (value >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        value = (value ^ (value >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);

        value ^ (value >> 31)
    }
}

/// Combines a universe seed with a domain identifier.
///
/// The domain is incorporated into the initial state so that each
/// subsystem receives an independent deterministic sequence.
///
/// For example, consuming random values from the Demon stream does
/// not advance or otherwise modify the Mining stream.
fn mix_seed(seed: Seed, domain: RandomDomain) -> u64 {
    let mut state = seed_value(seed);

    for byte in domain.as_str().bytes() {
        state ^= u64::from(byte);
        state = state.wrapping_mul(0x1000_0000_01B3).rotate_left(13);
    }

    avalanche(state)
}

/// Extracts the raw deterministic value from a Seed.
///
/// This conversion remains private to the random subsystem so that
/// other systems do not become coupled to Seed's internal
/// representation.
const fn seed_value(seed: Seed) -> u64 {
    seed.raw()
}

/// Applies a final avalanche transformation to improve bit diffusion.
///
/// Small changes in the input should affect many bits of the output.
/// This is useful when deriving independent deterministic stream
/// starting states.
fn avalanche(mut value: u64) -> u64 {
    value ^= value >> 30;
    value = value.wrapping_mul(0xBF58_476D_1CE4_E5B9);

    value ^= value >> 27;
    value = value.wrapping_mul(0x94D0_49BB_1331_11EB);

    value ^ (value >> 31)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_seed_and_domain_produce_same_stream() {
        let seed = Seed::new(666);

        let mut first = RandomStream::new(seed, RandomDomain::Demons);
        let mut second = RandomStream::new(seed, RandomDomain::Demons);

        for _ in 0..32 {
            assert_eq!(first.next_u64(), second.next_u64());
        }
    }

    #[test]
    fn different_domains_produce_different_streams() {
        let seed = Seed::new(666);

        let mut demons = RandomStream::new(seed, RandomDomain::Demons);
        let mut mining = RandomStream::new(seed, RandomDomain::Mining);

        assert_ne!(demons.next_u64(), mining.next_u64());
    }

    #[test]
    fn stream_advances() {
        let seed = Seed::new(666);
        let mut stream = RandomStream::new(seed, RandomDomain::Geometry);

        let first = stream.next_u64();
        let second = stream.next_u64();

        assert_ne!(first, second);
    }

    #[test]
    fn cloned_streams_produce_identical_sequences() {
        let seed = Seed::new(666);

        let mut original = RandomStream::new(seed, RandomDomain::Loot);
        let mut clone = original.clone();

        for _ in 0..32 {
            assert_eq!(original.next_u64(), clone.next_u64());
        }
    }

    #[test]
    fn different_seeds_produce_different_streams() {
        let mut first = RandomStream::new(Seed::new(666), RandomDomain::Demons);
        let mut second = RandomStream::new(Seed::new(667), RandomDomain::Demons);

        assert_ne!(first.next_u64(), second.next_u64());
    }
}
