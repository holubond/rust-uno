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
pub struct ErrorResponse {
    message: String,
}

#[post("/game/{gameID}/player")]
pub async fn join_game(
    data: web::Data<Arc<Mutex<InMemoryGameRepo>>>,
    body: web::Json<GameJoinData>,
    address_repo: web::Data<Arc<AddressRepo>>,
    authorization_repo: web::Data<Arc<AuthorizationRepo>>,
    params: web::Path<String>,
) -> impl Responder {
    let gameID = params.into_inner();
    if body.name.clone().is_empty() {
        return HttpResponse::BadRequest().json(ErrorResponse {
            message: "Name of the player cannot be empty.".to_string(),
        });
    }
    if data
        .lock()
        .unwrap()
        .games
        .iter_mut()
        .find(|x| x.id == gameID)
        .is_none()
    {
        return HttpResponse::NotFound().json(ErrorResponse {
            message: "Game does not exist.".to_string(),
        });
    }
    if data
        .lock()
        .unwrap()
        .games
        .iter_mut()
        .find(|x| x.id == gameID)
        .unwrap()
        .status
        != GameStatus::Lobby
    {
        return HttpResponse::NotFound().json(ErrorResponse {
            message: "Game does not accept any new players.".to_string(),
        });
    }
    data.lock()
        .unwrap()
        .games
        .iter_mut()
        .find(|x| x.id == gameID)
        .unwrap()
        .add_player(body.name.clone());

    let jwt = authorization_repo.generate_jwt(&body.name, &gameID);

    HttpResponse::Created().json(GameJoinResponse {
        server: address_repo.full_local_address(),
        token: jwt,
    })
}
