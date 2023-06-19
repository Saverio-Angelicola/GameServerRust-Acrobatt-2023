use serde::{Deserialize, Serialize};

use crate::core::game::items::Item;

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Player {
    pub id: u32,
    pub pseudo: String,
    pub score: u32,
    pub inventory: Vec<Item>,
}
