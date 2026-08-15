// ⛧-Doom-Slayer-⛧

use super::random::{RandomDomain, RandomStream};
use super::seed::Seed;

/// Central state of the simulated Abyss universe.
///
/// `RealityKernel` owns the immutable universe identity (`Seed`) and
/// the mutable parameters describing how deeply reality has been
/// corrupted.
///
/// Random streams are derived from the same seed but isolated by
/// `RandomDomain`, which keeps procedural generation deterministic.
#[derive(Debug, Clone)]
pub struct RealityKernel {
    seed: Seed,
    depth: u64,
    entropy: f64,
    corruption: f64,
}

impl RealityKernel {
    /// Creates a new universe at the surface state.
    pub fn new(seed: Seed) -> Self {
        Self {
            seed,
            depth: 0,
            entropy: 0.0,
            corruption: 0.0,
        }
    }

    /// Returns the universe seed.
    pub const fn seed(&self) -> Seed {
        self.seed
    }

    /// Returns the current fractal depth.
    pub const fn depth(&self) -> u64 {
        self.depth
    }

    /// Returns the current entropy level.
    pub const fn entropy(&self) -> f64 {
        self.entropy
    }

    /// Returns the current corruption level.
    pub const fn corruption(&self) -> f64 {
        self.corruption
    }

    /// Returns the current reality stability.
    ///
    /// Stability is reduced by both entropy and corruption and is
    /// clamped to the valid range `[0.0, 1.0]`.
    pub fn stability(&self) -> f64 {
        (1.0 - ((self.entropy + self.corruption) / 2.0)).clamp(0.0, 1.0)
    }

    /// Changes the current fractal depth.
    pub fn set_depth(&mut self, depth: u64) {
        self.depth = depth;
    }

    /// Changes entropy while keeping it inside `[0.0, 1.0]`.
    pub fn set_entropy(&mut self, entropy: f64) {
        self.entropy = entropy.clamp(0.0, 1.0);
    }

    /// Changes corruption while keeping it inside `[0.0, 1.0]`.
    pub fn set_corruption(&mut self, corruption: f64) {
        self.corruption = corruption.clamp(0.0, 1.0);
    }

    /// Creates an isolated deterministic random stream.
    ///
    /// Every subsystem receives a stream derived from the universe
    /// seed and its own domain. Consuming random values from one
    /// domain therefore does not affect another domain.
    pub fn random_stream(&self, domain: RandomDomain) -> RandomStream {
        RandomStream::new(self.seed, domain)
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

        assert_eq!(kernel.depth(), 0);
    }

    #[test]
    fn entropy_is_clamped() {
        let mut kernel = RealityKernel::new(Seed::new(1));

        kernel.set_entropy(2.0);
        assert_eq!(kernel.entropy(), 1.0);

        kernel.set_entropy(-1.0);
        assert_eq!(kernel.entropy(), 0.0);
    }

    #[test]
    fn corruption_cannot_be_negative() {
        let mut kernel = RealityKernel::new(Seed::new(1));

        kernel.set_corruption(-10.0);

        assert_eq!(kernel.corruption(), 0.0);
    }

    #[test]
    fn stability_is_derived_from_entropy_and_corruption() {
        let mut kernel = RealityKernel::new(Seed::new(1));

        kernel.set_entropy(0.4);
        kernel.set_corruption(0.6);

        assert!((kernel.stability() - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn random_stream_is_deterministic() {
        let kernel = RealityKernel::new(Seed::new(666));

        let mut first = kernel.random_stream(RandomDomain::Demons);
        let mut second = kernel.random_stream(RandomDomain::Demons);

        assert_eq!(first.next_u64(), second.next_u64());
    }

    #[test]
    fn random_domains_are_isolated() {
        let kernel = RealityKernel::new(Seed::new(666));

        let mut demons = kernel.random_stream(RandomDomain::Demons);
        let mut mining = kernel.random_stream(RandomDomain::Mining);

        assert_ne!(demons.next_u64(), mining.next_u64());
    }
}
