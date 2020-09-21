const SCRIPT: &'static str = r##"
fn print_recursive(label, data, indent) {
    let deeper_indent = indent + "    ";
    if type_of(data) == "()" {
        print(indent + label + ": ()")
    } else if type_of(data) in ["u64", "i64", "string", "bool", "csgo_gsi::update::map::Mode", "csgo_gsi::update::map::Phase", "csgo_gsi::update::player::Activity", "csgo_gsi::update::player::WeaponType", "csgo_gsi::update::player::WeaponState", "csgo_gsi::update::round::Phase", "csgo_gsi::update::round::BombState", "csgo_gsi::update::Team"] {
        print(indent + label + ": " + data);
    } else if type_of(data) == "map" {
        print(indent + label + ": #{");
        for name in keys(data) {
            print_recursive(name, data[name], deeper_indent);
        }
        print(indent + "}");
    } else if type_of(data) == "csgo_gsi::update::Update" {
        print(indent + label + ":");
        print_recursive("Map", data.map, deeper_indent);
        print_recursive("Player", data.player, deeper_indent);
        print_recursive("Provider", data.provider, deeper_indent);
        print_recursive("Round", data.round, deeper_indent);
    } else if type_of(data) == "csgo_gsi::update::map::Map" {
        print(indent + label + ":");
        print_recursive("Current Spectators", data.current_spectators, deeper_indent);
        print_recursive("Mode", data.mode, deeper_indent);
        print_recursive("Name", data.name, deeper_indent);
        print_recursive("# Matches to Win Series", data.num_matches_to_win_series, deeper_indent);
        print_recursive("Phase", data.phase, deeper_indent);
        print_recursive("Round", data.round, deeper_indent);
        print_recursive("Round Wins", data.round_wins, deeper_indent);
        print_recursive("Souvenirs (Total)", data.souvenirs_total, deeper_indent);
        print_recursive("CT Team", data.team_ct, deeper_indent);
        print_recursive("T Team", data.team_t, deeper_indent);
    } else if type_of(data) == "csgo_gsi::update::map::Team" {
        print(indent + label + ":");
        print_recursive("Score", data.score, deeper_indent);
        print_recursive("Consecutive Round Losses", data.consecutive_round_losses, deeper_indent);
        print_recursive("Timeouts Remaining", data.timeouts_remaining, deeper_indent);
        print_recursive("Matches Won This Series", data.matches_won_this_series, deeper_indent);
        print_recursive("Name", data.name, deeper_indent);
        print_recursive("Flag", data.flag, deeper_indent);
    } else if type_of(data) == "csgo_gsi::update::player::Player" {
        print(indent + label + ":");
        print_recursive("Steam ID", data.steam_id, deeper_indent);
        print_recursive("Name", data.name, deeper_indent);
        print_recursive("Observer Slot", data.observer_slot, deeper_indent);
        print_recursive("Activity", data.activity, deeper_indent);
        print_recursive("Match Stats", data.match_stats, deeper_indent);
        print_recursive("State", data.state, deeper_indent);
        print_recursive("Team", data.team, deeper_indent);
        print_recursive("Weapons", data.weapons, deeper_indent);
        print_recursive("Clan", data.clan, deeper_indent);
    } else if type_of(data) == "csgo_gsi::update::player::MatchStats" {
        print(indent + label + ":");
        print_recursive("Kills", data.kills, deeper_indent);
        print_recursive("Assists", data.assists, deeper_indent);
        print_recursive("Deaths", data.deaths, deeper_indent);
        print_recursive("MVPs", data.mvps, deeper_indent);
        print_recursive("Score", data.score, deeper_indent);
    } else if type_of(data) == "csgo_gsi::update::player::State" {
        print(indent + label + ":");
        print_recursive("Health", data.health, deeper_indent);
        print_recursive("Armor", data.armor, deeper_indent);
        print_recursive("Helmet", data.helmet, deeper_indent);
        print_recursive("Flashed", data.flashed, deeper_indent);
        print_recursive("Smoked", data.smoked, deeper_indent);
        print_recursive("Burning", data.burning, deeper_indent);
        print_recursive("Money", data.money, deeper_indent);
        print_recursive("Round Kills", data.round_kills, deeper_indent);
        print_recursive("Round (Headshot?) Kills", data.round_killhs, deeper_indent);
        print_recursive("Equipment Value", data.equip_value, deeper_indent);
        print_recursive("Total Damage (Dealt?) This Round", data.round_totaldmg, deeper_indent);
        print_recursive("Defuse Kit", data.defuse_kit, deeper_indent);
    } else if type_of(data) == "csgo_gsi::update::player::Weapon" {
        print(indent + label + ":");
        print_recursive("Name", data.name, deeper_indent);
        print_recursive("Skin", data.paintkit, deeper_indent);
        print_recursive("Type", data.type, deeper_indent);
        print_recursive("State", data.state, deeper_indent);
        print_recursive("Current Bullets", data.ammo_clip, deeper_indent);
        print_recursive("Bullets Per Clip", data.ammo_clip_max, deeper_indent);
        print_recursive("Bullets In Reserve", data.ammo_reserve, deeper_indent);
    } else if type_of(data) == "csgo_gsi::update::Provider" {
        print(indent + label + ":");
        print_recursive("Name", data.name, deeper_indent);
        print_recursive("App ID", data.app_id, deeper_indent);
        print_recursive("Version", data.version, deeper_indent);
        print_recursive("Steam ID", data.steam_id, deeper_indent);
        print_recursive("Timestamp", data.timestamp, deeper_indent);
    } else if type_of(data) == "csgo_gsi::update::round::Round" {
        print(indent + label + ":");
        print_recursive("Phase", data.phase, deeper_indent);
        print_recursive("Bomb", data.bomb, deeper_indent);
        print_recursive("Win Team", data.win_team, deeper_indent);
    } else {
        let data_type = type_of(data);
        print(indent + label + ": unknown type " + data_type);
        throw "Unknown type " + data_type;
    }
}

fn handle_update(update) {
    print_recursive("Update", update, "");
    return;
}
"##;

use csgo_gsi::{GSIConfigBuilder, GSIServer, Subscription};
use csgo_gsi::update::{Update, CSGOPackage};
use rhai::{AST, Engine, packages::Package, Scope};

pub struct ScriptHost {
    engine: Engine,
    scope: Scope<'static>,
    ast: AST,
}

impl ScriptHost {
    pub fn new() -> Self {
        let mut engine = Engine::new();
        engine.load_package(CSGOPackage::new().get());
        let mut scope = Scope::new();

        let ast = engine.compile_with_scope(&mut scope, SCRIPT)
            .map_err(|x| x.to_string())
            .expect("Couldn't compile script");
        // if there's some global state or on-boot handling, make sure it runs
        engine.consume_ast_with_scope(&mut scope, &ast)
            .expect("Couldn't run script");
        Self {
            engine,
            scope,
            ast,
        }
    }

    pub fn handle_update(&mut self, update: &Update) {
        let result = self.engine.call_fn::<(Update,), ()>(&mut self.scope, &mut self.ast, "handle_update", (update.clone(),));
        if let Err(e) = result {
            eprintln!("Error when handling update: {}", e);
        };
    }
}

#[tokio::main]
async fn main() {
    let config = GSIConfigBuilder::new("csgo-gsi Example")
        .subscribe_multiple(Subscription::UNRESTRICTED)
        .build();

    let mut host = ScriptHost::new();
    let mut server = GSIServer::new(config, 31337);
    server.add_listener(move |update| host.handle_update(update));

    server
        .run()
        .await
        .expect("server didn't start");
}
