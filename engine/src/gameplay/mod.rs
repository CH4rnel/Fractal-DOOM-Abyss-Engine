//! ⛧-Doom-Slayer-⛧
//! Gameplay systems: entities, combat, weapons, navigation.

pub mod combat;
pub mod damage;
pub mod entity;
pub mod navigation;
pub mod player;
pub mod weapon;

pub use combat::{FractalHitZone, RaycastHit, raycast};
pub use damage::{DamageEvent, DamageResult, DamageType, GeometryDeformation};
pub use entity::{EntityId, EntityState};
pub use navigation::{NavigationPath, find_path, has_line_of_sight, move_towards};
pub use player::Player;
pub use weapon::{Weapon, WeaponType};
