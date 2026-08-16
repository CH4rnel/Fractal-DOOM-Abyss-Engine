//! ⛧-Doom-Slayer-⛧
//! Behavioral parameters: movement, aggression, perception, attacks.

use crate::core::random::stream::RandomStream;

/// Behavioral profile of a demon.
#[derive(Debug, Clone, PartialEq)]
pub struct Behavior {
    /// How the demon moves through space.
    pub movement_archetype: MovementArchetype,
    /// Aggression level (0.0 passive to 1.0 berserk).
    pub aggression: f64,
    /// Movement speed multiplier (0.5 to 3.5).
    pub speed: f64,
    /// Detection range in world units (5.0 to 55.0).
    pub perception_range: f64,
    /// Primary attack pattern.
    pub attack_pattern: AttackPattern,
}

/// Movement archetype classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MovementArchetype {
    Crawler,
    Flyer,
    Burrower,
    PhaseShifter,
    Leviathan,
}

/// Attack pattern classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttackPattern {
    Melee,
    Ranged,
    AreaDenial,
    Summoning,
    Corruption,
}

impl Behavior {
    /// Derives behavior parameters from a deterministic random stream.
    pub fn derive(stream: &mut RandomStream) -> Self {
        let movement_archetype = match stream.next_u64() % 5 {
            0 => MovementArchetype::Crawler,
            1 => MovementArchetype::Flyer,
            2 => MovementArchetype::Burrower,
            3 => MovementArchetype::PhaseShifter,
            _ => MovementArchetype::Leviathan,
        };

        let aggression = (stream.next_u64() % 1000) as f64 / 1000.0;
        let speed = 0.5 + (stream.next_u64() % 1000) as f64 / 1000.0 * 3.0;
        let perception_range = 5.0 + (stream.next_u64() % 1000) as f64 / 1000.0 * 50.0;

        let attack_pattern = match stream.next_u64() % 5 {
            0 => AttackPattern::Melee,
            1 => AttackPattern::Ranged,
            2 => AttackPattern::AreaDenial,
            3 => AttackPattern::Summoning,
            _ => AttackPattern::Corruption,
        };

        Self {
            movement_archetype,
            aggression,
            speed,
            perception_range,
            attack_pattern,
        }
    }

    /// Returns a stable textual identifier for the movement archetype.
    pub fn movement_str(&self) -> &'static str {
        match self.movement_archetype {
            MovementArchetype::Crawler => "CRAWLER",
            MovementArchetype::Flyer => "FLYER",
            MovementArchetype::Burrower => "BURROWER",
            MovementArchetype::PhaseShifter => "PHASE_SHIFTER",
            MovementArchetype::Leviathan => "LEVIATHAN",
        }
    }

    /// Returns a stable textual identifier for the attack pattern.
    pub fn attack_str(&self) -> &'static str {
        match self.attack_pattern {
            AttackPattern::Melee => "MELEE",
            AttackPattern::Ranged => "RANGED",
            AttackPattern::AreaDenial => "AREA_DENIAL",
            AttackPattern::Summoning => "SUMMONING",
            AttackPattern::Corruption => "CORRUPTION",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::random::domain::RandomDomain;
    use crate::core::seed::Seed;

    #[test]
    fn behavior_is_deterministic() {
        let mut stream_a = RandomStream::new(Seed::new(666), RandomDomain::Demons);
        let mut stream_b = RandomStream::new(Seed::new(666), RandomDomain::Demons);

        let behavior_a = Behavior::derive(&mut stream_a);
        let behavior_b = Behavior::derive(&mut stream_b);

        assert_eq!(behavior_a, behavior_b);
    }

    #[test]
    fn behavior_values_are_in_bounds() {
        let mut stream = RandomStream::new(Seed::new(666), RandomDomain::Demons);
        let behavior = Behavior::derive(&mut stream);

        assert!(behavior.aggression >= 0.0 && behavior.aggression <= 1.0);
        assert!(behavior.speed >= 0.5 && behavior.speed <= 3.5);
        assert!(behavior.perception_range >= 5.0 && behavior.perception_range <= 55.0);
    }

    #[test]
    fn behavior_str_methods_work() {
        let mut stream = RandomStream::new(Seed::new(666), RandomDomain::Demons);
        let behavior = Behavior::derive(&mut stream);

        assert!(!behavior.movement_str().is_empty());
        assert!(!behavior.attack_str().is_empty());
    }
}
