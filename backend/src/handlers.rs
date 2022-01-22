use crate::gamestate::game::GameStatus;
use crate::repo::game_repo::GameRepo;
use crate::InMemoryGameRepo;
use actix_web::http::header::Header;
use actix_web::{post, web, HttpRequest, HttpResponse, Responder};
use actix_web_httpauth::headers::authorization::{Authorization, Bearer};
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
    let game_index = data
        .lock()
        .unwrap()
        .games
        .iter()
        .position(|x| x.id == game_result.as_ref().unwrap().id)
        .unwrap();
    let jwt = data
        .lock()
        .unwrap()
        .games
        .get(game_index)
        .unwrap()
        .find_author()
        .unwrap()
        .jwt
        .clone();

    HttpResponse::Created().json(GameCreateResponse {
        gameID: game_result.as_ref().unwrap().id.clone(),
        server: format!("{}:{}", local_ip().unwrap(), port),
        token: jwt,
    })
}

#[post("game/{gameID}/statusRunning")]
pub async fn start_game(
    data: web::Data<Arc<Mutex<InMemoryGameRepo>>>,
    request: HttpRequest,
    params: web::Path<String>,
) -> impl Responder {
    let gameID = params.into_inner();
    if data
        .lock()
        .unwrap()
        .games
        .iter()
        .position(|x| x.id == gameID)
        .is_none()
    {
        return HttpResponse::NotFound();
    }

    let game_index = data
        .lock()
        .unwrap()
        .games
        .iter()
        .position(|x| x.id == gameID)
        .unwrap();

    if Authorization::<Bearer>::parse(&request).is_err() {
        return HttpResponse::Unauthorized();
    }
    let jwt = Authorization::<Bearer>::parse(&request)
        .unwrap()
        .to_string();
    let jwt_author = format!(
        "Bearer {}",
        data.lock()
            .unwrap()
            .games
            .get(game_index)
            .unwrap()
            .find_author()
            .unwrap()
            .clone()
            .jwt
    );
    if jwt != jwt_author {
        return HttpResponse::Forbidden();
    }

    if data.lock().unwrap().games.get(game_index).unwrap().status == GameStatus::Running {
        return HttpResponse::Conflict();
    }

    HttpResponse::Ok()
}
