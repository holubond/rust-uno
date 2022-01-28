use crate::gamestate::game::Game;
use crate::handler::service::auth::AuthService;
use crate::InMemoryGameRepo;
use crate::handler::util::response::ErrMsg;
use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;
use serde::Serialize;
use std::sync::Mutex;

use super::util::safe_lock::safe_lock;

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestBody {
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameCreateResponse {
    #[serde(rename(serialize = "gameID", deserialize = "gameID"))]
    game_id: String,
    server: String,
    token: String,
}

#[post("/game")]
pub async fn create_game(
    game_repo: web::Data<Mutex<InMemoryGameRepo>>,
    authorization_repo: web::Data<AuthService>,
    body: web::Json<RequestBody>,
) -> impl Responder {
    let author_name = &body.name;

    if author_name.is_empty() {
        return HttpResponse::BadRequest().json(
            ErrMsg::new("Name of the player cannot be empty")
        );
    }

    let game = Game::new(author_name.clone());
    let game_id = game.id.clone();
    let jwt = authorization_repo.generate_jwt(author_name, &game_id);

    let mut game_repo = match safe_lock(&game_repo) {
        Err(response) => return response,
        Ok(repo) => repo,
    };
    
    game_repo.add_game(game);

    HttpResponse::Created().json(GameCreateResponse {
        game_id: game_id,
        server: "TODO: implement".to_string(),
        token: jwt,
    })
}
