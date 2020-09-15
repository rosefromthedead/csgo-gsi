//! all information that can be contained in a GSI update

use std::collections::HashMap;

use serde::{Serialize, Deserialize, de::IgnoredAny};

pub mod player;
use player::Player;

pub mod map;
use map::Map;

pub mod round;
use round::Round;

// TODO abuse generics to align subscriptions with these types

/// a team
#[derive(Debug, Deserialize, Serialize)]
pub enum Team {
    /// counter-terrorists
    CT,
    /// terrorists
    T,
}

/// an update received from CS:GO
#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Update {
    /// map info
    pub map: Option<Map>,
    /// player info
    pub player: Option<Player>,
    /// provider (CS:GO) info
    pub provider: Option<Provider>,
    /// authentication info, matching initial config
    pub auth: HashMap<String, String>,
    /// round info
    pub round: Option<Round>,
    #[serde(skip_serializing, default)]
    added: IgnoredAny,
    #[serde(skip_serializing, default)]
    previously: IgnoredAny,
}

/// information about the GSI info provider (CS:GO itself)
#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Provider {
    /// game name
    pub name: String,
    /// steam app ID
    #[serde(rename = "appid")]
    pub app_id: u64,
    /// version number
    pub version: u64,
    /// player's steam ID
    #[serde(rename = "steamid")]
    pub steam_id: String,
    /// update timestamp
    pub timestamp: u64,
}
