use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::core::game::player::player_init::Player;

use super::{items::{Item, ItemInLoading}, player::player_init::Position};

#[derive(Serialize, Deserialize, Clone)]
pub struct Flag {
    pub id: String,
    pub is_captured: bool,
    pub player_id: u32,
    pub team_id: u64,
    pub time: i64,
    pub position: Position,
    pub action_radius: u64,
    pub visibility_radius: u64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Team {
    pub id: u64,
    pub players: Vec<Player>,
    pub color: String,
    pub nb_players: u64,
    pub score: u32,
    pub name: String
}

#[derive(Serialize, Deserialize, Clone)]
pub enum GameMode {
    CAPTURE,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Game {
    pub flags: Vec<Flag>,
    pub duration: i64,
    pub game_mode: GameMode,
    pub is_loading: bool,
    pub teams: HashMap<u64, Team>,
    pub map: String,
    pub items: Vec<Item>,
    pub traps: Vec<ItemInLoading>
}
