use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{
    game_config::{Flag, Team},
    items::Item,
    player::player_init::{Player, Position},
};

#[derive(Serialize, Deserialize, Clone)]
pub struct JoinTeamResponse {
    pub message: String,
    pub team_id: u32,
    pub player_id: u32,
    pub pseudo: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct NewPlayerResponse {
    pub player_id: u32,
    pub pseudo: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LeavePlayerResponse {
    pub player_id: u32,
    pub pseudo: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GameLauncherResponse {
    pub game_loading: bool,
    pub message: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CaptureFlagResponse {
    pub player: Player,
    pub team: Team,
    pub flag_id: String,
    pub is_capture: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FlagFreeResponse {
    pub flag_id: String,
    pub is_capture: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ScoreResponse {
    pub winner: Vec<Team>,
    pub teams: HashMap<u64, Team>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AddFlagResponse {
    pub flag: Flag,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RemoveFlagResponse {
    pub flag_id: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AddItemResponse {
    pub item: Item,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RemoveItemResponse {
    pub item_id: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AddPointInterestResponse {
    pub point_id: String,
    pub player_id: u32,
    pub team_id: u64,
    pub position: Position,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DeletePointInterestResponse {
    pub point_id: String,
    pub team_id: u64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SatelliteEnableResponse {
    pub type_id: u64,
    pub enable: bool,
    pub time: u64,
    pub client_id: u32
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CombinaisonEnableResponse {
    pub type_id: u64,
    pub enable: bool,
    pub time: u64
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ExtensionEnableResponse {
    pub type_id: u64,
    pub enable: bool,
    pub additionnal_location: u32
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MineEnableResponse {
    pub type_id: u64,
    pub enable: bool,
    pub time: u64,
    pub position: Position,
    pub item: Item,
    pub team_id: u64
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BrouilleurEnableResponse {
    pub type_id: u64,
    pub enable: bool,
    pub time: u64,
    pub team_id: u64
}
