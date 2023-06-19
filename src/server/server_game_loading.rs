use std::{collections::HashMap, fs::File, io::Read};

use serde_json::Value;

use crate::core::game::{
    self,
    game_config::{Flag, Game, Team},
    items::Item,
    player::player_init::Position,
};

use super::server_constants::{
    GAME_CONFIG_FILE_UNIX, GAME_CONFIG_FILE_WINDOWS, MAP_CONFIG_FILE_UNIX, MAP_CONFIG_FILE_WINDOWS,
};
pub fn load_configuration(game_config: &mut String, map_config: &mut String) {
    let mut game_configuration_filename: &str = GAME_CONFIG_FILE_UNIX;
    let mut map_config_filename: &str = MAP_CONFIG_FILE_UNIX;

    // Utilise un path différent quand on est sur windows en mode release
    if cfg!(windows) && cfg!(release) {
        game_configuration_filename = GAME_CONFIG_FILE_WINDOWS;
        map_config_filename = MAP_CONFIG_FILE_WINDOWS;
    }

    //Récupère le contenu de la configuration de la partie

    File::open(game_configuration_filename)
        .unwrap()
        .read_to_string(game_config)
        .unwrap();

    File::open(map_config_filename)
        .unwrap()
        .read_to_string(map_config)
        .unwrap();
}

pub fn extract_configuration(game_config: &String, map_config: &String) -> Game {
    let game: Value = serde_json::from_str(&game_config).unwrap();
    let map: Value = serde_json::from_str(&map_config).unwrap();

    let duration = game["Duration"].as_i64().unwrap();
    //let game_name = game["gameName"].as_str().unwrap();
    //let game_mode = game["gameMode"].as_u64().unwrap();
    //let max_flags = game["maxFlags"].as_u64().unwrap();
    //let max_players = game["maxPlayers"].as_u64().unwrap();
    let teams_value = game["Teams"].as_array().unwrap();

    let elements = map["features"].as_array().unwrap();
    let mut flags: Vec<Flag> = Vec::new();
    let mut teams: HashMap<u64, Team> = HashMap::new();
    let mut items: Vec<Item> = Vec::new();

    for item in elements {
        let properties = item["properties"].as_object().unwrap();
        let geometry = item["geometry"].as_object().unwrap();
        let coordinates = geometry.get("coordinates").unwrap().as_array().unwrap();

        if properties.contains_key("typeId") {
            let point_type = properties.get("typeId").unwrap().as_u64().unwrap();
            if point_type == 1 {
                let flag = Flag {
                    id: item["id"].as_str().unwrap().to_string(),
                    is_captured: false,
                    player_id: 0,
                    team_id: 0,
                    time: 30,
                    position: Position {
                        x: coordinates[0].as_f64().unwrap(),
                        y: coordinates[1].as_f64().unwrap(),
                    },
                    action_radius: properties["actionRadius"].as_u64().unwrap(),
                    visibility_radius: properties["visibilityRadius"].as_u64().unwrap(),
                };
                flags.push(flag);
            }

            if point_type == 2 {
                let item: Item = Item {
                    id: item["id"].as_str().unwrap().to_string(),
                    item_type: properties["itemId"].as_u64().unwrap(),
                    position: Position {
                        x: coordinates[0].as_f64().unwrap(),
                        y: coordinates[1].as_f64().unwrap(),
                    },
                    name: properties["itemName"].as_str().unwrap().to_string(),
                    effect_duration: properties["effectDuration"].as_u64().unwrap(),
                    action_radius: properties["actionRadius"].as_u64().unwrap(),
                    visibility_radius: properties["visibilityRadius"].as_u64().unwrap(),
                    description: properties["itemDescription"].as_str().unwrap().to_string(),
                };
                items.push(item)
            }
        }
    }

    for value in teams_value {
        let team = Team {
            id: value["Id"].as_u64().unwrap(),
            players: Vec::new(),
            color: value["Color"].as_str().unwrap().to_string(),
            nb_players: value["NbPlayer"].as_u64().unwrap(),
            score: 0,
            name: value["Name"].as_str().unwrap().to_string()
        };

        teams.entry(team.id).or_insert(team);
    }

    return Game {
        flags,
        duration,
        game_mode: game::game_config::GameMode::CAPTURE,
        is_loading: false,
        teams,
        items,
        map: map_config.clone(),
        traps: Vec::new()
    };
}
