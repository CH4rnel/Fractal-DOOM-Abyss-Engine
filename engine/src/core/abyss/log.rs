//! ⛧-Doom-Slayer-⛧
//! Authoritative chronological history of universe mutations.

use super::state::{AbyssState, MutationEvent};

/// A single historical record of a reality transition.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MutationRecord {
    pub tick: u64,
    pub event: MutationEvent,
    pub previous_state: AbyssState,
    pub new_state: AbyssState,
    pub entropy_delta: f64,
    pub corruption_delta: f64,
}

/// The authoritative chronological history of the universe's mutations.
///
/// This log is the foundation for replay, multiplayer synchronization,
/// AI lore generation, persistence, and divergence detection.
#[derive(Debug, Clone, Default)]
pub struct MutationLog {
    records: Vec<MutationRecord>,
}

impl MutationLog {
    /// Creates an empty mutation log.
    pub fn new() -> Self {
        Self {
            records: Vec::new(),
        }
    }

    /// Appends a new mutation record to the history.
    pub fn record(
        &mut self,
        tick: u64,
        event: MutationEvent,
        previous_state: AbyssState,
        new_state: AbyssState,
        entropy_delta: f64,
        corruption_delta: f64,
    ) {
        self.records.push(MutationRecord {
            tick,
            event,
            previous_state,
            new_state,
            entropy_delta,
            corruption_delta,
        });
    }

    /// Returns the immutable mutation history.
    pub fn history(&self) -> &[MutationRecord] {
        &self.records
    }

    /// Returns the number of recorded mutations.
    pub fn len(&self) -> usize {
        self.records.len()
    }

    /// Returns true if no mutations have been recorded.
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn log_records_events_chronologically() {
        let mut log = MutationLog::new();
        assert!(log.is_empty());

        log.record(
            1,
            MutationEvent::Descend,
            AbyssState::Surface,
            AbyssState::Descent,
            0.05,
            0.0,
        );
        log.record(
            2,
            MutationEvent::Corrupt,
            AbyssState::Descent,
            AbyssState::Corruption,
            0.0,
            0.4,
        );

        assert_eq!(log.len(), 2);
        assert_eq!(log.history()[0].event, MutationEvent::Descend);
        assert_eq!(log.history()[1].new_state, AbyssState::Corruption);
    }
}
