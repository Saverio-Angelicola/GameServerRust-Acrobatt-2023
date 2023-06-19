extern crate actix_files;
extern crate actix_web;
extern crate console;

use actix_web::Result;
use server::{client_runner::launch_client, server_runner::run_server};

mod core;
mod server;
mod shared;

#[actix_web::main]
async fn main() -> Result<()> {
    launch_client().await;
    run_server().await;
    Ok(())
}
