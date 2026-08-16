//! ⛧-Doom-Slayer-⛧
//! Visual mutation, material profile, and audio profile.

use crate::core::random::stream::RandomStream;

/// Visual mutation and sensory profile of a demon.
#[derive(Debug, Clone, PartialEq)]
pub struct Mutation {
    /// Mutation level (0 = none, 5 = void-touched).
    pub mutation_level: u32,
    /// Type of mutation.
    pub mutation_type: MutationType,
    /// Surface material classification.
    pub material_profile: MaterialProfile,
    /// Audio signature classification.
    pub audio_profile: AudioProfile,
    /// Color hue (0.0 to 360.0).
    pub color_hue: f64,
}

/// Mutation type classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MutationType {
    None,
    Growth,
    Decay,
    Fusion,
    Crystallization,
    VoidTouch,
}

/// Surface material classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaterialProfile {
    Flesh,
    Stone,
    Metal,
    Crystal,
    Void,
}

/// Audio signature classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioProfile {
    Silent,
    Whisper,
    Growl,
    Roar,
    Scream,
    Harmonic,
}

impl Mutation {
    /// Derives mutation parameters from a deterministic random stream.
    pub fn derive(stream: &mut RandomStream) -> Self {
        let mutation_level = (stream.next_u64() % 6) as u32;

        let mutation_type = match mutation_level {
            0 => MutationType::None,
            1 => MutationType::Growth,
            2 => MutationType::Decay,
            3 => MutationType::Fusion,
            4 => MutationType::Crystallization,
            _ => MutationType::VoidTouch,
        };

        let material_profile = match stream.next_u64() % 5 {
            0 => MaterialProfile::Flesh,
            1 => MaterialProfile::Stone,
            2 => MaterialProfile::Metal,
            3 => MaterialProfile::Crystal,
            _ => MaterialProfile::Void,
        };

        let audio_profile = match stream.next_u64() % 6 {
            0 => AudioProfile::Silent,
            1 => AudioProfile::Whisper,
            2 => AudioProfile::Growl,
            3 => AudioProfile::Roar,
            4 => AudioProfile::Scream,
            _ => AudioProfile::Harmonic,
        };

        let color_hue = (stream.next_u64() % 360) as f64;

        Self {
            mutation_level,
            mutation_type,
            material_profile,
            audio_profile,
            color_hue,
        }
    }

    /// Returns a stable textual identifier for the mutation type.
    pub fn mutation_type_str(&self) -> &'static str {
        match self.mutation_type {
            MutationType::None => "NONE",
            MutationType::Growth => "GROWTH",
            MutationType::Decay => "DECAY",
            MutationType::Fusion => "FUSION",
            MutationType::Crystallization => "CRYSTALLIZATION",
            MutationType::VoidTouch => "VOID_TOUCH",
        }
    }

    /// Returns a stable textual identifier for the material profile.
    pub fn material_str(&self) -> &'static str {
        match self.material_profile {
            MaterialProfile::Flesh => "FLESH",
            MaterialProfile::Stone => "STONE",
            MaterialProfile::Metal => "METAL",
            MaterialProfile::Crystal => "CRYSTAL",
            MaterialProfile::Void => "VOID",
        }
    }

    /// Returns a stable textual identifier for the audio profile.
    pub fn audio_str(&self) -> &'static str {
        match self.audio_profile {
            AudioProfile::Silent => "SILENT",
            AudioProfile::Whisper => "WHISPER",
            AudioProfile::Growl => "GROWL",
            AudioProfile::Roar => "ROAR",
            AudioProfile::Scream => "SCREAM",
            AudioProfile::Harmonic => "HARMONIC",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::random::domain::RandomDomain;
    use crate::core::seed::Seed;

    #[test]
    fn mutation_is_deterministic() {
        let mut stream_a = RandomStream::new(Seed::new(666), RandomDomain::Demons);
        let mut stream_b = RandomStream::new(Seed::new(666), RandomDomain::Demons);

        let mutation_a = Mutation::derive(&mut stream_a);
        let mutation_b = Mutation::derive(&mut stream_b);

        assert_eq!(mutation_a, mutation_b);
    }

    #[test]
    fn mutation_level_matches_type() {
        let mut stream = RandomStream::new(Seed::new(666), RandomDomain::Demons);
        let mutation = Mutation::derive(&mut stream);

        let expected_type = match mutation.mutation_level {
            0 => MutationType::None,
            1 => MutationType::Growth,
            2 => MutationType::Decay,
            3 => MutationType::Fusion,
            4 => MutationType::Crystallization,
            _ => MutationType::VoidTouch,
        };

        assert_eq!(mutation.mutation_type, expected_type);
    }

    #[test]
    fn mutation_color_hue_in_bounds() {
        let mut stream = RandomStream::new(Seed::new(666), RandomDomain::Demons);
        let mutation = Mutation::derive(&mut stream);

        assert!(mutation.color_hue >= 0.0 && mutation.color_hue < 360.0);
    }

    #[test]
    fn mutation_str_methods_work() {
        let mut stream = RandomStream::new(Seed::new(666), RandomDomain::Demons);
        let mutation = Mutation::derive(&mut stream);

        assert!(!mutation.mutation_type_str().is_empty());
        assert!(!mutation.material_str().is_empty());
        assert!(!mutation.audio_str().is_empty());
    }
}
