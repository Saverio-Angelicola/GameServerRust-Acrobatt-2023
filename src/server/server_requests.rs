use serde::{Deserialize, Serialize};
use serde_json::Value;

// Structure des requêtes de base envoyer par le client

#[derive(Serialize, Deserialize)]
pub struct ServerRequest {
    pub command: String,
    pub data: Value,
}
