use actix_web::{web, App, HttpServer};
use std::sync::{Arc, Mutex};
use crate::repo::game_repo::InMemoryGameRepo;

mod cards;
mod gamestate;
mod handlers;
mod repo;
mod jwt_generate;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let games = Vec::new();
    let game_repo = Arc::new(Mutex::new(InMemoryGameRepo::new(games)));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(game_repo.clone()))
            .service(handlers::create_game)
    })
    .bind("127.0.0.1:9000")?
    .run()
    .await?;

    Ok(())
}
