//! ⛧-Doom-Slayer-⛧
//! Player entity with movement and orientation.

use super::entity::{EntityId, EntityState};
use crate::fractal::Vec3;

/// Player entity with position, orientation, and movement.
#[derive(Debug, Clone, PartialEq)]
pub struct Player {
    pub entity: EntityState,
    pub yaw: f64,
    pub pitch: f64,
    pub speed: f64,
    pub jump_velocity: f64,
}

impl Player {
    pub fn new(position: Vec3) -> Self {
        Self {
            entity: EntityState::new(EntityId::new(0), position, 100.0),
            yaw: 0.0,
            pitch: 0.0,
            speed: 10.0,
            jump_velocity: 5.0,
        }
    }

    pub fn forward_direction(&self) -> Vec3 {
        let cos_pitch = self.pitch.cos();
        Vec3::new(
            self.yaw.cos() * cos_pitch,
            self.pitch.sin(),
            self.yaw.sin() * cos_pitch,
        )
    }

    pub fn move_direction(&mut self, direction: Vec3, delta_time: f64) {
        let normalized = self.normalize(direction);
        self.entity.velocity = normalized * self.speed;
        self.entity.update_movement(delta_time);
        self.entity.velocity = Vec3::new(0.0, 0.0, 0.0);
    }

    pub fn rotate(&mut self, yaw_delta: f64, pitch_delta: f64) {
        self.yaw += yaw_delta;
        self.pitch = (self.pitch + pitch_delta).clamp(-1.5, 1.5);
    }

    pub fn jump(&mut self) {
        self.entity.velocity = Vec3::new(0.0, self.jump_velocity, 0.0);
    }

    fn normalize(&self, v: Vec3) -> Vec3 {
        let len = v.length();
        if len < 0.0001 {
            Vec3::new(0.0, 0.0, 0.0)
        } else {
            v * (1.0 / len)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn player_forward_at_zero() {
        let player = Player::new(Vec3::new(0.0, 0.0, 0.0));
        let fwd = player.forward_direction();
        assert!((fwd.z - 1.0).abs() < 0.001);
    }

    #[test]
    fn player_rotation_clamps() {
        let mut player = Player::new(Vec3::new(0.0, 0.0, 0.0));
        player.rotate(0.0, 10.0);
        assert!(player.pitch <= 1.5);
    }

    #[test]
    fn player_movement() {
        let mut player = Player::new(Vec3::new(0.0, 0.0, 0.0));
        player.move_direction(Vec3::new(1.0, 0.0, 0.0), 1.0);
        assert!(player.entity.position.x > 0.0);
    }
}
