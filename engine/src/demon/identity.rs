//! ⛧-Doom-Slayer-⛧
//! Deterministic demon identity: seed derivation + full reconstruction.

use crate::core::random::domain::RandomDomain;
use crate::core::random::stream::RandomStream;
use crate::core::seed::Seed;
use crate::world::chunk::ChunkCoord;

use super::anatomy::Anatomy;
use super::behavior::Behavior;
use super::genealogy::Genealogy;
use super::mutation::Mutation;

/// Compact mathematical identity of a demon.
///
/// A demon is reconstructible from:
/// `UniverseSeed + Coordinate + SpawnTick + DemonDomain`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DemonSeed {
    universe: Seed,
    coord: ChunkCoord,
    spawn_tick: u64,
}

impl DemonSeed {
    /// Creates a new demon seed from its constituent parts.
    pub fn new(universe: Seed, coord: ChunkCoord, spawn_tick: u64) -> Self {
        Self {
            universe,
            coord,
            spawn_tick,
        }
    }

    /// Derives the demon's unique identity seed via deterministic hashing.
    ///
    /// The same inputs always produce the same identity.
    pub fn derive_identity(&self) -> Seed {
        let mut hash: u64 = self.universe.raw();

        // Mix coordinate (FNV-1a style)
        hash ^= (self.coord.x as u64).wrapping_mul(0x51_7C_C1_B7_27_22_0A_95);
        hash = hash.wrapping_mul(0x1000_0000_01B3);
        hash ^= (self.coord.y as u64).wrapping_mul(0x6C_62_27_2E_07_BB_01_42);
        hash = hash.wrapping_mul(0x1000_0000_01B3);
        hash ^= (self.coord.z as u64).wrapping_mul(0x9E_37_79_B9_7F_4A_7C_15);
        hash = hash.wrapping_mul(0x1000_0000_01B3);

        // Mix spawn tick
        hash ^= self.spawn_tick.wrapping_mul(0x85_EB_CA_6B_C2_B7_27_22);
        hash = hash.wrapping_mul(0x1000_0000_01B3);

        // Mix domain identifier
        for byte in RandomDomain::Demons.as_str().bytes() {
            hash ^= u64::from(byte);
            hash = hash.wrapping_mul(0x1000_0000_01B3).rotate_left(13);
        }

        // Final avalanche
        hash ^= hash >> 30;
        hash = hash.wrapping_mul(0xBF58_476D_1CE4_E5B9);
        hash ^= hash >> 27;
        hash = hash.wrapping_mul(0x94D0_49BB_1331_11EB);
        hash ^= hash >> 31;

        Seed::new(hash)
    }

    /// Creates a deterministic random stream scoped to the Demons domain.
    pub fn random_stream(&self) -> RandomStream {
        RandomStream::new(self.derive_identity(), RandomDomain::Demons)
    }

    /// Returns the universe seed this demon belongs to.
    pub fn universe(&self) -> Seed {
        self.universe
    }

    /// Returns the spawn coordinate.
    pub fn coord(&self) -> ChunkCoord {
        self.coord
    }

    /// Returns the tick at which this demon spawned.
    pub fn spawn_tick(&self) -> u64 {
        self.spawn_tick
    }
}

/// Complete deterministic reconstruction of a demon.
///
/// This is the full identity derived from a compact DemonSeed.
/// The same DemonSeed always produces the same DemonIdentity.
#[derive(Debug, Clone, PartialEq)]
pub struct DemonIdentity {
    pub seed: DemonSeed,
    pub anatomy: Anatomy,
    pub behavior: Behavior,
    pub mutation: Mutation,
    pub genealogy: Genealogy,
}

impl DemonIdentity {
    /// Reconstructs a demon's complete identity from its seed.
    ///
    /// This is a pure function: no side effects, no external state.
    pub fn reconstruct(seed: DemonSeed) -> Self {
        let mut stream = seed.random_stream();

        let anatomy = Anatomy::derive(&mut stream);
        let behavior = Behavior::derive(&mut stream);
        let mutation = Mutation::derive(&mut stream);
        let genealogy = Genealogy::derive(&mut stream, &seed);

        Self {
            seed,
            anatomy,
            behavior,
            mutation,
            genealogy,
        }
    }

    /// Computes the overall threat level from combined parameters.
    ///
    /// This is an emergent property: not stored, but derived.
    pub fn overall_threat(&self) -> super::genealogy::ThreatLevel {
        use super::genealogy::ThreatLevel;

        let score = self.genealogy.recursion_depth as f64
            + self.behavior.aggression * 5.0
            + self.mutation.mutation_level as f64
            + self.anatomy.body_scale;

        match score {
            s if s < 5.0 => ThreatLevel::Negligible,
            s if s < 10.0 => ThreatLevel::Minor,
            s if s < 15.0 => ThreatLevel::Moderate,
            s if s < 20.0 => ThreatLevel::Severe,
            s if s < 25.0 => ThreatLevel::Extreme,
            _ => ThreatLevel::Apocalyptic,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demon_seed_is_deterministic() {
        let universe = Seed::new(666);
        let coord = ChunkCoord::new(1, 2, 3);
        let tick = 42;

        let seed_a = DemonSeed::new(universe, coord, tick);
        let seed_b = DemonSeed::new(universe, coord, tick);

        assert_eq!(seed_a.derive_identity(), seed_b.derive_identity());
    }

    #[test]
    fn different_coords_different_demons() {
        let universe = Seed::new(666);
        let tick = 42;

        let seed_a = DemonSeed::new(universe, ChunkCoord::new(1, 2, 3), tick);
        let seed_b = DemonSeed::new(universe, ChunkCoord::new(3, 2, 1), tick);

        assert_ne!(seed_a.derive_identity(), seed_b.derive_identity());
    }

    #[test]
    fn different_ticks_different_demons() {
        let universe = Seed::new(666);
        let coord = ChunkCoord::new(1, 2, 3);

        let seed_a = DemonSeed::new(universe, coord, 42);
        let seed_b = DemonSeed::new(universe, coord, 43);

        assert_ne!(seed_a.derive_identity(), seed_b.derive_identity());
    }

    #[test]
    fn identity_reconstruction_is_deterministic() {
        let universe = Seed::new(666);
        let coord = ChunkCoord::new(5, 5, 5);
        let seed = DemonSeed::new(universe, coord, 100);

        let identity_a = DemonIdentity::reconstruct(seed);
        let identity_b = DemonIdentity::reconstruct(seed);

        assert_eq!(identity_a, identity_b);
    }

    #[test]
    fn overall_threat_is_valid() {
        let universe = Seed::new(666);
        let seed = DemonSeed::new(universe, ChunkCoord::new(0, 0, 0), 0);
        let identity = DemonIdentity::reconstruct(seed);

        // Just verify it doesn't panic and returns a valid threat level
        let _threat = identity.overall_threat();
    }
}