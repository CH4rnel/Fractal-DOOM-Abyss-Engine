//! ⛧-Doom-Slayer-⛧
//! Deterministic chunk system for the infinite Abyss.

use crate::core::seed::Seed;

/// Size of a single chunk along each axis (in world units).
pub const CHUNK_SIZE: f64 = 16.0;

/// Integer coordinates identifying a chunk in the infinite grid.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChunkCoord {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl ChunkCoord {
    pub const fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    /// Returns the world-space origin (minimum corner) of this chunk.
    pub fn origin(&self) -> (f64, f64, f64) {
        (
            self.x as f64 * CHUNK_SIZE,
            self.y as f64 * CHUNK_SIZE,
            self.z as f64 * CHUNK_SIZE,
        )
    }

    /// Derives a deterministic chunk seed from the universe seed and coordinates.
    ///
    /// This ensures the same universe + same coordinates always produce
    /// the same chunk, regardless of generation order.
    pub fn derive_seed(&self, universe: Seed) -> Seed {
        let mut hash: u64 = universe.raw();

        // FNV-1a style mixing with chunk coordinates
        hash ^= (self.x as u64).wrapping_mul(0x51_7C_C1_B7_27_22_0A_95);
        hash = hash.wrapping_mul(0x1000_0000_01B3);
        hash ^= (self.y as u64).wrapping_mul(0x6C_62_27_2E_07_BB_01_42);
        hash = hash.wrapping_mul(0x1000_0000_01B3);
        hash ^= (self.z as u64).wrapping_mul(0x9E_37_79_B9_7F_4A_7C_15);
        hash = hash.wrapping_mul(0x1000_0000_01B3);

        // Final avalanche
        hash ^= hash >> 30;
        hash = hash.wrapping_mul(0xBF58_476D_1CE4_E5B9);
        hash ^= hash >> 27;
        hash = hash.wrapping_mul(0x94D0_49BB_1331_11EB);
        hash ^= hash >> 31;

        Seed::new(hash)
    }
}

/// Lifecycle state of a chunk in the streaming system.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChunkState {
    Unloaded,
    Loading,
    Active,
    Corrupted,
    Unloading,
}

/// A single chunk of the infinite Abyss world.
#[derive(Debug, Clone)]
pub struct Chunk {
    pub coord: ChunkCoord,
    pub seed: Seed,
    pub state: ChunkState,
}

impl Chunk {
    /// Creates a new chunk with deterministic seed derivation.
    pub fn new(coord: ChunkCoord, universe: Seed) -> Self {
        let seed = coord.derive_seed(universe);
        Self {
            coord,
            seed,
            state: ChunkState::Unloaded,
        }
    }

    /// Transitions the chunk to the Active state.
    pub fn activate(&mut self) {
        self.state = ChunkState::Active;
    }

    /// Marks the chunk as corrupted (e.g., by player actions).
    pub fn corrupt(&mut self) {
        self.state = ChunkState::Corrupted;
    }

    /// Returns true if the chunk is currently active or corrupted.
    pub fn is_present(&self) -> bool {
        matches!(self.state, ChunkState::Active | ChunkState::Corrupted)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chunk_origin_is_deterministic() {
        let coord = ChunkCoord::new(1, 2, 3);
        let (x, y, z) = coord.origin();
        assert_eq!(x, CHUNK_SIZE);
        assert_eq!(y, CHUNK_SIZE * 2.0);
        assert_eq!(z, CHUNK_SIZE * 3.0);
    }

    #[test]
    fn same_universe_same_coord_same_seed() {
        let universe = Seed::new(666);
        let coord = ChunkCoord::new(1, 2, 3);

        let seed_a = coord.derive_seed(universe);
        let seed_b = coord.derive_seed(universe);

        assert_eq!(seed_a, seed_b);
    }

    #[test]
    fn different_coords_different_seeds() {
        let universe = Seed::new(666);

        let seed_a = ChunkCoord::new(1, 2, 3).derive_seed(universe);
        let seed_b = ChunkCoord::new(3, 2, 1).derive_seed(universe);

        assert_ne!(seed_a, seed_b);
    }

    #[test]
    fn chunk_lifecycle_works() {
        let universe = Seed::new(666);
        let coord = ChunkCoord::new(0, 0, 0);
        let mut chunk = Chunk::new(coord, universe);

        assert_eq!(chunk.state, ChunkState::Unloaded);
        assert!(!chunk.is_present());

        chunk.activate();
        assert_eq!(chunk.state, ChunkState::Active);
        assert!(chunk.is_present());

        chunk.corrupt();
        assert_eq!(chunk.state, ChunkState::Corrupted);
        assert!(chunk.is_present());
    }
}
