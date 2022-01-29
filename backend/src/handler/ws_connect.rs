use crate::gamestate::game::Game;
use crate::gamestate::player::Player;
use crate::handler::util::response::ErrMsg;
use crate::handler::util::safe_lock::safe_lock;
use crate::{AuthService, InMemoryGameRepo};
use actix_web::{get, web, HttpRequest, HttpResponse};
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

    let (game_id, player_name) = match auth_service.extract_data_from_token(jwt) {
        Err(response) => return response,
        Ok(data) => data,
    };
    let player_name = player_name.into_inner();

    let mut game_repo = match safe_lock(&game_repo) {
        Err(response) => return response,
        Ok(repo) => repo,
    };

    let game = match game_repo.get_game_by_id_mut(game_id.into_inner()) {
        Err(response) => return response.into(),
        Ok(game) => game,
    };

    let msg = match WSMsg::status(game, player_name.clone()) {
        Err(error) => return HttpResponse::InternalServerError().json(
            ErrMsg::new(error)
        ),
        Ok(msg) => msg,
    };

    let player = match game.find_player_mut(&player_name) {
        None => return HttpResponse::NotFound().json(
            ErrMsg::new_from_scratch("Player with this name does not exist")
        ),
        Some(player) => player,
    };
    
    let (conn, response) = match WSConn::new(&request, stream) {
        Err(error) => return HttpResponse::InternalServerError().json(
            ErrMsg::new(error)
        ),
        Ok(data) => data,
    };
    
    player.set_connection(conn);
    
    player.message(msg);

    response
}
