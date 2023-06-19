extern crate actix_files;
extern crate actix_web;
extern crate console;

use std::path::PathBuf;

use actix_files as fs;
use actix_web::{middleware, rt, web, App, HttpServer, Result};
use console::Style;

async fn single_page_app() -> Result<fs::NamedFile> {
    let path: PathBuf = PathBuf::from("client/build/index.html");
    Ok(fs::NamedFile::open(path)?)
}

pub async fn launch_client() {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let blue: Style = Style::new().blue();

    let prefix: &str = "0.0.0.0:"; // // Use 0.0.0.0 instead of localhost or 127.0.0.1 to use Actix with docker
    let port: i32 = 5050; // We will use 80 for aws with env variable.
    let target: String = format!("{}{}", prefix, port);

    println!(
        "\nServer ready at {}",
        blue.apply_to(format!("http://{}", &target))
    );

    let srv = HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(single_page_app))
            .route("/game", web::get().to(single_page_app))
            .service(fs::Files::new("/", "client/build").index_file("index.html"))
    })
    .bind(&target)
    .unwrap()
    .run();

    rt::spawn(srv);
}
