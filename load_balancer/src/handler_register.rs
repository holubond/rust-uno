use actix_web::{HttpResponse, HttpRequest, web, put};

use crate::game_server_repo::{GameServerRepo, AddGameServerResult};

#[put("/gameServer")]
pub async fn register_game_server(
    request: HttpRequest,
    game_server_repo: web::Data<GameServerRepo>,
) -> HttpResponse {
    let conn_info = request.connection_info();
    let ip = match conn_info.realip_remote_addr() {
        None => return HttpResponse::InternalServerError().body(
            "Cannot resolve IP from request"
        ),
        Some(ip) => ip,
    };

    game_server_repo.add(ip).into()
}

impl From<AddGameServerResult> for HttpResponse {
    fn from(result: AddGameServerResult) -> Self {
        use self::AddGameServerResult::*;
        match result {
            CouldNotGetLock => 
                HttpResponse::InternalServerError().body(
                    "Could not aquire lock on game server repo"
                ),
            ServerAlreadyRegistered(position) => 
                HttpResponse::Ok().body(format!("{}", position)),
            ServerAdded(position) => 
                HttpResponse::Created().body(format!("{}", position)),
        }
    }
}