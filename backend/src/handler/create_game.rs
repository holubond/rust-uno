use crate::gamestate::game::Game;
use crate::repo::game_repo::GameRepo;
use crate::repo::address_repo::AddressRepo;
use crate::repo::authorization_repo::AuthorizationRepo;
use crate::{InMemoryGameRepo};
use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;
use serde::Serialize;
use std::sync::{Arc, Mutex};

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
        return HttpResponse::BadRequest().json("Name of the player cannot be empty");
    }

    let game = Game::new(author_name);
    let game_id = game.id.clone();

    game_repo.lock().unwrap().add_game(game);
    let jwt = authorization_repo.generate_jwt(author_name, &game_id);

    HttpResponse::Created().json(GameCreateResponse {
        gameID: game_id,
        server: address_repo.full_local_address(),
        token: jwt,
    })
}
