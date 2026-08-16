//! ⛧-Doom-Slayer-⛧
//! World generation: SDF evaluation, fractal caves, corruption zones.

use crate::core::reality::RealityKernel;
use crate::fractal::math::Vec3;
use crate::fractal::sdf::sdf_mandelbulb;

use super::biome::Biome;
use super::chunk::{CHUNK_SIZE, Chunk};

/// The Abyss World Generator evaluates the mathematical density
/// of the world at any given point.
pub struct WorldGenerator<'a> {
    kernel: &'a RealityKernel,
}

impl<'a> WorldGenerator<'a> {
    /// Creates a new world generator bound to the current reality state.
    pub fn new(kernel: &'a RealityKernel) -> Self {
        Self { kernel }
    }

    /// Returns the current biome based on reality state.
    pub fn biome(&self) -> Biome {
        Biome::from_state(self.kernel.state())
    }

    /// Evaluates the signed distance to the nearest solid surface.
    pub fn evaluate_sdf(&self, pos: Vec3) -> f64 {
        let biome = self.biome();
        let fractal_pos = pos * 0.1;

        let fractal_distance = sdf_mandelbulb(
            fractal_pos,
            biome.fractal_iterations(),
            biome.fractal_power(),
        );

        let corruption = self.kernel.corruption() * biome.corruption_intensity();
        let warped_distance = self.apply_corruption(fractal_distance, pos, corruption);

        warped_distance - biome.density_threshold()
    }

    /// Evaluates whether a point is inside solid matter.
    pub fn is_solid(&self, pos: Vec3) -> bool {
        self.evaluate_sdf(pos) < 0.0
    }

    /// Evaluates terrain density at a point (0.0 = empty, 1.0 = solid).
    pub fn density(&self, pos: Vec3) -> f64 {
        let sdf = self.evaluate_sdf(pos);
        (-sdf).clamp(0.0, 1.0)
    }

    /// Applies corruption warping to the distance field.
    fn apply_corruption(&self, distance: f64, pos: Vec3, corruption: f64) -> f64 {
        if corruption <= 0.0 {
            return distance;
        }
        let noise = self.positional_noise(pos);
        let warp = noise * corruption * 0.5;
        distance + warp
    }

    /// Generates deterministic pseudo-noise from a position.
    fn positional_noise(&self, pos: Vec3) -> f64 {
        let x = pos.x * 12.9898;
        let y = pos.y * 78.233;
        let z = pos.z * 37.719;
        let combined = x + y + z;
        combined.sin()
    }

    /// Evaluates the world within a chunk, returning a summary.
    pub fn evaluate_chunk(&self, chunk: &Chunk) -> ChunkEvaluation {
        let (ox, oy, oz) = chunk.coord.origin();
        let center = Vec3::new(
            ox + CHUNK_SIZE / 2.0,
            oy + CHUNK_SIZE / 2.0,
            oz + CHUNK_SIZE / 2.0,
        );

        let center_sdf = self.evaluate_sdf(center);
        let center_density = self.density(center);
        let biome = self.biome();

        ChunkEvaluation {
            coord: chunk.coord,
            biome,
            center_sdf,
            center_density,
            is_mostly_solid: center_density > 0.5,
        }
    }
}

/// Summary of a chunk's world evaluation.
#[derive(Debug, Clone)]
pub struct ChunkEvaluation {
    pub coord: super::chunk::ChunkCoord,
    pub biome: Biome,
    pub center_sdf: f64,
    pub center_density: f64,
    pub is_mostly_solid: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::seed::Seed;
    use crate::world::chunk::ChunkCoord;

    #[test]
    fn world_generator_is_deterministic() {
        let kernel = RealityKernel::new(Seed::new(666));
        let world_gen = WorldGenerator::new(&kernel);

        let pos = Vec3::new(1.0, 2.0, 3.0);
        let sdf_a = world_gen.evaluate_sdf(pos);
        let sdf_b = world_gen.evaluate_sdf(pos);

        assert_eq!(sdf_a, sdf_b);
    }

    #[test]
    fn biome_changes_with_reality_state() {
        let mut kernel = RealityKernel::new(Seed::new(666));

        // Temporary generator avoids borrow conflict across mutation.
        assert_eq!(WorldGenerator::new(&kernel).biome(), Biome::Surface);

        kernel.descend();

        assert_eq!(WorldGenerator::new(&kernel).biome(), Biome::Descent);
    }

    #[test]
    fn chunk_evaluation_produces_valid_density() {
        let kernel = RealityKernel::new(Seed::new(666));
        let world_gen = WorldGenerator::new(&kernel);

        let coord = ChunkCoord::new(0, 0, 0);
        let chunk = Chunk::new(coord, kernel.seed());
        let eval = world_gen.evaluate_chunk(&chunk);

        assert!(eval.center_density >= 0.0);
        assert!(eval.center_density <= 1.0);
    }
}
