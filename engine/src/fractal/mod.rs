//! ⛧-Doom-Slayer-⛧
//! Fractal Mathematics and Spatial Evaluation.

pub mod math;
pub mod sdf;

// Expose core mathematical primitives and evaluators to the engine level
pub use math::Vec3;
pub use sdf::{Scene, sdf_sphere};
