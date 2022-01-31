use actix_web::{put, web, HttpResponse};
use serde::Deserialize;

use crate::game_server_repo::{AddGameServerResult, GameServerRepo};

#[derive(Deserialize, Debug)]
pub struct RequestBody {
    server: String,
}

#[put("/gameServer")]
pub async fn register_game_server(
    body: web::Json<RequestBody>,
    game_server_repo: web::Data<GameServerRepo>,
) -> HttpResponse {
    game_server_repo.add(body.server.clone()).into()
}

impl From<AddGameServerResult> for HttpResponse {
    fn from(result: AddGameServerResult) -> Self {
        use self::AddGameServerResult::*;
        match result {
            CouldNotGetLock => HttpResponse::InternalServerError()
                .body("Could not aquire lock on game server repo"),
            ServerAlreadyRegistered => HttpResponse::NoContent().finish(),
            ServerAdded => HttpResponse::Created().finish(),
        }
    }
}
