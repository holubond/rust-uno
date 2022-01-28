use crate::handler::create_game::create_game;
use crate::handler::draw_card::draw_card;
use crate::handler::join_game::join_game;
use crate::handler::restart_game::start_game;
use crate::handler::service::auth::AuthService;
use crate::repo::address_repo::AddressRepo;
use crate::repo::game_repo::InMemoryGameRepo;
use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use clap::Parser;
use handler::{play_card::play_card, ws_connect::ws_connect};
use std::{
    env,
    sync::{Arc, Mutex},
};

mod cards;
mod err;
mod gamestate;
mod handler;
mod repo;
mod ws;

#[derive(Parser)]
#[clap(version = "1.0", author = "L.G.")]
struct Opts {
    #[clap(short = 'p', long = "port", default_value = "9000")]
    port: String,
}

async fn fallback_to_spa() -> actix_files::NamedFile {
    actix_files::NamedFile::open("./static/index.html").unwrap()
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let port = match env::var("PORT") {
        Ok(p) => p,
        Err(_) => Opts::parse().port,
    };

    let game_repo = web::Data::new(InMemoryGameRepo::new());
    let address_repo = web::Data::new(AddressRepo::new(port.clone()));
    let auth_service = web::Data::new(AuthService::new());

    println!("Starting server on port {}", port);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_header()
            .allow_any_origin()
            .allow_any_method();

        App::new()
            .wrap(cors)
            .app_data(game_repo.clone())
            .app_data(address_repo.clone())
            .app_data(auth_service.clone())
            .service(create_game)
            .service(start_game)
            .service(draw_card)
            .service(join_game)
            .service(play_card)
            .service(ws_connect)
            .service(actix_files::Files::new("/", "./static").index_file("index.html"))
            .default_service(web::resource("").route(web::get().to(fallback_to_spa)))
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await?;

    Ok(())
}
