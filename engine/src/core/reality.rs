// ⛧-Doom-Slayer-⛧

use crate::core::seed::Seed;

/// Authoritative deterministic state of the Abyss.
///
/// The Reality Kernel contains only fundamental world state.
/// Rendering, audio, networking and gameplay systems must derive
/// their behaviour from this state rather than owning competing
/// versions of reality.
#[derive(Debug, Clone)]
pub struct RealityKernel {
    seed: Seed,
    depth: f64,
    entropy: f64,
    corruption: f64,
}

impl RealityKernel {
    /// Creates a new universe from a deterministic seed.
    pub fn new(seed: Seed) -> Self {
        Self {
            seed,
            depth: 0.0,
            entropy: 0.0,
            corruption: 0.0,
        }
    }

    /// Returns the universe seed.
    pub fn seed(&self) -> Seed {
        self.seed
    }

    /// Returns the current world depth.
    pub fn depth(&self) -> f64 {
        self.depth
    }

    /// Returns the current entropy level.
    pub fn entropy(&self) -> f64 {
        self.entropy
    }

    /// Returns the current corruption level.
    pub fn corruption(&self) -> f64 {
        self.corruption
    }

    /// Returns the current dimensional stability.
    ///
    /// Stability is derived from entropy and corruption rather than
    /// stored separately, preventing duplicated state.
    pub fn stability(&self) -> f64 {
        1.0 - (self.entropy * 0.5 + self.corruption * 0.5)
    }

    /// Changes the current depth.
    ///
    /// Depth is clamped to non-negative values because the initial
    /// world coordinate cannot exist below the surface.
    pub fn set_depth(&mut self, depth: f64) {
        self.depth = depth.max(0.0);
    }

    /// Sets the entropy of the current reality.
    ///
    /// Entropy is normalized to the range 0.0..=1.0.
    pub fn set_entropy(&mut self, entropy: f64) {
        self.entropy = entropy.clamp(0.0, 1.0);
    }

    /// Sets the corruption of the current reality.
    ///
    /// Corruption is normalized to the range 0.0..=1.0.
    pub fn set_corruption(&mut self, corruption: f64) {
        self.corruption = corruption.clamp(0.0, 1.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kernel_preserves_seed() {
        let seed = Seed::new(666);
        let kernel = RealityKernel::new(seed);

        assert_eq!(kernel.seed(), seed);
    }

    #[test]
    fn kernel_starts_at_surface() {
        let kernel = RealityKernel::new(Seed::new(1));

        assert_eq!(kernel.depth(), 0.0);
    }

    #[test]
    fn entropy_is_clamped() {
        let mut kernel = RealityKernel::new(Seed::new(1));

        kernel.set_entropy(42.0);

        assert_eq!(kernel.entropy(), 1.0);
    }

    #[test]
    fn corruption_cannot_be_negative() {
        let mut kernel = RealityKernel::new(Seed::new(1));

        kernel.set_corruption(-666.0);

        assert_eq!(kernel.corruption(), 0.0);
    }

    #[test]
    fn stability_is_derived_from_entropy_and_corruption() {
        let mut kernel = RealityKernel::new(Seed::new(1));

        kernel.set_entropy(0.4);
        kernel.set_corruption(0.6);

        assert!((kernel.stability() - 0.5).abs() < f64::EPSILON);
    }
}
