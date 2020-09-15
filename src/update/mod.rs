use std::collections::HashMap;

use serde::{Serialize, Deserialize, de::IgnoredAny};

mod player;
use player::Player;

mod map;
use map::Map;

mod round;
use round::Round;

// TODO abuse generics to align subscriptions with these types

#[derive(Debug, Deserialize, Serialize)]
pub enum Team {
    CT,
    T,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Update {
    map: Option<Map>,
    player: Option<Player>,
    provider: Option<Provider>,
    auth: HashMap<String, String>,
    round: Option<Round>,
    #[serde(skip_serializing, default)]
    added: IgnoredAny,
    #[serde(skip_serializing, default)]
    previously: IgnoredAny,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Provider {
    name: String,
    #[serde(rename = "appid")]
    app_id: u64,
    version: u64,
    #[serde(rename = "steamid")]
    steam_id: String,
    timestamp: u64,
}
