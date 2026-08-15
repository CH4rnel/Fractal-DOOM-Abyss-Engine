// ⛧-Doom-Slayer-⛧

/// Deterministic subsystem domains of the Abyss.
///
/// Every procedural subsystem receives its own domain.
/// This prevents unrelated systems from influencing each
/// other's deterministic output.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RandomDomain {
    Geometry,
    Demons,
    Mining,
    Loot,
    Audio,
    Lore,
    Events,
}

impl RandomDomain {
    /// Returns the stable textual identifier of the domain.
    ///
    /// These identifiers are part of the deterministic contract.
    /// Changing them changes generated universes.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Geometry => "geometry",
            Self::Demons => "demons",
            Self::Mining => "mining",
            Self::Loot => "loot",
            Self::Audio => "audio",
            Self::Lore => "lore",
            Self::Events => "events",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn domain_names_are_stable() {
        assert_eq!(RandomDomain::Geometry.as_str(), "geometry");
        assert_eq!(RandomDomain::Demons.as_str(), "demons");
        assert_eq!(RandomDomain::Mining.as_str(), "mining");
    }
}
