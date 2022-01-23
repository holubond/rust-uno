use crate::handler::create_game::create_game;
use crate::handler::restart_game::start_game;
use crate::repo::address_repo::AddressRepo;
use crate::repo::game_repo::InMemoryGameRepo;
use actix_web::{web, App, HttpServer};
use clap::Parser;
use handler::ws_connect::ws_connect;
use std::sync::{Arc, Mutex};

mod cards;
mod gamestate;
mod handler;
mod jwt;
mod repo;
mod ws;

#[derive(Parser)]
#[clap(version = "1.0", author = "L.G.")]
struct Opts {
    #[clap(short = 'p', long = "port", default_value = "9000")]
    port: String,
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    let port = opts.port;

    let game_repo = Arc::new(Mutex::new(InMemoryGameRepo::new()));
    let address_repo = Arc::new(AddressRepo::new(port.clone()));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(game_repo.clone()))
            .app_data(web::Data::new(address_repo.clone()))
            .service(create_game)
            .service(start_game)
            .service(ws_connect)
    })
    .bind(format!("127.0.0.1:{}", port))?
    .run()
    .await?;

    Ok(())
}
