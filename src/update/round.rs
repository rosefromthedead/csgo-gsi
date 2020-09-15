//! round-related info

use serde::{Serialize, Deserialize};

/// round info
#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Round {
    /// phase of round
    pub phase: Phase,
    /// status of bomb
    pub bomb: Option<BombState>,
    /// which team won
    pub win_team: Option<super::Team>,
}

/// round phase
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Phase {
    /// in progress
    Live,
    /// ended
    Over,
    /// hasn't yet started
    FreezeTime,
}

/// bomb state
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum BombState {
    /// planted
    Planted,
    /// defused
    Defused,
    /// exploded
    Exploded,
}
