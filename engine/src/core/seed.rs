// ⛧-Doom-Slayer-⛧

use std::fmt;

/// Deterministic identity of an Abyss universe.
///
/// A seed is intentionally small and cheap to copy.
/// Higher-level systems derive independent deterministic streams
/// from this value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Seed(u64);

impl Seed {
    /// Creates a seed from a raw 64-bit value.
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the raw deterministic value of this seed.
    ///
    /// Higher-level systems should normally use domain-specific
    /// random streams instead of manipulating the raw value.
    pub const fn raw(self) -> u64 {
        self.0
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

    #[test]
    fn default_seed_is_stable() {
        assert_eq!(Seed::default().raw(), 0x7FA9_DA31_0666_AB55);
    }
}
