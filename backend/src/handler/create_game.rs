use crate::gamestate::game::Game;
use crate::handler::service::auth::AuthorizationRepo;
use crate::repo::address_repo::AddressRepo;
use crate::InMemoryGameRepo;
use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;
use serde::Serialize;
use std::sync::{Arc, Mutex};

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageResponse {
    message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameCreateData {
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameCreateResponse {
    gameID: String,
    server: String,
    token: String,
}

#[post("/game")]
pub async fn create_game(
    game_repo: web::Data<Arc<Mutex<InMemoryGameRepo>>>,
    authorization_repo: web::Data<Arc<AuthorizationRepo>>,
    address_repo: web::Data<Arc<AddressRepo>>,
    body: web::Json<GameCreateData>,
) -> impl Responder {

    let author_name = &body.name;
    
    if author_name.is_empty() {
        return HttpResponse::BadRequest().json(MessageResponse{message: "Name of the player cannot be empty".to_string()});
    }

    let game = Game::new(author_name.clone());
    let game_id = game.id.clone();
    let jwt = authorization_repo.generate_jwt(author_name, &game_id);

    game_repo.lock().unwrap().add_game(game);

    HttpResponse::Created().json(GameCreateResponse {
        gameID: game_id,
        server: address_repo.full_local_address(),
        token: jwt,
    })
}
