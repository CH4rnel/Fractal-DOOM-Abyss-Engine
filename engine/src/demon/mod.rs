//! ⛧-Doom-Slayer-⛧
//! Demon Seed Generator: deterministic procedural demon identities.

pub mod anatomy;
pub mod behavior;
pub mod genealogy;
pub mod identity;
pub mod mutation;

// Re-export only types actively used by the engine surface.
// Internal sub-types remain accessible through the identity's fields
// (e.g. identity.anatomy.body_scale) without polluting the public API.
pub use genealogy::ThreatLevel;
pub use identity::{DemonIdentity, DemonSeed};
