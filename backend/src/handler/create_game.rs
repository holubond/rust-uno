use crate::gamestate::game::Game;
use crate::handler::service::auth::AuthService;
use crate::handler::util::response::ErrMsg;
use crate::InMemoryGameRepo;
use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;
use serde::Serialize;
use std::sync::Mutex;

use super::util::safe_lock::safe_lock;

#[derive(Deserialize, Debug)]
pub struct RequestBody {
    name: String,
    ais: String
}

#[derive(Serialize, Debug)]
pub struct SuccessResponse {
    #[serde(rename(serialize = "gameID", deserialize = "gameID"))]
    game_id: String,
    token: String,
}

#[post("/game")]
pub async fn create_game(
    request_body: web::Json<RequestBody>,
    auth_service: web::Data<AuthService>,
    game_repo: web::Data<Mutex<InMemoryGameRepo>>,
) -> impl Responder {
    let author_name = &request_body.name;
    let ais = match request_body.ais.parse::<usize>() {
        Ok(ais) => ais,
        Err(_) => return HttpResponse::InternalServerError().json(ErrMsg::new_from_scratch(
            "Number of AIS must be positive number"))
    };

    if author_name.is_empty() {
        return HttpResponse::BadRequest().json(ErrMsg::new_from_scratch(
            "Name of the player cannot be empty",
        ));
    }

    let game = Game::new_with_ai(author_name.clone(), ais);
    let game_id = game.id.clone();
    let jwt = auth_service.generate_jwt(author_name, &game_id);

    let mut game_repo = match safe_lock(&game_repo) {
        Err(response) => return response,
        Ok(repo) => repo,
    };

    game_repo.add_game(game);

    HttpResponse::Created().json(SuccessResponse {
        game_id,
        token: jwt,
    })
}
