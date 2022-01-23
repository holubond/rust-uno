use std::sync::{Arc, Mutex};
use actix_web::{post, HttpResponse, Responder, web};
use actix_web::web::Path;
use local_ip_address::local_ip;
use crate::{AddressRepo, AuthorizationRepo, InMemoryGameRepo};
use serde::Serialize;
use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug)]
pub struct GameJoinData {
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameJoinResponse {
    server: String,
    token: String,
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
        return HttpResponse::BadRequest().message_body("Name of the player cannot be empty");
    }
    if data.lock().unwrap().games.iter_mut().find(|x|x.id == gameID).is_none(){
        return HttpResponse::NotFound().message_body("Game does not exist.");
    }
    data.lock().unwrap().games.iter_mut().find(|x|x.id == gameID).unwrap().add_player(body.name.clone());

    let jwt = authorization_repo.generate_jwt(&body.name,&gameID);

    HttpResponse::Created().json(GameJoinResponse {
        server: address_repo.full_local_address(),
        token: jwt,
    })
}