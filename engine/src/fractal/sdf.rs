//! ⛧-Doom-Slayer-⛧
//! Signed Distance Functions (SDF) and Fractal Evaluators.

use super::math::Vec3;

/// Evaluates the signed distance from a point to a sphere.
pub fn sdf_sphere(pos: Vec3, center: Vec3, radius: f64) -> f64 {
    (pos - center).length() - radius
}

/// Evaluates the Mandelbulb fractal distance estimator.
pub fn sdf_mandelbulb(pos: Vec3, iterations: u32, power: f64) -> f64 {
    let mut z = pos;
    let mut dr = 1.0;
    let mut r = 0.0;

    for _ in 0..iterations {
        r = z.length();
        if r > 2.0 {
            break; // Escape radius
        }

        // Guard against division by zero at the mathematical origin
        let theta = if r == 0.0 {
            0.0
        } else {
            (z.y / r).clamp(-1.0, 1.0).acos()
        };
        let phi = if r == 0.0 { 0.0 } else { z.z.atan2(z.x) };

        dr = r.powf(power - 1.0) * power * dr + 1.0;

        let zr = r.powf(power);
        let theta = theta * power;
        let phi = phi * power;

        let sin_theta = theta.sin();
        z = Vec3::new(sin_theta * phi.cos(), theta.cos(), sin_theta * phi.sin()) * zr + pos;
    }

    if r == 0.0 { 0.0 } else { 0.5 * r.ln() * r / dr }
}

pub struct Scene;

impl Scene {
    pub fn evaluate(pos: Vec3) -> f64 {
        sdf_mandelbulb(pos, 8, 8.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mandelbulb_center_is_inside() {
        // At the exact origin, distance should be 0.0 (safely < 0.1)
        let d = Scene::evaluate(Vec3::zero());
        assert!(d < 0.1);
    }

    #[test]
    fn mandelbulb_far_away_is_positive() {
        let d = Scene::evaluate(Vec3::new(10.0, 0.0, 0.0));
        assert!(d > 1.0);
    }

    #[test]
    fn sphere_distance_is_correct() {
        let center = Vec3::zero();
        let d_inside = sdf_sphere(Vec3::new(0.0, 0.0, 0.0), center, 1.0);
        assert!((d_inside - (-1.0)).abs() < f64::EPSILON);

        let d_outside = sdf_sphere(Vec3::new(3.0, 0.0, 0.0), center, 1.0);
        assert!((d_outside - 2.0).abs() < f64::EPSILON);
    }
}
