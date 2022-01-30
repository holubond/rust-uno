use actix_web::{get, web, HttpResponse};

use crate::{game_server_repo::{GameServerRepo, GetGameServerResult}, server_id::ServerId};

#[get("/gameServer/{gameID}")]
pub async fn get_game_server(
    route_params: web::Path<String>,
    game_server_repo: web::Data<GameServerRepo>,
) -> HttpResponse {
    let game_id = route_params.into_inner();

    let server_id = match ServerId::from(game_id) {
        Err(response) => return response,
        Ok(id) => id,
    };

    game_server_repo.get(server_id).into()
}

impl From<GetGameServerResult> for HttpResponse {
    fn from(result: GetGameServerResult) -> Self {
        use GetGameServerResult::*;
        match result {
            CouldNotGetLock => 
                HttpResponse::InternalServerError().body(
                    "Could not aquire lock on game server repo"
                ),
            Found(server_address) =>
                HttpResponse::Ok().body(
                    server_address
                ),
            NotFound => 
                HttpResponse::NotFound().finish(),
        }
    }
}
