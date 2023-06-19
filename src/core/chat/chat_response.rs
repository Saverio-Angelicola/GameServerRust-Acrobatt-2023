use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ChatResponse {
    pub pseudo: String,
    pub msg: String,
}
