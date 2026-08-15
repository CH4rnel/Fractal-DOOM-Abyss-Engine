//! ⛧-Doom-Slayer-⛧
//! Deterministic subsystem domains of the Abyss.

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

    /// Returns an iterator over all domains to generate the Universe Manifest.
    pub fn iter_all() -> impl Iterator<Item = RandomDomain> {
        [
            Self::Geometry,
            Self::Demons,
            Self::Mining,
            Self::Loot,
            Self::Audio,
            Self::Lore,
            Self::Events,
        ]
        .into_iter()
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
