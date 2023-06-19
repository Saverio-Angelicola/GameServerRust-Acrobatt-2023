use std::sync::mpsc::Sender;

use serde_json::Value;
use strum::IntoEnumIterator;

use crate::server::{server_config::Client, server_error::ErrorResponse};

use super::{
    game_config::{Flag, Game, Team},
    game_request::{
        AddFlagRequest, AddItemRequest, AddPointInterestRequest, CaptureFlagRequest,
        RemoveFlagRequest, RemoveItemRequest, UseItemRequest,
    },
    game_response::{
        AddFlagResponse, AddItemResponse, AddPointInterestResponse, BrouilleurEnableResponse,
        CaptureFlagResponse, CombinaisonEnableResponse, ExtensionEnableResponse, JoinTeamResponse,
        MineEnableResponse, RemoveFlagResponse, RemoveItemResponse, SatelliteEnableResponse,
    },
    items::{
        DropItemRequest, DropItemResponse, GetItemRequest, GetItemResponse, Item, ItemInLoading,
        ItemType,
    },
};
use crate::core::game::player::player_init::Player;

impl Game {
    pub fn play(&mut self, tx: &Sender<bool>) -> () {
        self.is_loading = true;
        // Envoie une donnée au thread via un channel de synchronisation
        tx.send(true).expect("play game failed");
    }

    pub fn stop(&mut self, tx: &Sender<bool>) -> () {
        self.is_loading = false;
        tx.send(false).expect("stop game failed");
    }

