use crate::gamestate::game::Game;
use crate::gamestate::game_repo::{GameRepo, PostgresGameRepo};
use actix_web::{get, post, web, HttpResponse, Responder};
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

#[post("/LB/game")]
pub async fn create_game(
    mut data: web::Data<Arc<Mutex<PostgresGameRepo>>>,
    body: web::Json<GamePostData>,
) -> impl Responder {
    println!("ahoj");
    let result = data.lock().unwrap().create_game(body.name.clone()).await;
    println!("{:?}", result.as_ref().unwrap().id.clone());
    let response = Response {
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
    };
    match result {
        Ok(_) => HttpResponse::Created().json(response),
        Err(_) => HttpResponse::BadRequest().json(""),
    }
}
