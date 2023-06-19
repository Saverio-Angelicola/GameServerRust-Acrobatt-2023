use serde::{Deserialize, Serialize};

use crate::shared::json::JsonFmt;

use super::{game_config::Flag, items::Item, player::player_init::Position};

// DTOs

#[derive(Serialize, Deserialize, Clone)]
pub struct JoinTeamRequest {
    pub team_id: u32,
}

impl JsonFmt for JoinTeamRequest {
    fn from_json(json: &String) -> Self {
        return serde_json::from_str(&json).unwrap();
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CaptureFlagRequest {
    pub flag_id: String,
}

impl JsonFmt for CaptureFlagRequest {
    fn from_json(json: &String) -> Self
    where
        Self: Sized,
    {
        return serde_json::from_str(&json).unwrap();
    }
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct GameTriggerRequest {
    pub trigger: bool,
}

impl JsonFmt for GameTriggerRequest {
    fn from_json(json: &String) -> Self {
        return serde_json::from_str(&json).unwrap();
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AddFlagRequest {
    pub flag: Flag,
}

impl JsonFmt for AddFlagRequest {
    fn from_json(json: &String) -> Self {
        return serde_json::from_str(&json).unwrap();
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RemoveFlagRequest {
    pub flag_id: String,
}

impl JsonFmt for RemoveFlagRequest {
    fn from_json(json: &String) -> Self {
        return serde_json::from_str(&json).unwrap();
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AddItemRequest {
    pub item: Item,
}

impl JsonFmt for AddItemRequest {
    fn from_json(json: &String) -> Self {
        return serde_json::from_str(&json).unwrap();
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RemoveItemRequest {
    pub item_id: String,
}

impl JsonFmt for RemoveItemRequest {
    fn from_json(json: &String) -> Self {
        return serde_json::from_str(&json).unwrap();
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AddPointInterestRequest
{
    pub point_id: String,
    pub position: Position
}

impl JsonFmt for AddPointInterestRequest {
    fn from_json(json: &String) -> Self {
        return serde_json::from_str(&json).unwrap();
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DeletePointInterestRequest {
    pub point_id: String
}

impl JsonFmt for DeletePointInterestRequest {
    fn from_json(json: &String) -> Self {
        return serde_json::from_str(&json).unwrap();
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UseItemRequest{
    pub item_id: String,
    pub position: Position
}

impl JsonFmt for UseItemRequest {
    fn from_json(json: &String) -> Self {
        return serde_json::from_str(&json).unwrap();
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MineExplodeRequest{
    pub mine_id: String
}

impl JsonFmt for MineExplodeRequest {
    fn from_json(json: &String) -> Self {
        return serde_json::from_str(&json).unwrap();
    }
}