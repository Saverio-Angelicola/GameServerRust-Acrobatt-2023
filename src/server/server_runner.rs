use std::{
    env,
    sync::{
        mpsc::{self, Receiver},
        Arc, Mutex,
    },
};
use ws::{listen, Sender};

use crate::core::game::{
    game_config::Flag, game_initialization::init_game_loop, items::ItemInLoading,
};

use super::{
    server_config::{Client, ServerConfig},
    server_game_loading::{extract_configuration, load_configuration},
};

pub async fn run_server() {
    let thread = tokio::spawn(async {
        // Port passé en paramètre dans la CLI
        let port: String = env::args().nth(1).unwrap_or(String::from("8000"));
        let host: String = format!("0.0.0.0:{}", &port);

        // Channel de communication entre les threads
        let (tx_game_launcher, rx_game_launcher) = mpsc::channel();
        let (tx_websocket, rx_websocket): (mpsc::Sender<ws::Sender>, Receiver<ws::Sender>) =
            mpsc::channel();
        let (tx_capture_flag, rx_capture_flag): (mpsc::Sender<Flag>, Receiver<Flag>) =
            mpsc::channel();
        let (tx_use_item, rx_use_item): (mpsc::Sender<ItemInLoading>, Receiver<ItemInLoading>) =
            mpsc::channel();

        // Initialisation des variables qui vont servir de buffer pour récuperer le contenu des fichiers de configurations
        let mut game_configuration: String = String::new();
        let mut map_configuration: String = String::new();

        // Charge la configuration de jeu et de la map
        load_configuration(&mut game_configuration, &mut map_configuration);
        let initial_game_config = extract_configuration(&game_configuration, &map_configuration);
        let game_loaded = Arc::new(Mutex::new(initial_game_config.clone()));

        // données partagées alloué sur le tas et protégé par un mutex
        let next_id = Arc::new(Mutex::new(1));
        let clients = Arc::new(Mutex::new(Vec::new()));

        // Instancie le thread du jeu
        init_game_loop(
            rx_game_launcher,
            rx_websocket,
            rx_capture_flag,
            rx_use_item,
            &game_loaded,
            initial_game_config.clone(),
        );

        listen(host.clone(), |out: Sender| {
            tx_websocket.send(out.clone()).expect("send failed");
            ServerConfig {
                initial_config: initial_game_config.clone(),
                game: game_loaded.clone(),
                out,
                current_client: Client::default(),
                game_trigger: tx_game_launcher.clone(),
                next_id: next_id.clone(),
                clients: clients.clone(),
                game_configuration_file: game_configuration.clone(),
                map_config_file: map_configuration.clone(),
                tx_capture_flag: tx_capture_flag.clone(),
                tx_use_item: tx_use_item.clone(),
            }
        })
        .expect("websocket server KO");
    });

    thread.await.expect("thread KO");
}
