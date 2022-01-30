use actix_web::{get, web, HttpResponse};
use serde::Serialize;

use crate::{
    game_server_repo::{GameServerRepo, GetGameServerError},
    server_id::ServerId,
};

#[derive(Serialize)]
pub struct SuccessResponse {
    #[serde(rename(serialize = "gameID"))]
    game_id: String,
    server: String,
}

#[get("/gameServer/{fullGameID}")]
pub async fn get_game_server(
    route_params: web::Path<String>,
    game_server_repo: web::Data<GameServerRepo>,
) -> HttpResponse {
    let full_game_id = route_params.into_inner();

    let (server_id, game_id) = match ServerId::parse_full_id(full_game_id) {
        Err(response) => return response,
        Ok(ids) => ids,
    };

    let server_address = match game_server_repo.get(server_id) {
        Err(error) => return error.into(),
        Ok(addr) => addr,
    };

    HttpResponse::Ok().json(SuccessResponse {
        game_id,
        server: server_address,
    })
}

impl From<GetGameServerError> for HttpResponse {
    fn from(result: GetGameServerError) -> Self {
        use GetGameServerError::*;
        match result {
            CouldNotGetLock => HttpResponse::InternalServerError()
                .body("Could not aquire lock on game server repo"),
            NotFound => HttpResponse::NotFound().finish(),
        }
    }
}
