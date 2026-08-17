//! ⛧-Doom-Slayer-⛧
//! Weapon definitions and damage parameters.

/// Classification of weapon damage delivery.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeaponType {
    Melee,
    Ranged,
    Explosive,
    Fractal,
}

/// A weapon that can be used in combat.
#[derive(Debug, Clone, PartialEq)]
pub struct Weapon {
    pub id: u32,
    pub name: &'static str,
    pub damage: f64,
    pub fire_rate: f64,
    pub range: f64,
    pub weapon_type: WeaponType,
}

impl Weapon {
    pub fn new(
        id: u32,
        name: &'static str,
        damage: f64,
        fire_rate: f64,
        range: f64,
        weapon_type: WeaponType,
    ) -> Self {
        Self {
            id,
            name,
            damage,
            fire_rate,
            range,
            weapon_type,
        }
    }

    pub fn dps(&self) -> f64 {
        self.damage * self.fire_rate
    }

    pub fn weapon_type_str(&self) -> &'static str {
        match self.weapon_type {
            WeaponType::Melee => "MELEE",
            WeaponType::Ranged => "RANGED",
            WeaponType::Explosive => "EXPLOSIVE",
            WeaponType::Fractal => "FRACTAL",
        }
    }

    pub fn shotgun() -> Self {
        Self::new(1, "ABYSSAL SHOTGUN", 40.0, 1.5, 25.0, WeaponType::Ranged)
    }

    pub fn chainsaw() -> Self {
        Self::new(2, "FRACTAL CHAINSAW", 15.0, 8.0, 3.0, WeaponType::Melee)
    }

    pub fn fractal_rifle() -> Self {
        Self::new(
            3,
            "REALITY FRACTURE RIFLE",
            25.0,
            4.0,
            50.0,
            WeaponType::Fractal,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn weapon_dps() {
        let w = Weapon::shotgun();
        assert!((w.dps() - 60.0).abs() < 0.001);
    }

    #[test]
    fn weapon_types() {
        assert_eq!(Weapon::shotgun().weapon_type_str(), "RANGED");
        assert_eq!(Weapon::fractal_rifle().weapon_type_str(), "FRACTAL");
    }
}
