//! ⛧-Doom-Slayer-⛧
//! Abyss state machine: reality states, transitions, and mutation history.

pub mod log;
pub mod state;

pub use log::{MutationLog, MutationRecord};
pub use state::{AbyssState, MutationEvent};
