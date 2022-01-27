use std::borrow::{Borrow, BorrowMut};
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use actix_web::{get, web, Error, HttpRequest, HttpResponse};
use actix_web::web::Path;
use crate::gamestate::game::Game;
use crate::{AuthService, InMemoryGameRepo};
use std::option::Option;
use actix::fut::err;
use actix_web::error::{ErrorBadGateway, ErrorBadRequest};
use actix_web_httpauth::headers::authorization::{Authorization, Bearer};
use crate::gamestate::player::Player;

use crate::ws::{ws_conn::WSConn, ws_message::WSMsg};

#[get("/ws/token/{token}")]
pub async fn ws_connect(r: HttpRequest, stream: web::Payload, params: web::Path<String>, game_repo: web::Data<Arc<Mutex<InMemoryGameRepo>>>, authorization_repo: web::Data<Arc<AuthService>>,) -> Result<HttpResponse, Error> {
    let jwt = params.into_inner();

    let mut game_repo_mut = game_repo.lock().unwrap();

    let jwt = match &jwt.is_empty() {
        false => jwt.clone(),
        _ => return Err(ErrorBadRequest("Token si empty"))
    };

    let (game_id, author_name) = match authorization_repo.extract_data_from_jwt(jwt) {
        Ok((author_name, game_id)) => (author_name, game_id),
        Err(_) => return Err(ErrorBadRequest("Token is invalid"))
    };

    let mut game_mut: &mut Game = match game_repo_mut.find_game_by_id_mut(&game_id) {
        Some(game) => game,
        _ => return Err(ErrorBadRequest("Game with given id does not exist"))
    };

    let (conn, response) = WSConn::new(&r, stream)?;
    if !game_mut.set_connection_to_player(&author_name, conn) {
        return Err(ErrorBadRequest("Player with given name does not exist"))
    }

    let msg = WSMsg::status(game_mut, author_name.clone()).unwrap();
    let mut player: &mut Player = match game_mut.find_player_mut(&author_name) {
        Some(player) => player,
        _ => return Err(ErrorBadRequest("Player with given name does not exist"))
    };
    player.message(msg);

    Ok(response)
}
