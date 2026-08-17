//! ⛧-Doom-Slayer-⛧
//! Procedural enemy navigation through fractal terrain.

use crate::fractal::Vec3;
use crate::world::WorldGenerator;
use super::entity::EntityState;

/// Navigation path through the world.
#[derive(Debug, Clone, PartialEq)]
pub struct NavigationPath {
    pub waypoints: Vec<Vec3>,
    pub total_length: f64,
}

impl NavigationPath {
    pub fn new() -> Self {
        Self { waypoints: Vec::new(), total_length: 0.0 }
    }

    pub fn add_waypoint(&mut self, wp: Vec3) {
        if let Some(last) = self.waypoints.last() {
            self.total_length += (wp - *last).length();
        }
        self.waypoints.push(wp);
    }

    pub fn is_empty(&self) -> bool {
        self.waypoints.is_empty()
    }
}

/// Finds a path from start to goal.
pub fn find_path(
    start: Vec3,
    goal: Vec3,
    world: &WorldGenerator,
    max_steps: usize,
) -> NavigationPath {
    let mut path = NavigationPath::new();
    path.add_waypoint(start);

    let mut current = start;
    let step = 0.5;

    for _ in 0..max_steps {
        let to_goal = goal - current;
        let dist = to_goal.length();

        if dist < step {
            path.add_waypoint(goal);
            break;
        }

        let dir = to_goal * (1.0 / dist);
        let next = current + dir * step;

        if world.is_solid(next) {
            let perp = Vec3::new(-dir.z, 0.0, dir.x);
            let alt = current + perp * step;
            if !world.is_solid(alt) {
                current = alt;
            } else {
                let alt2 = current - perp * step;
                if !world.is_solid(alt2) {
                    current = alt2;
                } else {
                    break;
                }
            }
        } else {
            current = next;
        }
        path.add_waypoint(current);
    }
    path
}

/// Moves entity towards target.
pub fn move_towards(entity: &mut EntityState, target: Vec3, speed: f64, dt: f64) {
    let to = target - entity.position;
    let dist = to.length();
    if dist < 0.001 {
        entity.velocity = Vec3::zero();
        return;
    }
    entity.velocity = (to * (1.0 / dist)) * speed;
    entity.update_movement(dt);
}

/// Checks line of sight.
pub fn has_line_of_sight(from: Vec3, to: Vec3, world: &WorldGenerator, max_dist: f64) -> bool {
    let dir = to - from;
    let dist = dir.length();
    if dist > max_dist {
        return false;
    }
    let norm = dir * (1.0 / dist);
    let step = 0.2;
    let mut d = 0.0;
    while d < dist {
        if world.is_solid(from + norm * d) {
            return false;
        }
        d += step;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn path_accumulates() {
        let mut p = NavigationPath::new();
        p.add_waypoint(Vec3::zero());
        p.add_waypoint(Vec3::new(1.0, 0.0, 0.0));
        p.add_waypoint(Vec3::new(2.0, 0.0, 0.0));
        assert!((p.total_length - 2.0).abs() < 0.001);
    }
}