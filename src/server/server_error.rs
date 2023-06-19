use serde::{Deserialize, Serialize};

// Structure générique pour les messages d'erreurs

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    code: u32,
    msg: String,
}

impl ErrorResponse {
    pub fn error_message(code: u32, msg: &str) -> String {
        return serde_json::to_value(ErrorResponse {
            code: code,
            msg: String::from(msg),
        })
        .unwrap()
        .to_string();
    }
}