    pub fn join_team(&mut self, team_id: u32, client: &Client) -> Option<String> {
        for mut ele in &mut self.teams {
            let team_players = ele.1.players.clone();
            let players: Vec<&Player> = team_players
                .iter()
                .filter(|player| player.id != client.id)
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

        let team = self.teams.get_mut(&u64::from(team_id));

        if team.is_none() {
            return None;
        }

        let client_details: Client = client.clone();

        team.expect("team not found").players.push(Player {
            id: client_details.id,
            pseudo: client_details.pseudo,
            score: 0,
            inventory: Vec::new(),
        });

        let repsonse = JoinTeamResponse {
            message: format!("The player {} has joined a team", client.pseudo),
            player_id: client.id,
            team_id,
            pseudo: client.pseudo.clone(),
        };

        return Some(serde_json::to_string(&repsonse).expect("Serialization failed"));
    }

    pub fn capture_flag(
        &mut self,
        request: &CaptureFlagRequest,
        player_id: u32,
        tx_capture_flag: &Sender<Flag>,
    ) -> Result<CaptureFlagResponse, String> {
        let mut flag_vec: Vec<&mut Flag> = self
            .flags
            .iter_mut()
            .filter(|flag| flag.id == request.flag_id)
            .collect();
        let mut flag = flag_vec.first_mut().expect("flag not found");
        let mut team_id: u64 = 0;
        let mut current_player: Option<Player> = None;
        let mut players_team = None;

        if self.is_loading == true {
            for team in self.teams.iter_mut() {
                let mut player: Vec<&mut Player> = team
                    .1
                    .players
                    .iter_mut()
                    .filter(|player| player.id == player_id)
                    .collect();

                if player.first().is_some() {
                    let player_clone: Vec<Player> = player
                        .iter()
                        .map(|p| Player {
                            id: p.id,
                            pseudo: p.pseudo.to_owned(),
                            score: p.score,
                            inventory: p.inventory.clone(),
                        })
                        .collect();

                    team.1.score += 1;
                    player.first_mut().unwrap().score += 1;
                    team_id = team.1.id.clone();
                    players_team = Some(team.1.clone());
                    current_player = player_clone.first().cloned();
                }
            }

            if team_id != 0 && players_team.is_some() && current_player.is_some() {
                flag.is_captured = true;
                flag.player_id = player_id;
                flag.team_id = team_id;
                tx_capture_flag
                    .send(flag.clone().clone())
                    .expect("capture flag failed");
                return Ok(CaptureFlagResponse {
                    player: current_player.expect("player not found").clone(),
                    team: players_team.expect("team not found"),
                    flag_id: flag.id.clone(),
                    is_capture: true,
                });
            }
            return Err(ErrorResponse::error_message(
                500,
                "The player is not part of any team",
            ));
        }

        return Err(ErrorResponse::error_message(
            500,
            "The game is over : capture flag disable",
        ));
    }

    pub fn set_item_in_inventory(
        &mut self,
        request: GetItemRequest,
        player_id: u32,
    ) -> Option<GetItemResponse> {
        let item_index = self.items.iter().position(|i| i.id == request.item_id);
        if item_index.is_some() {
            let item = self.items.get(item_index.unwrap()).unwrap().clone();
            self.items.remove(item_index.unwrap());
            self.teams.iter_mut().for_each(|t| {
                t.1.players.iter_mut().for_each(|p| {
                    if p.id == player_id {
                        p.inventory.push(item.clone());
                    }
                })
            });
            let response = GetItemResponse {
                item_id: request.item_id.clone(),
            };
            return Some(response);
        } else {
            return None;
        }
    }

    pub fn drop_item(
        &mut self,
        request: DropItemRequest,
        player_id: u32,
    ) -> Option<DropItemResponse> {
        let mut item: Option<Item> = None;
        self.teams.iter_mut().for_each(|t| {
            t.1.players.iter_mut().for_each(|p| {
                if p.id == player_id {
                    let inventory = p.inventory.clone();
                    let item_index = inventory.iter().position(|i| i.id == request.item_id);

                    if item_index.is_some() {
                        let get_item = inventory.get(item_index.unwrap());
                        item = get_item.cloned();
                        p.inventory.remove(item_index.unwrap());
                    }
                }
            })
        });

        if item.is_some() {
            let mut item_drop = item.unwrap();
            item_drop.position = request.position;

            self.items.push(item_drop.clone());

            return Some(DropItemResponse {
                item: item_drop,
                position: request.position,
            });
        } else {
            return None;
        }
    }

    pub fn add_item(&mut self, request: AddItemRequest) -> AddItemResponse {
        self.items.push(request.item.clone());
        return AddItemResponse { item: request.item };
    }

    pub fn remove_item(&mut self, request: RemoveItemRequest) -> RemoveItemResponse {
        let index = self
            .items
            .iter()
            .position(|i| i.id == request.item_id)
            .unwrap();
        self.items.remove(index);
        return RemoveItemResponse {
            item_id: request.item_id,
        };
    }

    pub fn add_flag(&mut self, request: AddFlagRequest) -> AddFlagResponse {
        self.flags.push(request.flag.clone());
        return AddFlagResponse { flag: request.flag };
    }

    pub fn remove_flag(&mut self, request: RemoveFlagRequest) -> RemoveFlagResponse {
        let index = self
            .flags
            .iter()
            .position(|f| f.id == request.flag_id)
            .expect("flag not found");

        self.flags.remove(index);
        return RemoveFlagResponse {
            flag_id: request.flag_id,
        };
    }

    pub fn add_point_interest(
        &mut self,
        request: AddPointInterestRequest,
        client_id: u32,
    ) -> Option<AddPointInterestResponse> {
        let mut player_id: u32 = 0;
        let mut team_id: u64 = 0;
        self.teams.iter().for_each(|t| {
            let vec_player: Vec<&Player> =
                t.1.players.iter().filter(|p| p.id == client_id).collect();
            if vec_player.len() == 1 {
                player_id = vec_player.first().unwrap().id.to_owned();
                team_id = t.1.id.clone();
            }
        });

        if player_id == 0 || team_id == 0 {
            return None;
        }

        return Some(AddPointInterestResponse {
            point_id: request.point_id,
            player_id,
            team_id,
            position: request.position,
        });
    }

    pub fn use_item(
        &mut self,
        request: UseItemRequest,
        player_id: u32,
        sender: &Sender<ItemInLoading>,
    ) -> Option<Value> {
        match self.get_team_by_player(&player_id) {
            Some(team) => match self.get_player(&player_id) {
                Some(player) => {
                    let item_index = player
                        .inventory
                        .iter()
                        .position(|i| i.id == request.item_id);

                    if item_index.is_some() {
                        let mut item: Item =
                            player.inventory.get(item_index.unwrap()).unwrap().clone();
                        player.inventory.remove(item_index.unwrap());

                        match ItemType::iter().nth(item.item_type as usize).unwrap() {
                            ItemType::Satellite => {
                                sender
                                    .send(ItemInLoading {
                                        item: item.clone(),
                                        client_id: player_id,
                                        team_id: team.id.clone(),
                                    })
                                    .unwrap();

                                return Some(
                                    serde_json::to_value(SatelliteEnableResponse {
                                        enable: true,
                                        type_id: item.item_type,
                                        time: item.effect_duration,
                                        client_id: player_id,
                                    })
                                    .unwrap(),
                                );
                            }
                            ItemType::Extension => {
                                return Some(
                                    serde_json::to_value(ExtensionEnableResponse {
                                        type_id: item.item_type,
                                        enable: true,
                                        additionnal_location: 2,
                                    })
                                    .unwrap(),
                                );
                            }
                            ItemType::Mine => {
                                item.position = request.position;
                                self.traps.push(ItemInLoading {
                                    item: item.clone(),
                                    client_id: player_id,
                                    team_id: team.id.clone(),
                                });
                                return Some(
                                    serde_json::to_value(MineEnableResponse {
                                        type_id: item.item_type,
                                        enable: true,
                                        time: item.effect_duration,
                                        position: request.position,
                                        item,
                                        team_id: 0,
                                    })
                                    .unwrap(),
                                );
                            }
                            ItemType::Brouilleur => {
                                sender
                                    .send(ItemInLoading {
                                        item: item.clone(),
                                        client_id: player_id,
                                        team_id: team.id.clone(),
                                    })
                                    .unwrap();
                                return Some(
                                    serde_json::to_value(BrouilleurEnableResponse {
                                        type_id: item.item_type,
                                        enable: true,
                                        time: item.effect_duration,
                                        team_id: team.id,
                                    })
                                    .unwrap(),
                                );
                            }
                        }
                    } else {
                        return None;
                    }
                }
                None => {
                    return None;
                }
            },
            None => {
                return None;
            }
        }
    }

    pub fn get_player(&mut self, id: &u32) -> Option<&mut Player> {
        let mut player = None;
        self.teams.iter_mut().for_each(|t| {
            let player_index: Option<usize> = t.1.players.iter().position(|p| p.id == *id);
            if player_index.is_some() {
                player = t.1.players.get_mut(player_index.unwrap());
            }
        });

        return player;
    }

    pub fn get_team_by_player(&mut self, player_id: &u32) -> Option<Team> {
        let mut team = None;
        self.teams.iter_mut().for_each(|t| {
            let team_index: Option<usize> = t.1.players.iter().position(|p| p.id == *player_id);
            if team_index.is_some() {
                team = Some(t.1.clone());
            }
        });

        return team;
    }
}
