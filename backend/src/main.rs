use crate::repo::game_repo::InMemoryGameRepo;
use crate::repo::address_repo::AddressRepo;
use actix_web::{web, App, HttpServer};
use clap::Parser;
use std::sync::{Arc, Mutex};

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
    
    let game_repo = Arc::new(Mutex::new(InMemoryGameRepo::new()));
    let address_repo = Arc::new(AddressRepo::new(port.clone()));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(game_repo.clone()))
            .app_data(web::Data::new(address_repo.clone()))
            .service(handlers::create_game)
    })
    .bind(format!("127.0.0.1:{}", port))?
    .run()
    .await?;

    Ok(())
}
