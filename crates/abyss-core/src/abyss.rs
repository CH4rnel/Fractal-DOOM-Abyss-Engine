//! ⛧-Doom-Slayer-⛧
//! §B6: Universe Signature & Integrity (Merkle Log).

use crate::seed::Seed;
use blake3::Hasher;

/// §B6.1: Generator version for signature compatibility.
/// Bump this when changing the mutation log structure or hashing algorithm.
pub const ABYSS_GENERATOR_VERSION: u32 = 1;

/// Represents the state of the Abyss.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)] // Critical for deterministic hashing across platforms
pub enum AbyssState {
    Surface = 0,
    Descent = 1,
    Corruption = 2,
    Mutation = 3,
    Inferno = 4,
    Collapse = 5,
    Void = 6,
}

impl AbyssState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Surface => "surface",
            Self::Descent => "descent",
            Self::Corruption => "corruption",
            Self::Mutation => "mutation",
            Self::Inferno => "inferno",
            Self::Collapse => "collapse",
            Self::Void => "void",
        }
    }

    fn to_bytes(self) -> [u8; 1] {
        [self as u8]
    }
}

/// Represents a mutation event in the reality kernel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)] // Critical for deterministic hashing across platforms
pub enum MutationEvent {
    Descend = 0,
    Corrupt = 1,
    Fracture = 2,
}

impl MutationEvent {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Descend => "descend",
            Self::Corrupt => "corrupt",
            Self::Fracture => "fracture",
        }
    }

    fn to_bytes(self) -> [u8; 1] {
        [self as u8]
    }
}

/// A single record in the mutation log.
#[derive(Debug, Clone, PartialEq)]
pub struct MutationRecord {
    pub tick: u64,
    pub event: MutationEvent,
    pub previous_state: AbyssState,
    pub new_state: AbyssState,
    pub entropy_delta: f64,
    pub corruption_delta: f64,
}

impl MutationRecord {
    /// §B6.1: Computes the leaf hash for this record.
    /// leaf_hash(t) = BLAKE3( tick || event_kind || prev_state || new_state || entropy_delta || corruption_delta )
    pub fn leaf_hash(&self) -> [u8; 32] {
        let mut hasher = Hasher::new();
        hasher.update(&self.tick.to_le_bytes());
        hasher.update(&self.event.to_bytes());
        hasher.update(&self.previous_state.to_bytes());
        hasher.update(&self.new_state.to_bytes());
        hasher.update(&self.entropy_delta.to_le_bytes());
        hasher.update(&self.corruption_delta.to_le_bytes());
        *hasher.finalize().as_bytes()
    }
}

/// The authoritative mutation log with incremental Merkle root.
#[derive(Debug, Clone, Default)]
pub struct MutationLog {
    entries: Vec<MutationRecord>,
    current_root: [u8; 32],
}

impl MutationLog {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            current_root: [0u8; 32], // Initial root is all zeros
        }
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn history(&self) -> &[MutationRecord] {
        &self.entries
    }

    pub fn current_root(&self) -> [u8; 32] {
        self.current_root
    }

    /// Records a new mutation, updates the Merkle root.
    pub fn record(
        &mut self,
        tick: u64,
        event: MutationEvent,
        previous_state: AbyssState,
        new_state: AbyssState,
        entropy_delta: f64,
        corruption_delta: f64,
    ) {
        let record = MutationRecord {
            tick,
            event,
            previous_state,
            new_state,
            entropy_delta,
            corruption_delta,
        };

        let leaf_hash = record.leaf_hash();

        // §B6.1: root(t) = BLAKE3( root(t-1) || leaf_hash(t) )
        let mut hasher = Hasher::new();
        hasher.update(&self.current_root);
        hasher.update(&leaf_hash);
        self.current_root = *hasher.finalize().as_bytes();

        self.entries.push(record);
    }

    /// §B6.1: Computes the Universe Signature.
    /// Σ(t) = BLAKE3( seed || ABYSS_GENERATOR_VERSION || root(t) || tick )
    pub fn signature(&self, seed: Seed, tick: u64) -> [u8; 32] {
        let mut hasher = Hasher::new();
        hasher.update(&seed.value().to_le_bytes());
        hasher.update(&ABYSS_GENERATOR_VERSION.to_le_bytes());
        hasher.update(&self.current_root);
        hasher.update(&tick.to_le_bytes());
        *hasher.finalize().as_bytes()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merkle_root_changes_on_record() {
        let mut log = MutationLog::new();
        let initial_root = log.current_root();

        log.record(
            0,
            MutationEvent::Descend,
            AbyssState::Surface,
            AbyssState::Descent,
            0.05,
            0.0,
        );

        assert_ne!(log.current_root(), initial_root);
    }

    #[test]
    fn signature_is_deterministic() {
        let mut log1 = MutationLog::new();
        let mut log2 = MutationLog::new();
        let seed = Seed::new(42);

        log1.record(
            0,
            MutationEvent::Descend,
            AbyssState::Surface,
            AbyssState::Descent,
            0.1,
            0.0,
        );
        log2.record(
            0,
            MutationEvent::Descend,
            AbyssState::Surface,
            AbyssState::Descent,
            0.1,
            0.0,
        );

        assert_eq!(log1.signature(seed, 1), log2.signature(seed, 1));
    }

    /// §B13 item 3: Golden-master snapshot test.
    /// This test establishes a cryptographic baseline. If the signature changes,
    /// it means the mutation log structure or hashing algorithm has changed.
    /// In that case, ABYSS_GENERATOR_VERSION must be bumped.
    #[test]
    fn golden_signature_snapshot() {
        let mut log = MutationLog::new();
        let seed = Seed::new(666);

        log.record(
            0,
            MutationEvent::Descend,
            AbyssState::Surface,
            AbyssState::Descent,
            0.05,
            0.0,
        );
        log.record(
            1,
            MutationEvent::Corrupt,
            AbyssState::Descent,
            AbyssState::Corruption,
            0.0,
            0.35,
        );
        log.record(
            2,
            MutationEvent::Fracture,
            AbyssState::Corruption,
            AbyssState::Mutation,
            0.2,
            0.2,
        );

        let sig = log.signature(seed, 3);

        // 1. This line will output the correct hash to the console when the test runs.
        println!("🔑 GOLDEN_SIG: {:?}", sig);

        // 2. A 32-byte temporary placeholder to allow the code to compile.
        const EXPECTED_SIG: [u8; 32] = [
            249, 168, 102, 208, 75, 114, 104, 236, 206, 56, 32, 209, 113, 122, 238, 21, 38, 43,
            165, 224, 228, 132, 129, 112, 133, 158, 185, 125, 126, 84, 1, 154,
        ];

        // check test
        assert_eq!(
            sig, EXPECTED_SIG,
            "Golden signature mismatch! Algorithm or structure changed."
        );
    }
}
