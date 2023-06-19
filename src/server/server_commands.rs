use actix_web::ResponseError;

use crate::core::chat::chat_requests::ChatRequest;
use crate::core::game::game_request::{
    AddFlagRequest, AddItemRequest, AddPointInterestRequest, CaptureFlagRequest,
    DeletePointInterestRequest, MineExplodeRequest, RemoveFlagRequest, RemoveItemRequest,
    UseItemRequest,
};
use crate::core::game::game_response::{
    BrouilleurEnableResponse, DeletePointInterestResponse, MineEnableResponse,
};
use crate::core::game::items::{DropItemRequest, GetItemRequest};
use crate::core::game::player::coordinate_checker::{
    check_client_coord, check_flag_coord, check_item_coord, check_trap_coord,
};
use crate::core::game::player::player_init::Player;
use crate::core::game::player::player_request::UpdatePositionRequest;
use crate::core::game::player::player_response::{CheckCoordResponse, PositionResponse};
use crate::{
    core::{
        chat::chat_sender::send_message_to_chat,
        game::{
            game_launcher::game_launch,
            game_request::{GameTriggerRequest, JoinTeamRequest},
        },
    },
    shared::json::JsonFmt,
};

use super::server_config::Client;
use super::server_error::ErrorResponse;
use super::server_response::ServerResponse;
use super::{server_config::ServerConfig, server_requests::ServerRequest};

// Gestion des commands

