use serde::{Deserialize, Serialize};

use crate::shared::json::JsonFmt;

use crate::core::game::player::player_init::Position;

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct UpdatePositionRequest {
    pub position: Position,
}

impl JsonFmt for UpdatePositionRequest {
    fn from_json(json: &String) -> Self {
        return serde_json::from_str(&json).unwrap();
    }
}
