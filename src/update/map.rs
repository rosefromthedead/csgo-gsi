use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Map {
    current_spectators: u64,
    mode: Mode,
    name: String,
    num_matches_to_win_series: u64,
    phase: Phase,
    round: u64,
    #[serde(default)]
    round_wins: HashMap<u64, RoundWin>,
    souvenirs_total: u64,
    team_ct: Team,
    team_t: Team,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    Competitive,
    Casual,
    Deathmatch,
    Training,
    #[serde(rename = "gungametrbomb")]
    Demolition,
    #[serde(rename = "gungameprogressive")]
    ArmsRace,
    #[serde(rename = "scrimcomp2v2")]
    Wingman,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Phase {
    Warmup,
    Live,
    Intermission,
    GameOver,
}

// TODO don't be stringly typed
// examples: "ct_win_time" "t_win_bomb" "ct_win_elimination" "ct_win_defuse"
pub type RoundWin = String;

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Team {
    score: u64,
    consecutive_round_losses: u64,
    timeouts_remaining: u64,
    matches_won_this_series: u64,
    name: Option<String>,
    flag: Option<String>,
}
