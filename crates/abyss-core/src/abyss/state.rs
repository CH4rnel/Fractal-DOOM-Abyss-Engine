//! ⛧-Doom-Slayer-⛧
//! Mathematical reality states and mutation events of the Abyss.

/// Mathematical/reality conditions of the universe.
///
/// These are not conventional levels. They describe the structural
/// integrity and corruption of the fractal reality.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AbyssState {
    Surface,
    Descent,
    Corruption,
    Mutation,
    Inferno,
    Collapse,
    Void,
}

impl AbyssState {
    /// Returns the stable textual identifier of the state.
    ///
    /// These identifiers are part of the deterministic contract.
    /// Renaming them changes generated universe histories.
    pub const fn as_str(self) -> &'static str {
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

/// Actions that force the universe to mutate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MutationEvent {
    Descend,
    Corrupt,
    Fracture,
}

impl MutationEvent {
    /// Returns the stable textual identifier of the event.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Descend => "DESCEND",
            Self::Corrupt => "CORRUPT",
            Self::Fracture => "FRACTURE",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_identifiers_are_stable() {
        assert_eq!(AbyssState::Surface.as_str(), "SURFACE");
        assert_eq!(AbyssState::Inferno.as_str(), "INFERNO");
        assert_eq!(AbyssState::Void.as_str(), "VOID");
    }

    #[test]
    fn event_identifiers_are_stable() {
        assert_eq!(MutationEvent::Descend.as_str(), "DESCEND");
        assert_eq!(MutationEvent::Fracture.as_str(), "FRACTURE");
    }
}
