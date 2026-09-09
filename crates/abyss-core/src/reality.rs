//! ⛧-Doom-Slayer-⛧
//! §B5: Reality Kernel: Formal State Model & Finite State Machine.

use std::collections::VecDeque;

use crate::abyss::{AbyssState, MutationEvent, MutationLog};
use crate::random::{RandomDomain, RandomStream};
use crate::seed::Seed;

/// §B5.2: Configurable weights for the stability formula.
/// Allows balancing without recompiling the kernel.
#[derive(Debug, Clone, Copy)]
pub struct StabilityModel {
    pub weight_entropy: f64,
    pub weight_corruption: f64,
    pub weight_order: f64,
}

impl Default for StabilityModel {
    fn default() -> Self {
        Self {
            // The sum wE + wC must be > 1.0 for stability to reach 0.0.
            // even at the maximum order (order = 1.0).
            weight_entropy: 0.6,
            weight_corruption: 0.5,
            weight_order: 0.1,
        }
    }
}

/// Central state of the simulated Abyss universe.
#[derive(Debug, Clone)]
pub struct RealityKernel {
    seed: Seed,
    depth: u64,
    entropy: f64,
    corruption: f64,
    tick: u64,
    state: AbyssState,
    log: MutationLog,
    recent_events: VecDeque<MutationEvent>, // For Shannon entropy calculation (§B5.2)
    stability_model: StabilityModel,
}

impl RealityKernel {
    pub fn new(seed: Seed) -> Self {
        Self {
            seed,
            depth: 0,
            entropy: 0.0,
            corruption: 0.0,
            tick: 0,
            state: AbyssState::Surface,
            log: MutationLog::new(),
            recent_events: VecDeque::with_capacity(64),
            stability_model: StabilityModel::default(),
        }
    }

    // --- Getters ---
    pub const fn seed(&self) -> Seed {
        self.seed
    }
    pub const fn depth(&self) -> u64 {
        self.depth
    }
    pub const fn entropy(&self) -> f64 {
        self.entropy
    }
    pub const fn corruption(&self) -> f64 {
        self.corruption
    }
    pub const fn tick(&self) -> u64 {
        self.tick
    }
    pub const fn state(&self) -> AbyssState {
        self.state
    }
    pub fn log(&self) -> &MutationLog {
        &self.log
    }

    /// §B5.2: Calculates normalized Shannon entropy of recent events.
    /// Returns a value in [0.0, 1.0], where 1.0 is maximum chaos (all events unique),
    /// and 0.0 is maximum order (all events are the same).
    fn calculate_order(&self) -> f64 {
        if self.recent_events.is_empty() {
            return 1.0; // Default to maximum order if no events
        }

        let total = self.recent_events.len() as f64;
        let mut counts = std::collections::HashMap::new();
        for &event in &self.recent_events {
            *counts.entry(event).or_insert(0) += 1;
        }

        let mut shannon_entropy = 0.0;
        let num_possible_events: f64 = 3.0; // Descend, Corrupt, Fracture

        for &count in counts.values() {
            let p = (count as f64) / total;
            if p > 0.0 {
                shannon_entropy -= p * p.log2();
            }
        }

        // Normalize by max possible entropy (log2(num_possible_events))
        let max_entropy = num_possible_events.log2();
        let normalized = if max_entropy > 0.0 {
            shannon_entropy / max_entropy
        } else {
            0.0
        };

        // Order is the inverse of chaos: 1.0 - normalized_entropy
        1.0 - normalized
    }

    /// §B5.2: Explicit stability formula.
    pub fn stability(&self) -> f64 {
        let order = self.calculate_order();
        let stability = 1.0
            - (self.stability_model.weight_entropy * self.entropy
                + self.stability_model.weight_corruption * self.corruption
                - self.stability_model.weight_order * order);
        stability.clamp(0.0, 1.0)
    }

    pub fn random_stream(&self, domain: RandomDomain) -> RandomStream {
        RandomStream::new(self.seed, domain)
    }

    // --- Mutations with FSM Guards (§B5.4) ---

    fn apply_mutation(&mut self, event: MutationEvent, mutate: impl FnOnce(&mut Self)) {
        let previous_state = self.state;
        let previous_entropy = self.entropy;
        let previous_corruption = self.corruption;

        mutate(self);

        // Update recent events window (max 64)
        self.recent_events.push_back(event);
        if self.recent_events.len() > 64 {
            self.recent_events.pop_front();
        }

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

    pub fn descend(&mut self) {
        self.apply_mutation(MutationEvent::Descend, |kernel| {
            kernel.depth = kernel.depth.saturating_add(1);
            kernel.entropy = (kernel.entropy + 0.05).min(1.0);
        });
    }

    pub fn corrupt(&mut self, delta: f64) {
        self.apply_mutation(MutationEvent::Corrupt, |kernel| {
            kernel.corruption = (kernel.corruption + delta).clamp(0.0, 1.0);
        });
    }

    pub fn fracture(&mut self) {
        self.apply_mutation(MutationEvent::Fracture, |kernel| {
            kernel.entropy = (kernel.entropy + 0.2).min(1.0);
            kernel.corruption = (kernel.corruption + 0.2).min(1.0);
        });
    }

    /// §B5.4: Explicit transition table with numeric guards.
    fn evaluate_state(&self) -> AbyssState {
        let stability = self.stability();

        // Guards are evaluated in order of severity
        if stability <= 0.03 {
            AbyssState::Void
        } else if stability <= 0.10 {
            AbyssState::Collapse
        } else if stability <= 0.25 {
            AbyssState::Inferno
        } else if self.corruption >= 0.35 && self.state != AbyssState::Surface {
            // Corruption guard requires us to be at least in Descent
            AbyssState::Corruption
        } else if self.depth > 0 && self.corruption < 0.35 {
            AbyssState::Descent
        } else {
            AbyssState::Surface
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn kernel_starts_at_surface() {
        let kernel = RealityKernel::new(Seed::new(1));
        assert_eq!(kernel.depth(), 0);
        assert_eq!(kernel.tick(), 0);
        assert_eq!(kernel.state(), AbyssState::Surface);
    }

    #[test]
    fn descend_advances_depth_and_state() {
        let mut kernel = RealityKernel::new(Seed::new(1));
        kernel.descend();
        assert_eq!(kernel.depth(), 1);
        assert_eq!(kernel.tick(), 1);
        assert_eq!(kernel.state(), AbyssState::Descent);
    }

    #[test]
    fn corruption_threshold_triggers_corruption_state() {
        let mut kernel = RealityKernel::new(Seed::new(1));
        kernel.descend(); // Must be in Descent first per guard
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
        assert!(kernel.stability() <= 0.03);
        assert_eq!(kernel.state(), AbyssState::Void);
    }

    // §B13: Property test for FSM invariants.
    // The kernel should never reach an undefined state, and stability must always be in [0.0, 1.0].
    // Note: Changed /// to // to avoid rustdoc warning on macro invocations.
    proptest! {
        #[test]
        fn stability_is_always_clamped(seed_val: u64, fractures in 0..10usize) {
            let mut kernel = RealityKernel::new(Seed::new(seed_val));
            for _ in 0..fractures {
                kernel.fracture();
            }
            let s = kernel.stability();
            prop_assert!(s >= 0.0 && s <= 1.0, "Stability {} is out of bounds", s);
        }
    }
}
