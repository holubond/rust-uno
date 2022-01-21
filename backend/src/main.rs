use crate::gamestate::game_repo::PostgresGameRepo;
use actix_web::{web, App, HttpServer};

mod cards;
mod gamestate;
mod handlers;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let games = Vec::new();
    let game_repo = (PostgresGameRepo::new(games));

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
