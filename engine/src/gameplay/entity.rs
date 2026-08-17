//! ⛧-Doom-Slayer-⛧
//! Base entity system for combat participants.

use crate::fractal::Vec3;

/// Unique identifier for an entity in the combat system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EntityId(pub u64);

impl EntityId {
    pub fn new(value: u64) -> Self {
        Self(value)
    }
}

/// Core state of any combat entity (player, demon, projectile).
#[derive(Debug, Clone, PartialEq)]
pub struct EntityState {
    pub id: EntityId,
    pub position: Vec3,
    pub velocity: Vec3,
    pub health: f64,
    pub max_health: f64,
    pub is_alive: bool,
}

impl EntityState {
    pub fn new(id: EntityId, position: Vec3, max_health: f64) -> Self {
        Self {
            id,
            position,
            velocity: Vec3::new(0.0, 0.0, 0.0),
            health: max_health,
            max_health,
            is_alive: true,
        }
    }

    pub fn apply_damage(&mut self, amount: f64) -> bool {
        if !self.is_alive {
            return false;
        }
        self.health = (self.health - amount).max(0.0);
        if self.health <= 0.0 {
            self.is_alive = false;
            return true;
        }
        false
    }

    pub fn health_fraction(&self) -> f64 {
        if self.max_health <= 0.0 {
            return 0.0;
        }
        (self.health / self.max_health).clamp(0.0, 1.0)
    }

    pub fn update_movement(&mut self, delta_time: f64) {
        self.position = self.position + self.velocity * delta_time;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entity_starts_alive() {
        let entity = EntityState::new(EntityId::new(1), Vec3::new(0.0, 0.0, 0.0), 100.0);
        assert!(entity.is_alive);
        assert_eq!(entity.health, 100.0);
    }

    #[test]
    fn entity_takes_damage() {
        let mut entity = EntityState::new(EntityId::new(1), Vec3::new(0.0, 0.0, 0.0), 100.0);
        let died = entity.apply_damage(30.0);
        assert!(!died);
        assert_eq!(entity.health, 70.0);
    }

    #[test]
    fn entity_dies_at_zero() {
        let mut entity = EntityState::new(EntityId::new(1), Vec3::new(0.0, 0.0, 0.0), 100.0);
        let died = entity.apply_damage(100.0);
        assert!(died);
        assert!(!entity.is_alive);
    }

    #[test]
    fn movement_updates_position() {
        let mut entity = EntityState::new(EntityId::new(1), Vec3::new(0.0, 0.0, 0.0), 100.0);
        entity.velocity = Vec3::new(1.0, 0.0, 0.0);
        entity.update_movement(2.0);
        assert_eq!(entity.position.x, 2.0);
    }
}
