// ⛧-Doom-Slayer-⛧

use std::fmt;

/// Deterministic identity of an Abyss universe.
///
/// A seed is intentionally small and cheap to copy.
/// More sophisticated deterministic random streams will be layered
/// on top of it as the engine evolves.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Seed(u64);

impl Seed {
    /// Creates a seed from a raw 64-bit value.
    pub const fn new(value: u64) -> Self {
        Self(value)
    }
}

impl Default for Seed {
    fn default() -> Self {
        Self::new(0x7FA9_DA31_0666_AB55)
    }
}

impl fmt::Display for Seed {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "0x{:016X}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seed_is_deterministic() {
        let first = Seed::new(42);
        let second = Seed::new(42);

        assert_eq!(first, second);
    }

    #[test]
    fn seed_display_is_hexadecimal() {
        let seed = Seed::new(0xDEAD_BEEF);

        assert_eq!(seed.to_string(), "0x00000000DEADBEEF");
    }
}
