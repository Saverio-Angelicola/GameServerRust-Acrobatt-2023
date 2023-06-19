use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

use crate::shared::json::JsonFmt;

use super::player::player_init::Position;

#[derive(Serialize, Deserialize, Clone)]
pub struct Item {
    pub id: String,
    pub item_type: u64,
    pub position: Position,
    pub name: String,
    pub effect_duration: u64,
    pub action_radius: u64,
    pub visibility_radius: u64,
    pub description: String
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ItemInLoading{
    pub item: Item,
    pub client_id: u32,
    pub team_id: u64
}

#[derive(Debug, PartialEq, EnumIter)]
pub enum ItemType {
    Satellite,
    Extension,
    Mine,
    Brouilleur,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GetItemRequest {
    pub item_id: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GetItemResponse {
    pub item_id: String,
}

impl JsonFmt for GetItemRequest {
    fn from_json(json: &String) -> Self
    where
        Self: Sized,
    {
        return serde_json::from_str(&json).unwrap();
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DropItemRequest {
    pub item_id: String,
    pub position: Position,
}

impl JsonFmt for DropItemRequest {
    fn from_json(json: &String) -> Self
    where
        Self: Sized,
    {
        return serde_json::from_str(&json).unwrap();
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DropItemResponse {
    pub item: Item,
    pub position: Position,
}
