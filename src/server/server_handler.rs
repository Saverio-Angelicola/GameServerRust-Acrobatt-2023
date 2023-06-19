use ws::Handshake;
use ws::{CloseCode, Handler, Message, Result};

use super::server_config::ServerConfig;

// Gère les events d'un flux websocket

impl Handler for ServerConfig {
    fn on_open(&mut self, shake: Handshake) -> Result<()> {
        return self.handle_request(&shake.request);
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        return self.execute(msg);
    }

    fn on_close(&mut self, _code: CloseCode, _reason: &str) {
        self.leave();
    }

    fn on_shutdown(&mut self) {
        self.leave();
    }
}
