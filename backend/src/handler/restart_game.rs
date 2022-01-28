use crate::gamestate::game::GameStatus;
use crate::{AuthService, InMemoryGameRepo};
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
    game_repo: web::Data<Mutex<InMemoryGameRepo>>,
    authorization_repo: web::Data<AuthService>,
    request: HttpRequest,
    params: web::Path<String>,
) -> impl Responder {
    let gameID = params.into_inner();

    let mut game_repo = game_repo.lock().unwrap();

    let game = match game_repo.find_game_by_id_mut(&gameID) {
        Some(game) => game,
        _=> return HttpResponse::NotFound().json(MessageResponse {message:"Game not found".to_string()})
    };

    let jwt = authorization_repo.parse_jwt(request);

    let jwt = match jwt {
        Ok(jwt) => jwt.to_string(),
        _ => return HttpResponse::Unauthorized().json(MessageResponse {message:"No auth token provided by the client".to_string()})
    };

    let claims = match authorization_repo.valid_jwt(&jwt)  {
        Ok(claims) => claims,
        _ => return HttpResponse::Unauthorized().json(MessageResponse {message:"Token is not valid".to_string()})
    };

    let author_name = match game.find_author() {
        Some(player) => player.name(),
        _ => return HttpResponse::InternalServerError().json(MessageResponse{message: "Game does not have player".to_string()})
    };
    if !authorization_repo.verify_jwt(author_name,gameID, claims) {
        return HttpResponse::Forbidden().json(MessageResponse {message:"Token does not prove client is the Author".to_string()});
    }

    if game.status() == GameStatus::Running {
        return HttpResponse::Conflict().json(MessageResponse {message:"Game cannot be started ((re)start is available to games with status LOBBY or FINISHED".to_string()});
    }

    game.start();

    HttpResponse::NoContent().finish()
}