use serde::Serialize;

use serde::Deserialize;

use super::player_init::Position;

#[derive(Serialize, Deserialize, Clone)]
pub struct PositionResponse {
    pub client_id: u32,
    pub position: Position,
    pub pseudo: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CheckCoordsPlayerResponse {
    pub client_id: u32,
    pub is_visible: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CheckPointCoordsResponse {
    pub point_id: String,
    pub visible: bool,
    pub capturable: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CheckTrapCoordsResponse {
    pub point_id: String,
    pub visible: bool,
    pub capturable: bool,
    pub team_id: u64
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CheckCoordResponse{
    pub points_zone: Vec<CheckPointCoordsResponse>,
    pub player_zone: Vec<CheckCoordsPlayerResponse>,
    pub trap_zone: Vec<CheckTrapCoordsResponse>
}
