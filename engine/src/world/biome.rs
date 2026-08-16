//! ⛧-Doom-Slayer-⛧
//! Biome definitions tied to Abyss reality states.

use crate::core::abyss::AbyssState;

/// Biome type determines the mathematical regime of world generation.
///
/// Each biome modifies fractal parameters, terrain density,
/// corruption intensity, and visual behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Biome {
    /// Stable surface geometry. Low entropy, predictable SDFs.
    Surface,
    /// Descending fractal layers. Increasing complexity.
    Descent,
    /// Corruption spreading through geometry. Unstable SDFs.
    Corruption,
    /// Reality mutating. Fractal parameters shift with position.
    Mutation,
    /// Infernal geometry. Aggressive fractals, high detail.
    Inferno,
    /// Collapsing topology. Negative space dominates.
    Collapse,
    /// The Void. Minimal geometry, pure mathematical emptiness.
    Void,
}

impl Biome {
    /// Maps an AbyssState to the corresponding biome.
    ///
    /// This creates a direct link between the reality kernel's
    /// mathematical state and the world's visual/structural behavior.
    pub const fn from_state(state: AbyssState) -> Self {
        match state {
            AbyssState::Surface => Self::Surface,
            AbyssState::Descent => Self::Descent,
            AbyssState::Corruption => Self::Corruption,
            AbyssState::Mutation => Self::Mutation,
            AbyssState::Inferno => Self::Inferno,
            AbyssState::Collapse => Self::Collapse,
            AbyssState::Void => Self::Void,
        }
    }

    /// Returns the fractal iteration count for this biome.
    ///
    /// Higher iterations = more complex geometry = more expensive evaluation.
    pub const fn fractal_iterations(&self) -> u32 {
        match self {
            Self::Surface => 4,
            Self::Descent => 6,
            Self::Corruption => 8,
            Self::Mutation => 10,
            Self::Inferno => 12,
            Self::Collapse => 8,
            Self::Void => 2,
        }
    }

    /// Returns the fractal power parameter.
    ///
    /// This controls the "aggressiveness" of the Mandelbulb formula.
    pub const fn fractal_power(&self) -> f64 {
        match self {
            Self::Surface => 4.0,
            Self::Descent => 6.0,
            Self::Corruption => 8.0,
            Self::Mutation => 10.0,
            Self::Inferno => 12.0,
            Self::Collapse => 8.0,
            Self::Void => 2.0,
        }
    }

    /// Returns the corruption intensity multiplier.
    ///
    /// This affects how much the local geometry is warped by corruption.
    pub const fn corruption_intensity(&self) -> f64 {
        match self {
            Self::Surface => 0.0,
            Self::Descent => 0.1,
            Self::Corruption => 0.5,
            Self::Mutation => 0.7,
            Self::Inferno => 1.0,
            Self::Collapse => 0.9,
            Self::Void => 0.0,
        }
    }

    /// Returns the terrain density threshold.
    ///
    /// Lower values = more solid terrain. Higher = more caves/void.
    pub const fn density_threshold(&self) -> f64 {
        match self {
            Self::Surface => 0.0,
            Self::Descent => 0.1,
            Self::Corruption => 0.3,
            Self::Mutation => 0.4,
            Self::Inferno => 0.5,
            Self::Collapse => 0.7,
            Self::Void => 0.9,
        }
    }

    /// Returns the stable textual identifier of the biome.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Surface => "SURFACE",
            Self::Descent => "DESCENT",
            Self::Corruption => "CORRUPTION",
            Self::Mutation => "MUTATION",
            Self::Inferno => "INFERNO",
            Self::Collapse => "COLLAPSE",
            Self::Void => "VOID",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn biome_maps_from_state() {
        assert_eq!(Biome::from_state(AbyssState::Surface), Biome::Surface);
        assert_eq!(Biome::from_state(AbyssState::Inferno), Biome::Inferno);
        assert_eq!(Biome::from_state(AbyssState::Void), Biome::Void);
    }

    #[test]
    fn biome_parameters_are_ordered() {
        assert!(Biome::Surface.fractal_iterations() < Biome::Inferno.fractal_iterations());
        assert!(Biome::Surface.corruption_intensity() < Biome::Inferno.corruption_intensity());
        assert!(Biome::Surface.density_threshold() < Biome::Void.density_threshold());
    }

    #[test]
    fn biome_identifiers_are_stable() {
        assert_eq!(Biome::Inferno.as_str(), "INFERNO");
        assert_eq!(Biome::Void.as_str(), "VOID");
    }
}
