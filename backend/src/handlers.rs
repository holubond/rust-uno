use crate::gamestate::game_repo::{GameRepo, StableGameRepo};
use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;
use serde::Serialize;
use std::sync::{Arc, Mutex};

#[derive(Serialize, Deserialize, Debug)]
pub struct GamePostData {
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    gameID: String,
    server: String,
    token: String,
}

#[post("/game")]
pub async fn create_game(
    data: web::Data<Arc<Mutex<StableGameRepo>>>,
    body: web::Json<GamePostData>,
) -> impl Responder {
    let result = data.lock().unwrap().create_game(body.name.clone()).await;
    match result {
        Ok(_) => HttpResponse::Created().json(Response {
            gameID: result.as_ref().unwrap().id.clone(),
            server: "127.0.0.1:9000".to_string(),
            token: result
                .as_ref()
                .unwrap()
                .players
                .iter()
                .find(|x| x.is_author)
                .unwrap()
                .jwt
                .clone(),
        }),
        Err(_) => HttpResponse::BadRequest().json("Name cannot be empty"),
    }
}
