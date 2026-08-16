//! ⛧-Doom-Slayer-⛧
//! Procedural demon genealogy and lineage classification.

use crate::core::random::stream::RandomStream;
use crate::world::chunk::ChunkCoord;

use super::identity::DemonSeed;

/// Genealogical classification of a demon.
#[derive(Debug, Clone, PartialEq)]
pub struct Genealogy {
    /// Fractal recursion depth (1 to 9).
    pub recursion_depth: u32,
    /// Lineage classification based on spawn depth.
    pub lineage: Lineage,
}

/// Lineage classification.
///
/// Deeper demons belong to darker lineages.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Lineage {
    SpawnOfTheAbyss,
    ChildOfFracture,
    EchoOfTheVoid,
    RemnantOfCollapse,
    HeraldOfInferno,
}

/// Threat level classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ThreatLevel {
    Negligible,
    Minor,
    Moderate,
    Severe,
    Extreme,
    Apocalyptic,
}

impl Genealogy {
    /// Derives genealogy from a random stream and spawn context.
    ///
    /// Lineage is determined by spawn depth (y-coordinate):
    /// deeper demons belong to darker lineages.
    pub fn derive(stream: &mut RandomStream, seed: &DemonSeed) -> Self {
        let recursion_depth = 1 + (stream.next_u64() % 9) as u32;

        let lineage = Self::lineage_from_depth(seed.coord());

        Self {
            recursion_depth,
            lineage,
        }
    }

    /// Maps spawn depth to lineage classification.
    fn lineage_from_depth(coord: ChunkCoord) -> Lineage {
        match coord.y {
            y if y < 2 => Lineage::SpawnOfTheAbyss,
            y if y < 5 => Lineage::ChildOfFracture,
            y if y < 10 => Lineage::EchoOfTheVoid,
            y if y < 20 => Lineage::RemnantOfCollapse,
            _ => Lineage::HeraldOfInferno,
        }
    }

    /// Returns a stable textual identifier for the lineage.
    pub fn lineage_str(&self) -> &'static str {
        match self.lineage {
            Lineage::SpawnOfTheAbyss => "SPAWN_OF_THE_ABYSS",
            Lineage::ChildOfFracture => "CHILD_OF_FRACTURE",
            Lineage::EchoOfTheVoid => "ECHO_OF_THE_VOID",
            Lineage::RemnantOfCollapse => "REMNANT_OF_COLLAPSE",
            Lineage::HeraldOfInferno => "HERALD_OF_INFERNO",
        }
    }
}

impl ThreatLevel {
    /// Returns a stable textual identifier for the threat level.
    pub fn as_str(&self) -> &'static str {
        match self {
            ThreatLevel::Negligible => "NEGLIGIBLE",
            ThreatLevel::Minor => "MINOR",
            ThreatLevel::Moderate => "MODERATE",
            ThreatLevel::Severe => "SEVERE",
            ThreatLevel::Extreme => "EXTREME",
            ThreatLevel::Apocalyptic => "APOCALYPTIC",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::random::domain::RandomDomain;
    use crate::core::seed::Seed;

    #[test]
    fn genealogy_is_deterministic() {
        let seed = DemonSeed::new(Seed::new(666), ChunkCoord::new(0, 3, 0), 0);

        let mut stream_a = RandomStream::new(Seed::new(666), RandomDomain::Demons);
        let mut stream_b = RandomStream::new(Seed::new(666), RandomDomain::Demons);

        let genealogy_a = Genealogy::derive(&mut stream_a, &seed);
        let genealogy_b = Genealogy::derive(&mut stream_b, &seed);

        assert_eq!(genealogy_a, genealogy_b);
    }

    #[test]
    fn lineage_maps_from_depth() {
        assert_eq!(
            Genealogy::lineage_from_depth(ChunkCoord::new(0, 0, 0)),
            Lineage::SpawnOfTheAbyss
        );
        assert_eq!(
            Genealogy::lineage_from_depth(ChunkCoord::new(0, 3, 0)),
            Lineage::ChildOfFracture
        );
        assert_eq!(
            Genealogy::lineage_from_depth(ChunkCoord::new(0, 7, 0)),
            Lineage::EchoOfTheVoid
        );
        assert_eq!(
            Genealogy::lineage_from_depth(ChunkCoord::new(0, 15, 0)),
            Lineage::RemnantOfCollapse
        );
        assert_eq!(
            Genealogy::lineage_from_depth(ChunkCoord::new(0, 50, 0)),
            Lineage::HeraldOfInferno
        );
    }

    #[test]
    fn recursion_depth_in_bounds() {
        let seed = DemonSeed::new(Seed::new(666), ChunkCoord::new(0, 0, 0), 0);
        let mut stream = RandomStream::new(Seed::new(666), RandomDomain::Demons);
        let genealogy = Genealogy::derive(&mut stream, &seed);

        assert!(genealogy.recursion_depth >= 1 && genealogy.recursion_depth <= 9);
    }

    #[test]
    fn threat_level_ordering() {
        assert!(ThreatLevel::Negligible < ThreatLevel::Minor);
        assert!(ThreatLevel::Minor < ThreatLevel::Moderate);
        assert!(ThreatLevel::Moderate < ThreatLevel::Severe);
        assert!(ThreatLevel::Severe < ThreatLevel::Extreme);
        assert!(ThreatLevel::Extreme < ThreatLevel::Apocalyptic);
    }

    #[test]
    fn threat_level_str_works() {
        assert_eq!(ThreatLevel::Apocalyptic.as_str(), "APOCALYPTIC");
    }

    #[test]
    fn lineage_str_works() {
        let seed = DemonSeed::new(Seed::new(666), ChunkCoord::new(0, 0, 0), 0);
        let mut stream = RandomStream::new(Seed::new(666), RandomDomain::Demons);
        let genealogy = Genealogy::derive(&mut stream, &seed);

        assert!(!genealogy.lineage_str().is_empty());
    }
}