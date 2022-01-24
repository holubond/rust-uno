use crate::gamestate::game::GameStatus;
use crate::{AddressRepo, AuthorizationRepo, InMemoryGameRepo};
use actix_web::web::Path;
use actix_web::{post, web, HttpResponse, Responder};
use local_ip_address::local_ip;
use serde::Deserialize;
use serde::Serialize;
use std::sync::{Arc, Mutex};

#[derive(Serialize, Deserialize, Debug)]
pub struct GameJoinData {
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameJoinResponse {
    server: String,
    token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageResponse {
    message: String,
}

#[post("/game/{gameID}/player")]
pub async fn join_game(
    game_repo: web::Data<Arc<Mutex<InMemoryGameRepo>>>,
    body: web::Json<GameJoinData>,
    address_repo: web::Data<Arc<AddressRepo>>,
    authorization_repo: web::Data<Arc<AuthorizationRepo>>,
    params: web::Path<String>,
) -> impl Responder {
    let gameID = params.into_inner();
    if body.name.is_empty() {
        return HttpResponse::BadRequest().json(MessageResponse {
            message: "Name of the player cannot be empty.".to_string(),
        });
    }
    let mut game_repo = game_repo.lock().unwrap();

    let game = match game_repo.find_game_by_id(&gameID) {
        Some(game) => game,
        _=> return HttpResponse::NotFound().json(MessageResponse {message:"Game not found".to_string()})
    };
    if game.status() != GameStatus::Lobby
    {
        return HttpResponse::NotFound().json(MessageResponse {
            message: "Game does not accept any new players.".to_string(),
        });
    }
    game.add_player(body.name.clone());

    let jwt = authorization_repo.generate_jwt(&body.name, &gameID);

    HttpResponse::Created().json(GameJoinResponse {
        server: address_repo.full_local_address(),
        token: jwt,
    })
}