//! ⛧-Doom-Slayer-⛧
//! Gameplay systems: entities, combat, weapons, navigation.

pub mod combat;
pub mod damage;
pub mod entity;
pub mod navigation;
pub mod player;
pub mod weapon;

pub use entity::{EntityId, EntityState};
pub use player::Player;
pub use weapon::{Weapon, WeaponType};
pub use damage::{DamageEvent, DamageType, DamageResult, GeometryDeformation};
pub use combat::{FractalHitZone, RaycastHit, raycast};
pub use navigation::{NavigationPath, find_path, move_towards, has_line_of_sight};