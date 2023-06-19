use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::core::game::game_config::Game;

use super::server_config::Client;

// Structure de la réponse lorsqu'on se connecte au serveur

#[derive(Serialize, Deserialize)]
pub struct ConnectionResponse {
    pub game: Game,
    pub clients: Vec<Client>,
}

// Structure des réponses pour les commandes

#[derive(Serialize, Deserialize, Clone)]
pub struct ServerResponse {
    pub command: String,
    pub data: Value,
}

impl ServerResponse {
    pub fn new(command: &str, data: Value) -> Self {
        return ServerResponse {
            command: command.to_string(),
            data,
        };
    }

    pub fn to_json(&mut self) -> String {
        return serde_json::to_string(self).expect("serialisation failed");
    }
}
