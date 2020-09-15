use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Player {
    #[serde(rename = "steamid")]
    steam_id: String,
    name: String,
    observer_slot: Option<u64>,
    activity: Activity,
    match_stats: Option<MatchStats>,
    state: Option<State>,
    team: Option<super::Team>,
    #[serde(default)]
    weapons: HashMap<String, Weapon>,
    clan: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Activity {
    Menu,
    Playing,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MatchStats {
    kills: i64,
    assists: u64,
    deaths: u64,
    mvps: u64,
    score: u64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct State {
    health: u64,
    armor: u64,
    helmet: bool,
    flashed: u64,
    smoked: u64,
    burning: u64,
    money: u64,
    round_kills: i64,
    round_killhs: u64,
    equip_value: u64,
    round_totaldmg: Option<u64>,
    #[serde(rename = "defusekit")]
    defuse_kit: Option<bool>
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Weapon {
    name: String,
    paintkit: String,
    r#type: Option<WeaponType>, // TODO is this ever missing for anything other than the taser
    state: WeaponState,
    ammo_clip: Option<u64>,
    ammo_clip_max: Option<u64>,
    ammo_reserve: Option<u64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum WeaponType {
    Knife,
    Pistol,
    #[serde(rename = "Submachine Gun")]
    SMG,
    #[serde(rename = "Machine Gun")]
    MachineGun,
    Rifle,
    SniperRifle,
    Shotgun,
    StackableItem,
    Grenade,
    C4,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum WeaponState {
    Holstered,
    Active,
    Reloading,
}
