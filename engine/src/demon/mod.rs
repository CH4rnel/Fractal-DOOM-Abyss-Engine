//! ⛧-Doom-Slayer-⛧
//! Demon Seed Generator: deterministic procedural demon identities.

pub mod anatomy;
pub mod behavior;
pub mod genealogy;
pub mod identity;
pub mod mutation;

pub use anatomy::{Anatomy, Symmetry, TorsoShape};
pub use behavior::{AttackPattern, Behavior, MovementArchetype};
pub use genealogy::{Genealogy, Lineage, ThreatLevel};
pub use identity::{DemonIdentity, DemonSeed};
pub use mutation::{AudioProfile, MaterialProfile, Mutation, MutationType};