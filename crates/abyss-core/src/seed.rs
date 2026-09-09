//! ⛧-Doom-Slayer-⛧
//! Universe seed: the immutable identity of the simulated reality.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Seed(pub u64);

impl Seed {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn value(&self) -> u64 {
        self.0
    }
}

impl Default for Seed {
    fn default() -> Self {
        Self::new(0)
    }
}
