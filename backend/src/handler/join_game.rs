use crate::gamestate::game::GameStatus;
use crate::ws::ws_message::WSMsg;
use crate::{AuthService, InMemoryGameRepo};
use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;
use serde::Serialize;
use std::sync::{Arc, Mutex};

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
    game_repo: web::Data<Mutex<InMemoryGameRepo>>,
    body: web::Json<GameJoinData>,
    authorization_repo: web::Data<AuthService>,
    params: web::Path<String>,
) -> impl Responder {
    let game_id = params.into_inner();
    let player_name = &body.name;

    if player_name.is_empty() {
        return HttpResponse::BadRequest().json(MessageResponse {
            message: "Name of the player cannot be empty.".to_string(),
        });
    }

    let mut game_repo = match game_repo.lock() {
        Err(_) => {
            return HttpResponse::InternalServerError().json(MessageResponse {
                message: "Cannot acquire lock on the game repo".to_string(),
            })
        }
        Ok(game_repo) => game_repo,
    };

    let game = match game_repo.find_game_by_id_mut(&game_id) {
        None => {
            return HttpResponse::NotFound().json(MessageResponse {
                message: "Game not found".to_string(),
            })
        }
        Some(game) => game,
    };

    if game.status() != GameStatus::Lobby {
        return HttpResponse::Gone().json(MessageResponse {
            message: "Game does not accept any new players.".to_string(),
        });
    }

    game.add_player(player_name.clone());

    let jwt = authorization_repo.generate_jwt(player_name, &game_id);

    HttpResponse::Created().json(GameJoinResponse {
        server: "TODO: implement".to_string(),
        token: jwt,
    })
}
