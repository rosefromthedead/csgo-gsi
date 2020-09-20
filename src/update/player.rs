//! player-related info

use std::collections::HashMap;

use serde::{Serialize, Deserialize};

/// player info
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Player {
    /// steam ID
    #[serde(rename = "steamid")]
    pub steam_id: String,
    /// display name
    pub name: String,
    /// observer slot number
    pub observer_slot: Option<u64>,
    /// current activity (in menu, playing game, etc)
    pub activity: Activity,
    /// match statistics
    pub match_stats: Option<MatchStats>,
    /// state (health, armor, etc)
    pub state: Option<State>,
    /// team
    pub team: Option<super::Team>,
    /// weapon inventory
    #[serde(default)]
    pub weapons: HashMap<String, Weapon>,
    /// clan
    pub clan: Option<String>,
}

/// an activity a player can be doing
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Activity {
    /// in a menu
    Menu,
    /// playing the game
    Playing,
}

/// a player's match statistics
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MatchStats {
    /// kills
    pub kills: i64,
    /// assists
    pub assists: u64,
    /// deaths
    pub deaths: u64,
    /// MVPs
    pub mvps: u64,
    /// score
    pub score: u64,
}

/// player state
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct State {
    /// health
    pub health: u64,
    /// armor
    pub armor: u64,
    /// has a helmet?
    pub helmet: bool,
    /// flashbang duration(?)
    pub flashed: u64,
    /// smoke duration(?)
    pub smoked: u64,
    /// on-fire duration(?)
    pub burning: u64,
    /// money
    pub money: u64,
    /// kills this round
    pub round_kills: i64,
    /// headshot(?) kills this round
    pub round_killhs: u64,
    /// current equipment value
    pub equip_value: u64,
    /// total damage dealt(?) this round
    pub round_totaldmg: Option<u64>,
    /// has a defuse kit?
    #[serde(rename = "defusekit")]
    pub defuse_kit: Option<bool>
}

/// weapon info
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Weapon {
    /// name
    pub name: String,
    /// skin
    pub paintkit: String,
    /// type (pistol, rifle, etc)
    pub r#type: Option<WeaponType>, // TODO is this ever missing for anything other than the taser
    /// state (holstered, active, etc)
    pub state: WeaponState,
    /// bullets in current clip
    pub ammo_clip: Option<u64>,
    /// bullets per clip
    pub ammo_clip_max: Option<u64>,
    /// bullets in reserve
    pub ammo_reserve: Option<u64>,
}

/// a type of weapon
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum WeaponType {
    /// knife
    Knife,
    /// pistol
    Pistol,
    /// submachine gun
    #[serde(rename = "Submachine Gun")]
    SMG,
    /// machine gun
    #[serde(rename = "Machine Gun")]
    MachineGun,
    /// regular rifle
    Rifle,
    /// sniper rifle
    SniperRifle,
    /// shotgun
    Shotgun,
    /// "stackable item" (health shot in deathmatch, other examples unknown)
    StackableItem,
    /// grenade
    Grenade,
    /// bomb
    C4,
}

/// status of weapon
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum WeaponState {
    /// not selected
    Holstered,
    /// selected
    Active,
    /// reloading
    Reloading,
}
