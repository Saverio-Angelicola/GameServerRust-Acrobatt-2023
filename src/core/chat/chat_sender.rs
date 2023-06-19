use serde_json::Value;

use super::{chat_requests::ChatRequest, chat_response::ChatResponse};

pub fn send_message_to_chat(chat_req: &ChatRequest, pseudo: &String) -> Value {
    let chat_response: ChatResponse = ChatResponse {
        pseudo: (&pseudo).to_string(),
        msg: chat_req.clone().msg,
    };

    return serde_json::to_value(chat_response).expect("Serialization failed");
}
