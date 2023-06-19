use ws::CloseCode;
use ws::Message;
use ws::Request;
use ws::Result;

use crate::core::game::game_response::LeavePlayerResponse;
use crate::core::game::game_response::NewPlayerResponse;
use crate::core::game::player::player_init::Player;
use crate::core::game::player::player_init::Position;

use super::server_commands::init_commands;
use super::server_config::Client;
use super::server_config::ServerConfig;
use super::server_error::ErrorResponse;
use super::server_requests::ServerRequest;
use super::server_response::ConnectionResponse;
use super::server_response::ServerResponse;

impl ServerConfig {
    pub fn handle_request(&mut self, request: &Request) -> Result<()> {
        // Récupère le pseudo à partir de l'URL du serveur
        let pseudo: Vec<&str> = request.resource().splitn(2, '/').collect();

        let mut clients = self.clients.try_lock().unwrap();

        if pseudo[1].len() <= 2 {
            self.out.close(CloseCode::Error).unwrap();
            return Ok(());
        }

        let mut next_id = self.next_id.try_lock().unwrap();

        // Ajoute le nouveau client

        let client = Client {
            id: *next_id,
            pseudo: pseudo[1].to_owned(),
            position: Position { x: 0.0, y: 0.0 },
        };

        self.current_client = client.clone();

        clients.push(client);

        // Mise à jour l'état de l'incrément
        *next_id = *next_id + 1;

        // Alerte tous les clients qu'un nouveau joueur à rejoint la game
        let mut response: ServerResponse = ServerResponse {
            command: "new-player".to_owned(),
            data: serde_json::to_value(NewPlayerResponse {
                player_id: self.current_client.id,
                pseudo: self.current_client.pseudo.clone(),
            })
            .expect("Serialization failed"),
        };
        // Envoie l'état actuelle de la partie au client
        let game = self.game.try_lock().expect("lock game failed").clone();
        let res = serde_json::to_string(&ServerResponse {
            command: "connection".to_owned(),
            data: serde_json::to_value(&ConnectionResponse {
                game,
                clients: clients.to_vec(),
            })
            .expect("Serialization failed"),
        })
        .expect("Serialization failed");
        self.out.send(res).expect("send failed");
        self.out
            .broadcast(response.to_json())
            .expect("broadcast failed");

        return Ok(());
    }

    pub fn execute(&mut self, msg: Message) -> Result<()> {
        match msg {
            Message::Text(text) => {
                // Parse le message en json
                let request_result: serde_json::Result<ServerRequest> = serde_json::from_str(&text);

                // Traite le résultat du parsing
                match request_result {
                    Ok(_) => init_commands(self, &request_result.expect("Parse json failed")),
                    Err(_) => {
                        self.out
                            .send(ErrorResponse::error_message(500, "Request not valid!"))
                            .expect("send failed");
                    }
                }
            }
            _ => {
                self.out.broadcast(msg).expect("broadcast failed");
            }
        }
        return Ok(());
    }

    pub fn leave(&mut self) {
        // Alerte les clients qu'un joueur s'est déconnecté

        let mut response: ServerResponse = ServerResponse {
            command: "player-leave".to_owned(),
            data: serde_json::to_value(LeavePlayerResponse {
                player_id: self.current_client.id,
                pseudo: self.current_client.pseudo.to_owned(),
            })
            .unwrap(),
        };

        match self.clients.try_lock() {
            Ok(mut clients) => {
                let index = clients
                    .iter()
                    .position(|c| c.id == self.current_client.id)
                    .unwrap();
                clients.remove(index);
            }
            Err(_) => {
                println!("clients lock")
            }
        }

        loop {
            match self.game.try_lock() {
                Ok(mut game) => {
                    for mut ele in &mut game.teams {
                        let team_players = ele.1.players.clone();
                        let players: Vec<&Player> = team_players
                            .iter()
                            .filter(|player| player.id != self.current_client.id)
                            .collect();
                        let players_clone: Vec<Player> = players
                            .iter()
                            .map(|player| Player {
                                id: player.id,
                                pseudo: player.pseudo.to_owned(),
                                score: player.score,
                                inventory: player.inventory.clone(),
                            })
                            .collect();
                        ele.1.players = players_clone.clone();
                    }

                    break;
                }
                Err(_) => {}
            }
        }

        self.out.broadcast(response.to_json()).unwrap();
    }
}