pub fn init_commands(server: &mut ServerConfig, request: &ServerRequest) {
    let command: &str = &request.command.as_str();
    let data: &String = &request.data.to_string();

    match server.game.try_lock() {
        Ok(mut game) => {
            match command {
                "update-position" => {
                    let position_request = UpdatePositionRequest::from_json(&data);
                    match server.clients.try_lock() {
                        Ok(mut clients) => {
                            let mut clients_filtered: Vec<&mut Client> = clients
                                .iter_mut()
                                .filter(|c| c.id == server.current_client.id)
                                .collect();

                            let client = clients_filtered.first_mut().unwrap();

                            client.position = position_request.position;

                            let position_response = PositionResponse {
                                position: position_request.position,
                                client_id: server.current_client.id,
                                pseudo: server.current_client.pseudo.clone(),
                            };

                            let server_response = ServerResponse::new(
                                &command,
                                serde_json::to_value(position_response).unwrap(),
                            )
                            .to_json();

                            server.out.broadcast(server_response).unwrap();

                            if game.is_loading == true {
                                let mut check_coord_response = CheckCoordResponse {
                                    points_zone: Vec::new(),
                                    player_zone: Vec::new(),
                                    trap_zone: Vec::new(),
                                };
                                game.flags.iter().for_each(|f| {
                                    check_coord_response
                                        .points_zone
                                        .push(check_flag_coord(f, position_request.position))
                                });

                                game.items.iter().for_each(|i| {
                                    check_coord_response
                                        .points_zone
                                        .push(check_item_coord(i, position_request.position))
                                });

                                game.traps.iter().for_each(|t| {
                                    check_coord_response
                                        .trap_zone
                                        .push(check_trap_coord(t, position_request.position))
                                });

                                clients.iter().for_each(|c| {
                                    if c.id != server.current_client.id
                                        && c.position.x != 0.0
                                        && c.position.y != 0.0
                                    {
                                        check_coord_response
                                            .player_zone
                                            .push(check_client_coord(c, position_request.position))
                                    }
                                });

                                let response = ServerResponse {
                                    command: String::from("check-coords"),
                                    data: serde_json::to_value(check_coord_response).unwrap(),
                                }
                                .to_json();
                                server.out.send(response).unwrap();
                            }
                        }
                        Err(_) => {
                            println!("ressource already lock");
                        }
                    }
                }
                "join-team" => {
                    let join_team_request = JoinTeamRequest::from_json(&data);
                    match game.join_team(join_team_request.team_id, &server.current_client) {
                        Some(join_team_response) => {
                            let server_response = ServerResponse::new(
                                &command,
                                serde_json::to_value(join_team_response).unwrap(),
                            )
                            .to_json();
                            server.out.broadcast(server_response).unwrap();
                        }
                        None => {
                            server
                                .out
                                .broadcast(ErrorResponse::error_message(404, "Team not found!"))
                                .unwrap();
                        }
                    }
                }
                "game-launcher" => {
                    let game_trigger = GameTriggerRequest::from_json(&data);
                    game_launch(game_trigger, &mut game, server);
                }
                "chat" => {
                    let chat_message = ChatRequest::from_json(&data);
                    let chat_response =
                        send_message_to_chat(&chat_message, &server.current_client.pseudo);
                    let server_response =
                        ServerResponse::new(&command, serde_json::to_value(chat_response).unwrap())
                            .to_json();
                    server.out.broadcast(server_response).unwrap();
                }
                "load-map" => {
                    // Renvoi le geojson de la map
                    let server_response = ServerResponse::new(
                        &command,
                        serde_json::to_value(server.map_config_file.clone()).unwrap(),
                    )
                    .to_json();
                    server.out.send(server_response).unwrap();
                }
                "get-item" => {
                    let server_request = GetItemRequest::from_json(&data);
                    let get_item_response =
                        game.set_item_in_inventory(server_request, server.current_client.id);
                    if get_item_response.is_some() {
                        let server_response = ServerResponse {
                            command: String::from("get-item"),
                            data: serde_json::to_value(get_item_response).unwrap(),
                        }
                        .to_json();
                        server.out.broadcast(server_response).unwrap();
                    } else {
                        server
                            .out
                            .send(ErrorResponse::error_message(500, "Item not found"))
                            .unwrap();
                    }
                }

                "drop-item" => {
                    let server_request = DropItemRequest::from_json(&data);
                    let drop_item_response =
                        game.drop_item(server_request, server.current_client.id);

                    if (drop_item_response.is_some()) {
                        let server_response = ServerResponse {
                            command: String::from("drop-item"),
                            data: serde_json::to_value(drop_item_response.unwrap()).unwrap(),
                        }
                        .to_json();

                        server.out.broadcast(server_response).unwrap();
                    } else {
                        server
                            .out
                            .broadcast(ErrorResponse::error_message(
                                500,
                                "item not found in inventory",
                            ))
                            .unwrap();
                    }
                }
                "capture-flag" => {
                    let server_request = CaptureFlagRequest::from_json(&data);
                    match game.capture_flag(
                        &server_request,
                        server.current_client.id,
                        &server.tx_capture_flag,
                    ) {
                        Ok(res) => {
                            let capture_flag_response = serde_json::to_value(res);

                            let mut server_response = ServerResponse {
                                command: "capture-flag".to_owned(),
                                data: capture_flag_response.unwrap(),
                            };
                            server.out.broadcast(server_response.to_json()).unwrap();
                        }
                        Err(err) => {
                            server.out.send(err).unwrap();
                        }
                    }
                }
                "remove-flag" => {
                    if server.current_client.pseudo == "master" && game.is_loading == true {
                        let server_request = RemoveFlagRequest::from_json(&data);
                        let response = game.remove_flag(server_request);
                        let server_response = ServerResponse {
                            command: String::from("remove-flag"),
                            data: serde_json::to_value(response).unwrap(),
                        }
                        .to_json();
                        server.out.broadcast(server_response).unwrap();
                    } else {
                        server
                            .out
                            .send(ErrorResponse::error_message(401, "Unauthorized"))
                            .unwrap();
                    }
                }
                "add-flag" => {
                    if server.current_client.pseudo == "master" && game.is_loading == true {
                        let server_request = AddFlagRequest::from_json(&data);
                        let response = game.add_flag(server_request);
                        let server_response = ServerResponse {
                            command: String::from("add-flag"),
                            data: serde_json::to_value(response).unwrap(),
                        }
                        .to_json();
                        server.out.broadcast(server_response).unwrap();
                    } else {
                        server
                            .out
                            .send(ErrorResponse::error_message(401, "Unauthorized"))
                            .unwrap();
                    }
                }
                "remove-item" => {
                    if server.current_client.pseudo == "master" && game.is_loading == true {
                        let server_request = RemoveItemRequest::from_json(&data);
                        let response = game.remove_item(server_request);
                        let server_response = ServerResponse {
                            command: String::from("remove-item"),
                            data: serde_json::to_value(response).unwrap(),
                        }
                        .to_json();
                        server.out.broadcast(server_response).unwrap();
                    } else {
                        server
                            .out
                            .send(ErrorResponse::error_message(401, "Unauthorized"))
                            .unwrap();
                    }
                }
                "add-item" => {
                    if server.current_client.pseudo == "master" && game.is_loading == true {
                        let server_request = AddItemRequest::from_json(&data);
                        let response = game.add_item(server_request);
                        let server_response = ServerResponse {
                            command: String::from("add-item"),
                            data: serde_json::to_value(response).unwrap(),
                        }
                        .to_json();
                        server.out.broadcast(server_response).unwrap();
                    } else {
                        server
                            .out
                            .send(ErrorResponse::error_message(401, "Unauthorized"))
                            .unwrap();
                    }
                }
                "set-point-interest" => {
                    let server_request = AddPointInterestRequest::from_json(&data);

                    match game.add_point_interest(server_request, server.current_client.id.clone())
                    {
                        Some(response) => {
                            let server_response = ServerResponse::new(
                                command,
                                serde_json::to_value(response).unwrap(),
                            )
                            .to_json();
                            server.out.broadcast(server_response).unwrap();
                        }
                        None => {}
                    }
                }
                "use-item" => {
                    let server_request = UseItemRequest::from_json(&data);
                    let use_item_response = game.use_item(
                        server_request,
                        server.current_client.id.clone(),
                        &server.tx_use_item,
                    );
                    match use_item_response{
                        Some(res) => {
                            let mut response = ServerResponse::new(command,res);
                            let is_mine: Result<MineEnableResponse, serde_json::Error> =
                            serde_json::from_value(response.data.clone());
    
                        let is_brouilleur: Result<BrouilleurEnableResponse, serde_json::Error> =
                            serde_json::from_value(response.data.clone());
    
                        if is_mine.is_ok() || is_brouilleur.is_ok() {
                            server.out.broadcast(response.to_json()).unwrap();
                        } else {
                            server.out.send(response.to_json()).unwrap();
                        }
                        },
                        None => {
                            server.out.send(ErrorResponse::error_message(500, "Item not found")).unwrap();
                        },
                    }
                    
                }
                "mine-explode" => {
                    let server_request = MineExplodeRequest::from_json(&data);
                    let mine_index = game
                        .traps
                        .iter()
                        .position(|m| m.item.id == server_request.mine_id);
                    if mine_index.is_some() {
                        let mine = game.traps.get(mine_index.unwrap()).unwrap().clone();
                        server.tx_use_item.send(mine).unwrap();
                        game.traps.remove(mine_index.unwrap());
                        let server_response = ServerResponse {
                            command: command.to_string(),
                            data: serde_json::to_value(server_request.clone()).unwrap(),
                        }
                        .to_json();
                        server.out.broadcast(server_response).unwrap();
                    } else {
                        server
                            .out
                            .send(ErrorResponse::error_message(
                                500,
                                "
                        The trap is not set",
                            ))
                            .unwrap();
                    }
                }

                _ => {
                    let message = format!("Unknown command: {}", command);
                    server
                        .out
                        .send(ErrorResponse::error_message(500, message.as_str()))
                        .unwrap();
                }
            }
        }
        Err(_) => {
            server
                .out
                .send(ErrorResponse::error_message(401, "Unauthorized"))
                .unwrap();
        }
    };
}
