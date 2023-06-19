use std::{
    sync::{mpsc::Receiver, Arc, Mutex},
    thread,
    time::Duration,
};

use strum::IntoEnumIterator;

use crate::server::server_response::ServerResponse;

use super::{
    game_config::{Flag, Game},
    game_response::{
        BrouilleurEnableResponse, CombinaisonEnableResponse, FlagFreeResponse,
        GameLauncherResponse, MineEnableResponse, SatelliteEnableResponse, ScoreResponse,
    },
    items::{ItemInLoading, ItemType},
};

// Initialisation du thread avec un timer pour la partie

pub fn init_game_loop(
    rx_game_launcher: Receiver<bool>,
    rx_websocket: Receiver<ws::Sender>,
    rx_capture_flag: Receiver<Flag>,
    rx_use_item: Receiver<ItemInLoading>,
    game: &Arc<Mutex<Game>>,
    initial_config: Game,
) {
    let mut duration = 600;
    match game.try_lock() {
        Ok(game) => duration = game.duration.clone(),
        Err(err) => {
            println!("{}", err.to_string());
        }
    }
    let mut ws_sender: Option<ws::Sender> = None;
    let game_thread = game.clone();

    thread::spawn(move || {
        let mut is_loading = false;
        let mut time = 0;
        let duration: i64 = duration;
        let mut flags_queue: Vec<Flag> = Vec::new();
        let mut items_queue: Vec<ItemInLoading> = Vec::new();
        loop {
            let result_sender = rx_websocket.try_recv();
            if result_sender.is_ok() {
                ws_sender = Some(result_sender.expect("get sender failed").to_owned());
            }

            let send = ws_sender.clone();

            if send.is_some() {
                let sender = send.unwrap();
                match rx_game_launcher.try_recv() {
                    Ok(state) => {
                        is_loading = state;
                    }
                    Err(_) => {
                        if is_loading == true {
                            if time <= duration {
                                time += 1;
                                thread::sleep(Duration::from_secs(1));
                            } else {
                                let mut server_response = ServerResponse {
                                    command: "game-launcher".to_owned(),
                                    data: serde_json::to_value(GameLauncherResponse {
                                        game_loading: false,
                                        message: "The game is over".to_owned(),
                                    })
                                    .expect("Serialization failed"),
                                };
                                sender
                                    .broadcast(server_response.to_json())
                                    .expect("Serialization failed");

                                match game_thread.try_lock() {
                                    Ok(mut game) => {
                                        game.is_loading = false;
                                        let mut score_response = ScoreResponse {
                                            winner: Vec::new(),
                                            teams: game.teams.clone(),
                                        };

                                        let highest_score = game
                                            .teams
                                            .iter()
                                            .max_by(|x, y| x.1.score.cmp(&y.1.score))
                                            .expect("find winner failed")
                                            .1
                                            .score
                                            .clone();

                                        score_response.winner = game
                                            .teams
                                            .iter()
                                            .filter(|w| w.1.score == highest_score)
                                            .map(|w| w.1.clone())
                                            .collect();

                                        let server_response = ServerResponse {
                                            command: String::from("score"),
                                            data: serde_json::to_value(score_response)
                                                .expect("Serialization failed"),
                                        }
                                        .to_json();
                                        sender.broadcast(server_response).unwrap();
                                        game.teams = initial_config.teams.clone();
                                        game.flags = initial_config.flags.clone();
                                        game.items = initial_config.items.clone();
                                    }
                                    Err(_) => {}
                                }

                                is_loading = false;
                            }
                        } else {
                            time = 0;
                        }
                    }
                }

                if is_loading {
                    match rx_capture_flag.try_recv() {
                        Ok(mut flag) => {
                            flag.time += time;
                            flags_queue.push(flag);
                        }
                        Err(_) => {}
                    }

                    let mut flags_queue_copy = flags_queue.clone();
                    let flags_finished: Vec<&mut Flag> = flags_queue_copy
                        .iter_mut()
                        .filter(|f| f.time - time == 0)
                        .collect();

                    let flags_left: Vec<Flag> = flags_queue
                        .clone()
                        .iter()
                        .filter(|f| f.time - time != 0)
                        .map(|f| f.clone())
                        .collect();

                    flags_queue = flags_left;

                    let mut waiting_flag_update: Vec<Flag> = Vec::new();

                    for flag in flags_finished {
                        match game_thread.try_lock() {
                            Ok(mut game) => {
                                let flag_index =
                                    game.flags.iter_mut().position(|f| f.id == flag.id);
                                if flag_index.is_some() {
                                    let mut game_flag =
                                        game.flags.get_mut(flag_index.unwrap()).unwrap();
                                    game_flag.is_captured = false;
                                    game_flag.player_id = 0;
                                    game_flag.team_id = 0;
                                }

                                if waiting_flag_update.len() > 0 {
                                    for waiting_flag in waiting_flag_update.iter_mut() {
                                        let flag_index = game
                                            .flags
                                            .iter_mut()
                                            .position(|f| f.id == waiting_flag.id);
                                        if flag_index.is_some() {
                                            let mut game_flag =
                                                game.flags.get_mut(flag_index.unwrap()).unwrap();
                                            game_flag.is_captured = false;
                                            game_flag.player_id = 0;
                                            game_flag.team_id = 0;
                                        }
                                    }
                                }
                            }
                            Err(_) => {
                                flag.is_captured = true;
                                waiting_flag_update.push(flag.clone());
                            }
                        }

                        let flag_free_res = serde_json::to_value(FlagFreeResponse {
                            flag_id: flag.id.clone(),
                            is_capture: false,
                        })
                        .expect("Serialization failed");

                        let mut server_response = ServerResponse {
                            command: "free-flag".to_owned(),
                            data: flag_free_res,
                        };

                        sender
                            .broadcast(server_response.to_json())
                            .expect("broadcast failed");
                    }

                    match rx_use_item.try_recv() {
                        Ok(mut item) => {
                            item.item.effect_duration += time as u64;
                            items_queue.push(item);
                        }
                        Err(_) => {}
                    }

                    let mut items_queue_copy = items_queue.clone();
                    let items_finished: Vec<&mut ItemInLoading> = items_queue_copy
                        .iter_mut()
                        .filter(|f| f.item.effect_duration - time as u64 == 0)
                        .collect();

                    let items_left: Vec<ItemInLoading> = items_queue
                        .clone()
                        .iter()
                        .filter(|f| f.item.effect_duration - time as u64 != 0)
                        .map(|f| f.clone())
                        .collect();

                    items_queue = items_left;

                    for item in items_finished {
                        let mut response = ServerResponse {
                            command: String::from("item-effect-finished"),
                            data: serde_json::Value::Null,
                        };

                        match ItemType::iter().nth(item.item.item_type as usize).unwrap() {
                            ItemType::Satellite => {
                                println!("item type {} finished", item.item.item_type);
                                response.data = serde_json::to_value(SatelliteEnableResponse {
                                    enable: false,
                                    type_id: item.item.item_type,
                                    time: 0,
                                    client_id: item.client_id,
                                })
                                .unwrap();
                                sender.broadcast(response.to_json()).unwrap();
                            }
                            ItemType::Extension => {}
                            ItemType::Mine => {
                                response.data = serde_json::to_value(MineEnableResponse {
                                    type_id: item.item.item_type,
                                    enable: false,
                                    time: 0,
                                    position: item.item.position,
                                    item: item.item.clone(),
                                    team_id: item.team_id,
                                })
                                .unwrap();

                                sender.broadcast(response.to_json()).unwrap();
                            }
                            ItemType::Brouilleur => {
                                response.data = serde_json::to_value(BrouilleurEnableResponse {
                                    type_id: item.item.item_type,
                                    enable: false,
                                    time: 0,
                                    team_id: item.team_id.clone(),
                                })
                                .unwrap();

                                sender.broadcast(response.to_json()).unwrap();
                            }
                        }
                    }
                }
            }
        }
    });
}
