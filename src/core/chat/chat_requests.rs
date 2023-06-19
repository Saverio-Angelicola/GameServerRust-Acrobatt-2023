use serde::{Deserialize, Serialize};

use crate::shared::json::JsonFmt;

#[derive(Serialize, Deserialize, Clone)]
pub struct ChatRequest {
    pub msg: String,
}

impl JsonFmt for ChatRequest {
    fn from_json(json: &String) -> Self {
        return serde_json::from_str(&json).expect("parsing json failed!");
    }
}
