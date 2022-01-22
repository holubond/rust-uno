use crate::gamestate::game::Game;
use crate::jwt_generate::generate_jwt;
use crate::repo::game_repo::GameRepo;
use crate::repo::address_repo::AddressRepo;
use crate::InMemoryGameRepo;
use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;
use serde::Serialize;
use std::sync::{Arc, Mutex};

#[derive(Serialize, Deserialize, Debug)]
pub struct GamePostData {
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
    data: web::Data<Arc<Mutex<InMemoryGameRepo>>>,
    address_repo: web::Data<Arc<AddressRepo>>,
    body: web::Json<GamePostData>,
) -> impl Responder {

    let author_name = body.name.clone();
    
    if author_name.is_empty() {
        return HttpResponse::BadRequest().json("Name of the player cannot be empty");
    }

    let game = Game::new(author_name);
    data.lock().unwrap().add_game(game.clone());

    HttpResponse::Created().json(GameCreateResponse {
        gameID: game.id.clone(),
        server: address_repo.full_local_address(),
        token: generate_jwt(body.name.clone(), game.id.clone()),
    })
}
