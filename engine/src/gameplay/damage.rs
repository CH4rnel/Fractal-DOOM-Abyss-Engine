//! ⛧-Doom-Slayer-⛧
//! Damage model with geometry modification.
//! Core principle: damage modifies geometry, not just HP.

use super::entity::EntityId;
use crate::fractal::Vec3;

/// Type of damage being applied.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DamageType {
    Physical,
    Fractal,
    Corruption,
    Void,
}

/// A damage event.
#[derive(Debug, Clone, PartialEq)]
pub struct DamageEvent {
    pub source: EntityId,
    pub target: EntityId,
    pub amount: f64,
    pub damage_type: DamageType,
    pub position: Vec3,
}

impl DamageEvent {
    pub fn new(
        source: EntityId,
        target: EntityId,
        amount: f64,
        damage_type: DamageType,
        position: Vec3,
    ) -> Self {
        Self {
            source,
            target,
            amount,
            damage_type,
            position,
        }
    }
}

/// Result of applying damage.
#[derive(Debug, Clone, PartialEq)]
pub struct DamageResult {
    pub target_died: bool,
    pub damage_applied: f64,
    pub geometry_deformation: f64,
}

/// Applies damage to an entity.
pub fn apply_damage(target: &mut super::entity::EntityState, event: &DamageEvent) -> DamageResult {
    let modifier = match event.damage_type {
        DamageType::Physical => 1.0,
        DamageType::Fractal => 1.2,
        DamageType::Corruption => 1.5,
        DamageType::Void => 2.0,
    };
    let effective = event.amount * modifier;
    let died = target.apply_damage(effective);

    let geo_def = if event.damage_type == DamageType::Fractal {
        effective * 0.1
    } else {
        0.0
    };

    DamageResult {
        target_died: died,
        damage_applied: effective,
        geometry_deformation: geo_def,
    }
}

/// A deformation applied to world geometry via SDF.
#[derive(Debug, Clone, PartialEq)]
pub struct GeometryDeformation {
    pub center: Vec3,
    pub radius: f64,
    pub depth: f64,
}

impl GeometryDeformation {
    pub fn new(impact: Vec3, damage: f64) -> Self {
        Self {
            center: impact,
            radius: (damage * 0.05).max(0.1),
            depth: damage * 0.02,
        }
    }

    /// Evaluates deformation at a point (negative = carve into geometry).
    pub fn evaluate_at(&self, point: Vec3) -> f64 {
        let dist = (point - self.center).length();
        if dist >= self.radius {
            return 0.0;
        }
        let falloff = 1.0 - (dist / self.radius);
        let smooth = falloff * falloff * (3.0 - 2.0 * falloff);
        -self.depth * smooth
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gameplay::entity::EntityState;

    #[test]
    fn physical_damage_full() {
        let mut e = EntityState::new(EntityId::new(1), Vec3::zero(), 100.0);
        let ev = DamageEvent::new(
            EntityId::new(0),
            EntityId::new(1),
            50.0,
            DamageType::Physical,
            Vec3::zero(),
        );
        let r = apply_damage(&mut e, &ev);
        assert!((r.damage_applied - 50.0).abs() < 0.001);
    }

    #[test]
    fn fractal_damage_bonus() {
        let mut e = EntityState::new(EntityId::new(1), Vec3::zero(), 100.0);
        let ev = DamageEvent::new(
            EntityId::new(0),
            EntityId::new(1),
            50.0,
            DamageType::Fractal,
            Vec3::zero(),
        );
        let r = apply_damage(&mut e, &ev);
        assert!(r.damage_applied > 50.0);
        assert!(r.geometry_deformation > 0.0);
    }

    #[test]
    fn deformation_carves() {
        let d = GeometryDeformation::new(Vec3::zero(), 100.0);
        let at_center = d.evaluate_at(Vec3::zero());
        let outside = d.evaluate_at(Vec3::new(100.0, 0.0, 0.0));
        assert!(at_center < 0.0);
        assert!(outside.abs() < 0.001);
    }
}
