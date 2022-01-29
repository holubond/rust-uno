use crate::gamestate::game::GameStatus;
use crate::handler::util::safe_lock::safe_lock;
use crate::{AuthService, InMemoryGameRepo};
use actix_web::{post, web, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;
use serde::Serialize;
use std::sync::Mutex;

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageResponse {
    message: String,
}

#[post("game/{gameID}/statusRunning")]
pub async fn start_game(
    route_params: web::Path<String>,
    request: HttpRequest,
    auth_service: web::Data<AuthService>,
    game_repo: web::Data<Mutex<InMemoryGameRepo>>,
) -> impl Responder {
    match start_game_response(route_params, request, auth_service, game_repo) {
        Err(response) => return response,
        Ok(response) => return response,
    }
}

pub fn start_game_response(
    route_params: web::Path<String>,
    request: HttpRequest,
    auth_service: web::Data<AuthService>,
    game_repo: web::Data<Mutex<InMemoryGameRepo>>,
) -> Result<HttpResponse, HttpResponse> {
    let game_id = route_params.into_inner();

    let (game_id_from_token, player_name_from_token) = auth_service.extract_data(&request)?;

    let game_id = game_id_from_token.check(game_id)?;

    let mut game_repo = safe_lock(&game_repo)?;

    let game = game_repo.get_game_by_id_mut(game_id.clone())?;

    let author_name = match game.find_author() {
        Some(player) => player.name(),
        _ => {
            return Err(HttpResponse::InternalServerError().json(MessageResponse {
                message: "Game does not have player".to_string(),
            }))
        }
    };
    
    player_name_from_token.check(&author_name)?;

    if game.status() == GameStatus::Running {
        return Err(HttpResponse::Conflict().json(MessageResponse {message:"Game cannot be started ((re)start is available to games with status LOBBY or FINISHED".to_string()}));
    }

    game.start();

    Ok(HttpResponse::NoContent().finish())
}