use crate::err::add_player::AddPlayerError;
use crate::gamestate::game::GameStatus;
use crate::handler::util::response::ErrMsg;
use crate::handler::util::safe_lock::safe_lock;
use crate::{AuthService, InMemoryGameRepo};
use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;
use serde::Serialize;
use std::sync::Mutex;

#[derive(Deserialize, Debug)]
pub struct RequestBody {
    name: String,
}

#[derive(Serialize, Debug)]
pub struct SuccessResponse {
    token: String,
}

#[post("/game/{gameID}/player")]
pub async fn join_game(
    route_params: web::Path<String>,
    request_body: web::Json<RequestBody>,
    auth_service: web::Data<AuthService>,
    game_repo: web::Data<Mutex<InMemoryGameRepo>>,
) -> impl Responder {
    let game_id = route_params.into_inner();
    let player_name = &request_body.name;

    if player_name.is_empty() {
        return HttpResponse::BadRequest().json(ErrMsg::new_from_scratch(
            "Name of the player cannot be empty.",
        ));
    }

    let mut game_repo = match safe_lock(&game_repo) {
        Err(response) => return response,
        Ok(game_repo) => game_repo,
    };

    let game = match game_repo.get_game_by_id_mut(game_id.clone()) {
        Err(response) => return response.into(),
        Ok(game) => game,
    };

    if game.status() != GameStatus::Lobby {
        return HttpResponse::Gone().json(ErrMsg::new_from_scratch(
            "Game does not accept any new players.",
        ));
    }

    match game.add_player(player_name.clone()) {
        Ok(_) => {
            let jwt = auth_service.generate_jwt(player_name, &game_id);
            HttpResponse::Created().json(SuccessResponse { token: jwt })
        }
        Err(err) => match err {
            AddPlayerError::AlreadyExists(x) => {
                HttpResponse::Conflict().json(ErrMsg::new_from_scratch(&x))
            }
            AddPlayerError::CreateStatusError(x) => {
                HttpResponse::InternalServerError().json(ErrMsg::new(x))
            }
        },
    }
}
