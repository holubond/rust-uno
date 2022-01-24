use crate::gamestate::game::GameStatus;
use crate::{AuthorizationRepo, InMemoryGameRepo};
use actix_web::{post, web, HttpRequest, HttpResponse, Responder};
use actix_web_httpauth::headers::authorization::{Authorization, Bearer};
use std::sync::{Arc, Mutex};
use actix_web::http::header::Header;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageResponse {
    message: String,
}

#[post("game/{gameID}/statusRunning")]
pub async fn start_game(
    data: web::Data<Arc<Mutex<InMemoryGameRepo>>>,
    authorization_repo: web::Data<Arc<AuthorizationRepo>>,
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
        return HttpResponse::NotFound().json(MessageResponse {message:"Game not found".to_string()});
    }

    let game_index = data
        .lock()
        .unwrap()
        .games
        .iter()
        .position(|x| x.id == gameID)
        .unwrap();

    if Authorization::<Bearer>::parse(&request).is_err() {
        return HttpResponse::Unauthorized().json(MessageResponse {message:"No auth token provided by the client".to_string()});
    }
    let jwt = Authorization::<Bearer>::parse(&request)
        .unwrap()
        .to_string();
    let author_name = data.lock().unwrap().games.get(game_index).unwrap().find_author().unwrap().clone().name();
    if !authorization_repo.verify_jwt(author_name,gameID, jwt) {
        return HttpResponse::Forbidden().json(MessageResponse {message:"Token does not prove client is the Author".to_string()});
    }

    if data.lock().unwrap().games.get(game_index).unwrap().status() == GameStatus::Running {
        return HttpResponse::Conflict().json(MessageResponse {message:"Game cannot be started ((re)start is available to games with status LOBBY or FINISHED".to_string()});
    }

    HttpResponse::Ok().json(MessageResponse {message: "".to_string()})
}