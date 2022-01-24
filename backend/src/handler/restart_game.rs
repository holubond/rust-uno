use crate::gamestate::game::GameStatus;
use crate::{AuthorizationRepo, InMemoryGameRepo};
use actix_web::{post, web, HttpRequest, HttpResponse, Responder};
use actix_web_httpauth::headers::authorization::{Authorization, Bearer};
use std::sync::{Arc, Mutex};
use actix_web::http::header::Header;
use serde::Deserialize;
use serde::Serialize;
use crate::repo::game_repo::GameRepo;

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageResponse {
    message: String,
}

#[post("game/{gameID}/statusRunning")]
pub async fn start_game(
    game_repo: web::Data<Arc<Mutex<InMemoryGameRepo>>>,
    authorization_repo: web::Data<Arc<AuthorizationRepo>>,
    request: HttpRequest,
    params: web::Path<String>,
) -> impl Responder {
    let gameID = params.into_inner();

    let game = match game_repo.lock().unwrap().find_game_by_id(gameID.clone()).clone() {
        Some(game) => game.clone(),
        _=> return HttpResponse::NotFound().json(MessageResponse {message:"Game not found".to_string()})
    };

    let jwt = authorization_repo.parse_jwt(request);

    let jwt = match jwt {
        Ok(jwt) => jwt.to_string(),
        _ => return HttpResponse::Unauthorized().json(MessageResponse {message:"No auth token provided by the client".to_string()})
    };

    let author_name = game.find_author().unwrap().clone().name();
    if !authorization_repo.verify_jwt(author_name,gameID, jwt) {
        return HttpResponse::Forbidden().json(MessageResponse {message:"Token does not prove client is the Author".to_string()});
    }

    if game.status() == GameStatus::Running {
        return HttpResponse::Conflict().json(MessageResponse {message:"Game cannot be started ((re)start is available to games with status LOBBY or FINISHED".to_string()});
    }

    //TODO start game

    HttpResponse::NoContent().finish()
}