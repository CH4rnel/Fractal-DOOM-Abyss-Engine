//! ⛧-Doom-Slayer-⛧
//! Chunk streaming: load/unload chunks around the player.

use std::collections::HashMap;

use crate::core::reality::RealityKernel;
use crate::fractal::math::Vec3;

use super::chunk::{CHUNK_SIZE, Chunk, ChunkCoord};
use super::generator::WorldGenerator;

/// Manages the active chunks around a player position.
pub struct WorldStreamer {
    chunks: HashMap<ChunkCoord, Chunk>,
    load_radius: i32,
    universe_seed: crate::core::seed::Seed,
}

impl WorldStreamer {
    /// Creates a new world streamer with the given load radius.
    pub fn new(universe_seed: crate::core::seed::Seed, load_radius: i32) -> Self {
        Self {
            chunks: HashMap::new(),
            load_radius,
            universe_seed,
        }
    }

    /// Returns the number of currently loaded chunks.
    pub fn loaded_count(&self) -> usize {
        self.chunks.values().filter(|c| c.is_present()).count()
    }

    /// Updates the streamed chunks around the given world position.
    pub fn update(&mut self, player_pos: Vec3) -> StreamUpdate {
        let player_chunk = self.world_to_chunk(player_pos);

        let mut needed: Vec<ChunkCoord> = Vec::new();
        for dx in -self.load_radius..=self.load_radius {
            for dy in -self.load_radius..=self.load_radius {
                for dz in -self.load_radius..=self.load_radius {
                    let coord = ChunkCoord::new(
                        player_chunk.x + dx,
                        player_chunk.y + dy,
                        player_chunk.z + dz,
                    );
                    needed.push(coord);
                }
            }
        }

        let mut loaded = 0;
        for coord in &needed {
            if !self.chunks.contains_key(coord) {
                let mut chunk = Chunk::new(*coord, self.universe_seed);
                chunk.activate();
                self.chunks.insert(*coord, chunk);
                loaded += 1;
            }
        }

        let mut unloaded = 0;
        let coords_to_remove: Vec<ChunkCoord> = self
            .chunks
            .keys()
            .filter(|coord| {
                let dx = (coord.x - player_chunk.x).abs();
                let dy = (coord.y - player_chunk.y).abs();
                let dz = (coord.z - player_chunk.z).abs();
                dx > self.load_radius || dy > self.load_radius || dz > self.load_radius
            })
            .copied()
            .collect();

        for coord in coords_to_remove {
            self.chunks.remove(&coord);
            unloaded += 1;
        }

        StreamUpdate { loaded, unloaded }
    }

    /// Converts a world-space position to chunk coordinates.
    fn world_to_chunk(&self, pos: Vec3) -> ChunkCoord {
        ChunkCoord::new(
            (pos.x / CHUNK_SIZE).floor() as i32,
            (pos.y / CHUNK_SIZE).floor() as i32,
            (pos.z / CHUNK_SIZE).floor() as i32,
        )
    }

    /// Returns a reference to a loaded chunk, if it exists.
    pub fn get_chunk(&self, coord: &ChunkCoord) -> Option<&Chunk> {
        self.chunks.get(coord)
    }

    /// Evaluates the world at a position using the current reality state.
    pub fn evaluate_at(&self, pos: Vec3, kernel: &RealityKernel) -> f64 {
        let world_gen = WorldGenerator::new(kernel);
        world_gen.evaluate_sdf(pos)
    }
}

/// Summary of a streaming update.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StreamUpdate {
    pub loaded: usize,
    pub unloaded: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::seed::Seed;

    #[test]
    fn streamer_loads_chunks_around_origin() {
        let universe = Seed::new(666);
        let mut streamer = WorldStreamer::new(universe, 1);

        let update = streamer.update(Vec3::new(0.0, 0.0, 0.0));

        assert_eq!(update.loaded, 27);
        assert_eq!(streamer.loaded_count(), 27);
    }

    #[test]
    fn streamer_unloads_distant_chunks() {
        let universe = Seed::new(666);
        let mut streamer = WorldStreamer::new(universe, 1);

        streamer.update(Vec3::new(0.0, 0.0, 0.0));
        assert_eq!(streamer.loaded_count(), 27);

        let update = streamer.update(Vec3::new(1000.0, 0.0, 0.0));

        assert!(update.unloaded > 0);
        assert_eq!(streamer.loaded_count(), 27);
    }

    #[test]
    fn streamer_is_deterministic() {
        let universe = Seed::new(666);
        let mut streamer_a = WorldStreamer::new(universe, 1);
        let mut streamer_b = WorldStreamer::new(universe, 1);

        let pos = Vec3::new(5.0, 5.0, 5.0);
        let update_a = streamer_a.update(pos);
        let update_b = streamer_b.update(pos);

        assert_eq!(update_a, update_b);
        assert_eq!(streamer_a.loaded_count(), streamer_b.loaded_count());
    }
}
