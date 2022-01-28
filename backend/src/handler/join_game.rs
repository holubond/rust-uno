use crate::gamestate::game::GameStatus;
use crate::handler::util::response::{ErrMsg, ErrResp};
use crate::handler::util::safe_lock::safe_lock;
use crate::{AuthService, InMemoryGameRepo};
use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;
use serde::Serialize;
use std::sync::Mutex;

#[derive(Serialize, Deserialize, Debug)]
pub struct GameJoinData {
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameJoinResponse {
    server: String,
    token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageResponse {
    message: String,
}

#[post("/game/{gameID}/player")]
pub async fn join_game(
    route_params: web::Path<String>,
    request_body: web::Json<GameJoinData>,
    auth_service: web::Data<AuthService>,
    game_repo: web::Data<Mutex<InMemoryGameRepo>>,
) -> impl Responder {
    let game_id = route_params.into_inner();
    let player_name = &request_body.name;

    if player_name.is_empty() {
        return HttpResponse::BadRequest().json(
            ErrMsg::new("Name of the player cannot be empty.")
        );
    }

    let mut game_repo = match safe_lock(&game_repo) {
        Err(response) => return response,
        Ok(game_repo) => game_repo,
    };

    let game = match game_repo.find_game_by_id_mut(&game_id) {
        None => return ErrResp::game_not_found(game_id),
        Some(game) => game,
    };

    if game.status() != GameStatus::Lobby {
        return HttpResponse::Gone().json(MessageResponse {
            message: "Game does not accept any new players.".to_string(),
        });
    }

    game.add_player(player_name.clone());

    let jwt = auth_service.generate_jwt(player_name, &game_id);

    HttpResponse::Created().json(GameJoinResponse {
        server: "TODO: implement".to_string(),
        token: jwt,
    })
}
