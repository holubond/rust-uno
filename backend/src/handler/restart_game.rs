use crate::gamestate::game::GameStatus;
use crate::handler::util::safe_lock::safe_lock;
use crate::{AuthService, InMemoryGameRepo};
use actix_web::{post, web, HttpRequest, HttpResponse, Responder};
use std::sync::Mutex;
use super::util::response::ErrMsg;

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
        None => return Err(
            HttpResponse::InternalServerError().json(
                ErrMsg::new_from_scratch("Author of the game not found")
            ) ),
        Some(author) => author.name(),
    };
    
    player_name_from_token.check(&author_name)?;

    if game.status() == GameStatus::Running {
        return Err(
            HttpResponse::Conflict().json(
                ErrMsg::new_from_scratch("Game cannot be (re)started, its status is RUNNING")
            )
        );
    }

    game.start();

    Ok(HttpResponse::NoContent().finish())
}