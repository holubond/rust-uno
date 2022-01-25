use std::sync::{Arc, Mutex};
use actix_web::{get, web, Error, HttpRequest, HttpResponse};
use actix_web::web::Path;
use crate::gamestate::game::Game;
use crate::InMemoryGameRepo;
use std::option::Option;

use crate::ws::{ws_conn::WSConn, ws_message::WSMsg};

#[get("/game/{gameID}/player/{name}/token/{token}")]
pub async fn ws_connect(r: HttpRequest, stream: web::Payload, params: web::Path<String>,params2: web::Path<String>,params3: web::Path<String>, game_repo: web::Data<Arc<Mutex<InMemoryGameRepo>>>) -> Result<HttpResponse, Error> {
    let gameID = params.into_inner();
    let player_name = &params2.into_inner();
    let token = &params3.into_inner();
/*
    let mut game: &mut Game = match game_repo.lock().unwrap().find_game_by_id(&gameID) {
        Some(game) => game,
        _=> return Error
    };
*/
    let mut game_repo = game_repo.lock().unwrap();
    let game = game_repo.find_game_by_id(&gameID).unwrap();
    /*
    if game.find_player(player_name).is_none() {
        return Error
    };
*/
    let player = game.find_player_mut(player_name).unwrap();
    let (conn, response) = WSConn::new(&r, stream)?;

    player.connection = Option::Some(conn);

    //conn.send(WSMsg::status(&game,"kaja".to_string()).unwrap());

    // Create a new WSConn
    //let (conn, response) = WSConn::new(&r, stream)?;

    // The connection is able to receive WSMsg messages
    // conn.send(WSMsg::custom("ABC".into())); // just an example, TODO remove me

    // TODO - Assign the WSConn to a player, create and send a STATUS message

    Ok(response)
}
