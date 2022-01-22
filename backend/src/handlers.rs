use std::fmt::format;
use crate::jwt_generate::generate_jwt;
use crate::repo::game_repo::GameRepo;
use crate::InMemoryGameRepo;
use actix_web::{post, web, HttpResponse, Responder};
use local_ip_address::local_ip;
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

#[post("/GameCreateData")]
pub async fn create_game(
    data: web::Data<Arc<Mutex<InMemoryGameRepo>>>,
    body: web::Json<GamePostData>,
) -> impl Responder {
    if body.name.clone().is_empty() {
        return HttpResponse::BadRequest().json("Name of the player cannot be empty");
    }
    let game_result = data.lock().unwrap().create_game(body.name.clone()).await;
    let port = data.lock().unwrap().port.clone();

    HttpResponse::Created().json(GameCreateResponse {
        gameID: game_result.as_ref().unwrap().id.clone(),
        server: format!("{}:{}",local_ip().unwrap(), port),
        token: generate_jwt(body.name.clone(), game_result.as_ref().unwrap().id.clone()),
    })
}
