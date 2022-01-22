use crate::repo::game_repo::InMemoryGameRepo;
use actix_web::{web, App, HttpServer};
use std::sync::{Arc, Mutex};
use clap::Parser;

mod cards;
mod gamestate;
mod handlers;
mod jwt_generate;
mod repo;


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
    let games = Vec::new();
    let game_repo = Arc::new(Mutex::new(InMemoryGameRepo::new(games, port.clone())));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(game_repo.clone()))
            .service(handlers::create_game)
    })
    .bind(format!("127.0.0.1:{}", port))?
    .run()
    .await?;

    Ok(())
}
