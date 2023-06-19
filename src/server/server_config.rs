use std::sync::{mpsc::Sender, Arc, Mutex};

use serde::{Deserialize, Serialize};

use crate::core::game::{
    game_config::{Flag, Game},
    items::ItemInLoading,
    player::player_init::Position,
};

// Config et structure du serveur websocket

#[derive(Serialize, Deserialize, Clone)]
pub struct Client {
    pub id: u32,
    pub pseudo: String,
    pub position: Position,
}

impl Default for Client {
    fn default() -> Self {
        Self {
            id: Default::default(),
            pseudo: Default::default(),
            position: Position { x: 0.0, y: 0.0 },
        }
    }
}

pub struct ServerConfig {
    pub initial_config: Game,
    pub out: ws::Sender,
    pub game: Arc<Mutex<Game>>,
    pub clients: Arc<Mutex<Vec<Client>>>,
    pub current_client: Client,
    pub next_id: Arc<Mutex<u32>>,
    pub game_trigger: Sender<bool>,
    pub game_configuration_file: String,
    pub map_config_file: String,
    pub tx_capture_flag: Sender<Flag>,
    pub tx_use_item: Sender<ItemInLoading>,
}
