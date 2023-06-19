use std::sync::MutexGuard;

use crate::server::{
    server_config::ServerConfig, server_error::ErrorResponse, server_response::ServerResponse,
};

use super::{
    game_config::Game,
    game_request::GameTriggerRequest,
    game_response::{GameLauncherResponse, ScoreResponse},
};

// Gère le déclenchement et l'arrêt d'une partie
pub fn game_launch(
    game_trigger: GameTriggerRequest,
    game: &mut MutexGuard<Game>,
    server: &ServerConfig,
) {
    if game_trigger.trigger && !game.is_loading {
        game.teams.iter_mut().for_each(|t| {
            t.1.score = 0;
            t.1.players.iter_mut().for_each(|p| {
                p.score = 0;
            });
        });
        game.play(&server.game_trigger);

        let mut server_response = ServerResponse {
            command: "game-launcher".to_owned(),
            data: serde_json::to_value(GameLauncherResponse {
                game_loading: true,
                message: "The game begin...".to_owned(),
            })
            .expect("game start failed"),
        };

        server
            .out
            .broadcast(server_response.to_json())
            .expect("broadcast failed");
    } else if game.is_loading && game_trigger.trigger {
        server
            .out
            .send(ErrorResponse::error_message(500, "Game is already started"))
            .expect("broadcast failed");
    }

    if !game_trigger.trigger && game.is_loading {
        game.stop(&server.game_trigger);

        let mut server_response = ServerResponse {
            command: "game-launcher".to_owned(),
            data: serde_json::to_value(GameLauncherResponse {
                game_loading: false,
                message: "The game is over".to_owned(),
            })
            .unwrap(),
        };

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

        let server_score_response = ServerResponse {
            command: String::from("score"),
            data: serde_json::to_value(score_response).expect("Serialisation failed"),
        }
        .to_json();
        server
            .out
            .broadcast(server_score_response)
            .expect("broadcast failed");

        server
            .out
            .broadcast(server_response.to_json())
            .expect("broadcast failed");

        game.teams = server.initial_config.teams.clone();
        game.flags = server.initial_config.flags.clone();
        game.items = server.initial_config.items.clone();
    } else if !game_trigger.trigger && !game.is_loading {
        let error_message = ErrorResponse::error_message(500, "Game is already stopped!");
        server.out.send(error_message).expect("send failed");
    }
}
