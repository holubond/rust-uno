use crate::gamestate::game::Game;
use crate::gamestate::player::Player;
use crate::{AuthService, InMemoryGameRepo};
use actix_web::error::ErrorBadRequest;
use actix_web::{get, web, Error, HttpRequest, HttpResponse};
use std::sync::Mutex;

use crate::ws::{ws_conn::WSConn, ws_message::WSMsg};

#[get("/ws/token/{token}")]
pub async fn ws_connect(
    route_params: web::Path<String>,
    request: HttpRequest,
    stream: web::Payload,
    auth_service: web::Data<AuthService>,
    game_repo: web::Data<Mutex<InMemoryGameRepo>>,
) -> Result<HttpResponse, Error> {
    let jwt = route_params.into_inner();

    let mut game_repo_mut = game_repo.lock().unwrap();

    let jwt = match &jwt.is_empty() {
        false => jwt.clone(),
        _ => return Err(ErrorBadRequest("Token si empty")),
    };

    let (game_id, author_name) = match auth_service.extract_data_from_jwt(jwt) {
        Ok((author_name, game_id)) => (author_name, game_id),
        Err(_) => return Err(ErrorBadRequest("Token is invalid")),
    };

    let mut game_mut: &mut Game = match game_repo_mut.find_game_by_id_mut(&game_id) {
        Some(game) => game,
        _ => return Err(ErrorBadRequest("Game with given id does not exist")),
    };

    let (conn, response) = WSConn::new(&request, stream)?;
    if !game_mut.set_connection_to_player(&author_name, conn) {
        return Err(ErrorBadRequest("Player with given name does not exist"));
    }

    let msg = WSMsg::status(game_mut, author_name.clone()).unwrap();
    let mut player: &mut Player = match game_mut.find_player_mut(&author_name) {
        Some(player) => player,
        _ => return Err(ErrorBadRequest("Player with given name does not exist")),
    };
    player.message(msg);

    Ok(response)
}
