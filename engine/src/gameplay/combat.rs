//! ⛧-Doom-Slayer-⛧
//! Fractal hit zones and SDF-based collision detection.

use crate::fractal::Vec3;
use crate::world::WorldGenerator;

/// Fractal hit zone using SDF evaluation.
#[derive(Debug, Clone, PartialEq)]
pub struct FractalHitZone {
    pub center: Vec3,
    pub radius: f64,
}

impl FractalHitZone {
    pub fn new(center: Vec3, radius: f64) -> Self {
        Self { center, radius }
    }

    pub fn contains_point(&self, point: Vec3) -> bool {
        (point - self.center).length() <= self.radius
    }
}

/// Raycast hit result.
#[derive(Debug, Clone, PartialEq)]
pub struct RaycastHit {
    pub position: Vec3,
    pub distance: f64,
    pub surface_normal: Vec3,
}

/// Raycast through fractal geometry.
pub fn raycast(
    origin: Vec3,
    direction: Vec3,
    max_distance: f64,
    world: &WorldGenerator,
) -> Option<RaycastHit> {
    let step = 0.1;
    let mut dist = 0.0;

    while dist < max_distance {
        let point = origin + direction * dist;
        if world.evaluate_sdf(point) <= 0.0 {
            return Some(RaycastHit {
                position: point,
                distance: dist,
                surface_normal: compute_normal(point, world),
            });
        }
        dist += step;
    }
    None
}

fn compute_normal(point: Vec3, world: &WorldGenerator) -> Vec3 {
    let eps = 0.001;
    let dx = world.evaluate_sdf(point + Vec3::new(eps, 0.0, 0.0))
        - world.evaluate_sdf(point - Vec3::new(eps, 0.0, 0.0));
    let dy = world.evaluate_sdf(point + Vec3::new(0.0, eps, 0.0))
        - world.evaluate_sdf(point - Vec3::new(0.0, eps, 0.0));
    let dz = world.evaluate_sdf(point + Vec3::new(0.0, 0.0, eps))
        - world.evaluate_sdf(point - Vec3::new(0.0, 0.0, eps));

    let n = Vec3::new(dx, dy, dz);
    let len = n.length();
    if len < 0.0001 {
        Vec3::new(0.0, 1.0, 0.0)
    } else {
        n * (1.0 / len)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hit_zone_contains_point() {
        let zone = FractalHitZone::new(Vec3::new(0.0, 0.0, 0.0), 5.0);
        assert!(zone.contains_point(Vec3::new(1.0, 1.0, 1.0)));
        assert!(!zone.contains_point(Vec3::new(10.0, 0.0, 0.0)));
    }
}