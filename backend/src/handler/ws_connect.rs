use crate::gamestate::game::Game;
use crate::gamestate::player::Player;
use crate::handler::util::response::ErrMsg;
use crate::{AuthService, InMemoryGameRepo};
use actix_web::error::ErrorBadRequest;
use actix_web::{get, web, Error, HttpRequest, HttpResponse, ResponseError};
use std::sync::Mutex;

use crate::ws::{ws_conn::WSConn, ws_message::WSMsg};

#[get("/ws/token/{token}")]
pub async fn ws_connect(
    route_params: web::Path<String>,
    request: HttpRequest,
    stream: web::Payload,
    auth_service: web::Data<AuthService>,
    game_repo: web::Data<Mutex<InMemoryGameRepo>>,
) -> HttpResponse {
    let jwt = route_params.into_inner();

    let (game_id, author_name) = match auth_service.extract_data_from_token(jwt) {
        Err(response) => return response,
        Ok(data) => data,
    };

    let author_name = author_name.into_inner();

    let mut game_repo_mut = game_repo.lock().unwrap();

    let mut game_mut: &mut Game = match game_repo_mut.get_game_by_id_mut(game_id.into_inner()) {
        Err(response) => return response.into(),
        Ok(game) => game,
    };

    let (conn, response) = match WSConn::new(&request, stream) {
        Err(error) => return HttpResponse::InternalServerError().json(
            ErrMsg::new(error)
        ),
        Ok(data) => data,
    };

    if !game_mut.set_connection_to_player(&author_name, conn) {
        return HttpResponse::BadRequest().json(
            ErrMsg::new_from_scratch("Player with given name does not exist")
        );
    }

    let msg = WSMsg::status(game_mut, author_name.clone()).unwrap();
    let mut player: &mut Player = match game_mut.find_player_mut(&author_name) {
        Some(player) => player,
        _ => return HttpResponse::BadRequest().json(
            ErrMsg::new_from_scratch("Player with given name does not exist")
        ),
    };
    player.message(msg);

    response
}
