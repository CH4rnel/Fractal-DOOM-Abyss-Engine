//! ⛧-Doom-Slayer-⛧
//! Morphology parameters defining a demon's physical structure.

use crate::core::random::stream::RandomStream;

/// Physical structure of a demon.
#[derive(Debug, Clone, PartialEq)]
pub struct Anatomy {
    /// Overall body scale (0.5 to 3.5).
    pub body_scale: f64,
    /// Number of limbs (2 to 8).
    pub limb_count: u32,
    /// Relative limb length (0.3 to 2.3).
    pub limb_length: f64,
    /// Torso shape classification.
    pub torso_shape: TorsoShape,
    /// Body symmetry type.
    pub symmetry: Symmetry,
}

/// Torso shape classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TorsoShape {
    Compact,
    Elongated,
    Compressed,
    Spherical,
}

/// Body symmetry type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Symmetry {
    Bilateral,
    Radial,
    Asymmetric,
}

impl Anatomy {
    /// Derives anatomy parameters from a deterministic random stream.
    pub fn derive(stream: &mut RandomStream) -> Self {
        let body_scale = 0.5 + (stream.next_u64() % 1000) as f64 / 1000.0 * 3.0;
        let limb_count = 2 + (stream.next_u64() % 7) as u32;
        let limb_length = 0.3 + (stream.next_u64() % 1000) as f64 / 1000.0 * 2.0;

        let torso_shape = match stream.next_u64() % 4 {
            0 => TorsoShape::Compact,
            1 => TorsoShape::Elongated,
            2 => TorsoShape::Compressed,
            _ => TorsoShape::Spherical,
        };

        let symmetry = match stream.next_u64() % 3 {
            0 => Symmetry::Bilateral,
            1 => Symmetry::Radial,
            _ => Symmetry::Asymmetric,
        };

        Self {
            body_scale,
            limb_count,
            limb_length,
            torso_shape,
            symmetry,
        }
    }

    /// Returns a stable textual identifier for the torso shape.
    pub fn torso_shape_str(&self) -> &'static str {
        match self.torso_shape {
            TorsoShape::Compact => "COMPACT",
            TorsoShape::Elongated => "ELONGATED",
            TorsoShape::Compressed => "COMPRESSED",
            TorsoShape::Spherical => "SPHERICAL",
        }
    }

    /// Returns a stable textual identifier for the symmetry.
    pub fn symmetry_str(&self) -> &'static str {
        match self.symmetry {
            Symmetry::Bilateral => "BILATERAL",
            Symmetry::Radial => "RADIAL",
            Symmetry::Asymmetric => "ASYMMETRIC",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::random::domain::RandomDomain;
    use crate::core::seed::Seed;

    #[test]
    fn anatomy_is_deterministic() {
        let mut stream_a = RandomStream::new(Seed::new(666), RandomDomain::Demons);
        let mut stream_b = RandomStream::new(Seed::new(666), RandomDomain::Demons);

        let anatomy_a = Anatomy::derive(&mut stream_a);
        let anatomy_b = Anatomy::derive(&mut stream_b);

        assert_eq!(anatomy_a, anatomy_b);
    }

    #[test]
    fn anatomy_values_are_in_bounds() {
        let mut stream = RandomStream::new(Seed::new(666), RandomDomain::Demons);
        let anatomy = Anatomy::derive(&mut stream);

        assert!(anatomy.body_scale >= 0.5 && anatomy.body_scale <= 3.5);
        assert!(anatomy.limb_count >= 2 && anatomy.limb_count <= 8);
        assert!(anatomy.limb_length >= 0.3 && anatomy.limb_length <= 2.3);
    }

    #[test]
    fn anatomy_str_methods_work() {
        let mut stream = RandomStream::new(Seed::new(666), RandomDomain::Demons);
        let anatomy = Anatomy::derive(&mut stream);

        // Verify string methods don't panic and return non-empty strings
        assert!(!anatomy.torso_shape_str().is_empty());
        assert!(!anatomy.symmetry_str().is_empty());
    }
}