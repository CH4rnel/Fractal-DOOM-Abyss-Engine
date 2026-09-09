//! ⛧-Doom-Slayer-⛧
//! The Reality Kernel: central state container for the simulated universe.

use super::abyss::{AbyssState, MutationEvent, MutationLog};
use super::random::{RandomDomain, RandomStream};
use super::seed::Seed;

/// Central state of the simulated Abyss universe.
///
/// `RealityKernel` owns the immutable universe identity (`Seed`) and the
/// mutable parameters describing how deeply reality has been corrupted.
/// State transitions are controlled and every mutation is recorded in the
/// authoritative `MutationLog`.
#[derive(Debug, Clone)]
pub struct RealityKernel {
    seed: Seed,
    depth: u64,
    entropy: f64,
    corruption: f64,
    tick: u64,
    state: AbyssState,
    log: MutationLog,
}

impl RealityKernel {
    /// Creates a new universe at the surface state.
    pub fn new(seed: Seed) -> Self {
        Self {
            seed,
            depth: 0,
            entropy: 0.0,
            corruption: 0.0,
            tick: 0,
            state: AbyssState::Surface,
            log: MutationLog::new(),
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

    /// Returns the current universe tick.
    pub const fn tick(&self) -> u64 {
        self.tick
    }

    /// Returns the current reality state.
    pub const fn state(&self) -> AbyssState {
        self.state
    }

    /// Returns the authoritative mutation log.
    pub fn log(&self) -> &MutationLog {
        &self.log
    }

    /// Returns the current reality stability.
    ///
    /// Stability is reduced by both entropy and corruption and is clamped
    /// to the valid range `[0.0, 1.0]`.
    pub fn stability(&self) -> f64 {
        (1.0 - ((self.entropy + self.corruption) / 2.0)).clamp(0.0, 1.0)
    }

    /// Creates an isolated deterministic random stream.
    pub fn random_stream(&self, domain: RandomDomain) -> RandomStream {
        RandomStream::new(self.seed, domain)
    }

    /// Forces reality to descend into a deeper fractal layer.
    pub fn descend(&mut self) {
        self.apply_mutation(MutationEvent::Descend, |kernel| {
            kernel.depth = kernel.depth.saturating_add(1);
            kernel.entropy = (kernel.entropy + 0.05).min(1.0);
        });
    }

    /// Injects corruption into the reality kernel.
    pub fn corrupt(&mut self, delta: f64) {
        self.apply_mutation(MutationEvent::Corrupt, |kernel| {
            kernel.corruption = (kernel.corruption + delta).clamp(0.0, 1.0);
        });
    }

    /// Triggers a mathematical fracture, spiking both entropy and corruption.
    pub fn fracture(&mut self) {
        self.apply_mutation(MutationEvent::Fracture, |kernel| {
            kernel.entropy = (kernel.entropy + 0.2).min(1.0);
            kernel.corruption = (kernel.corruption + 0.2).min(1.0);
        });
    }

    /// Applies a mutation, re-evaluates the mathematical state, records the
    /// transition, and advances the universe tick.
    fn apply_mutation<F>(&mut self, event: MutationEvent, mutate: F)
    where
        F: FnOnce(&mut Self),
    {
        let previous_state = self.state;
        let previous_entropy = self.entropy;
        let previous_corruption = self.corruption;

        mutate(self);

        let new_state = self.evaluate_state();
        let entropy_delta = self.entropy - previous_entropy;
        let corruption_delta = self.corruption - previous_corruption;

        self.log.record(
            self.tick,
            event,
            previous_state,
            new_state,
            entropy_delta,
            corruption_delta,
        );

        self.state = new_state;
        self.tick = self.tick.saturating_add(1);
    }

    /// Derives the reality state strictly from mathematical thresholds.
    fn evaluate_state(&self) -> AbyssState {
        let stability = self.stability();

        if stability <= 0.0 {
            AbyssState::Void
        } else if stability <= 0.1 {
            AbyssState::Collapse
        } else if self.corruption >= 0.8 && self.entropy >= 0.8 {
            AbyssState::Inferno
        } else if self.entropy >= 0.6 {
            AbyssState::Mutation
        } else if self.corruption >= 0.3 {
            AbyssState::Corruption
        } else if self.depth > 0 {
            AbyssState::Descent
        } else {
            AbyssState::Surface
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kernel_starts_at_surface() {
        let kernel = RealityKernel::new(Seed::new(1));
        assert_eq!(kernel.depth(), 0);
        assert_eq!(kernel.tick(), 0);
        assert_eq!(kernel.state(), AbyssState::Surface);
    }

    #[test]
    fn descend_advances_depth_tick_and_state() {
        let mut kernel = RealityKernel::new(Seed::new(1));
        kernel.descend();
        assert_eq!(kernel.depth(), 1);
        assert_eq!(kernel.tick(), 1);
        assert_eq!(kernel.state(), AbyssState::Descent);
    }

    #[test]
    fn corruption_threshold_triggers_corruption_state() {
        let mut kernel = RealityKernel::new(Seed::new(1));
        kernel.descend();
        kernel.corrupt(0.35);
        assert_eq!(kernel.state(), AbyssState::Corruption);
    }

    #[test]
    fn repeated_fracture_collapses_reality_to_void() {
        let mut kernel = RealityKernel::new(Seed::new(666));
        for _ in 0..5 {
            kernel.fracture();
        }
        assert_eq!(kernel.entropy(), 1.0);
        assert_eq!(kernel.corruption(), 1.0);
        assert_eq!(kernel.stability(), 0.0);
        assert_eq!(kernel.state(), AbyssState::Void);
    }

    #[test]
    fn mutation_log_records_every_transition() {
        let mut kernel = RealityKernel::new(Seed::new(1));
        kernel.descend();
        kernel.corrupt(0.4);
        assert_eq!(kernel.log().len(), 2);
        assert!(!kernel.log().is_empty());
    }

    #[test]
    fn corruption_is_clamped() {
        let mut kernel = RealityKernel::new(Seed::new(1));
        kernel.corrupt(5.0);
        assert_eq!(kernel.corruption(), 1.0);
        kernel.corrupt(-5.0);
        assert_eq!(kernel.corruption(), 0.0);
    }
}
