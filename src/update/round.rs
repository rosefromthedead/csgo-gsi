use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Round {
    phase: Phase,
    bomb: Option<BombState>,
    win_team: Option<super::Team>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Phase {
    Live,
    Over,
    FreezeTime,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum BombState {
    Planted,
    Defused,
    Exploded,
}
