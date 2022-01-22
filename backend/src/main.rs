use crate::gamestate::game_repo::StableGameRepo;
use actix_web::{web, App, HttpServer};
use std::sync::{Arc, Mutex};
use actix_cors::Cors;

mod cards;
mod gamestate;
mod handlers;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let games = Vec::new();
    let game_repo = Arc::new(Mutex::new(StableGameRepo::new(games)));

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_header()
            .allow_any_origin()
            .allow_any_method();
        App::new()
            .wrap(cors)
            .app_data(web::Data::new(game_repo.clone()))
            .service(handlers::create_game)
    })
    .bind("127.0.0.1:9000")?
    .run()
    .await?;

    Ok(())
}
