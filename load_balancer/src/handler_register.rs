use actix_web::{put, web, HttpRequest, HttpResponse};
use serde::Deserialize;

use crate::game_server_repo::{AddGameServerResult, GameServerRepo};

#[derive(Deserialize, Debug)]
pub struct RequestBody {
    port: String,
}

#[put("/gameServer")]
pub async fn register_game_server(
    request: HttpRequest,
    body: web::Json<RequestBody>,
    game_server_repo: web::Data<GameServerRepo>,
) -> HttpResponse {

    let conn_info = request.connection_info();
    let ip_with_outgoing_port = match conn_info.realip_remote_addr() {
        None => return HttpResponse::InternalServerError().body("Cannot resolve IP from request"),
        Some(ip) => ip,
    };

    let ip = match get_receiver_addr(ip_with_outgoing_port, body.port.clone()) {
        Err(response) => return response,
        Ok(ip) => ip,
    };

    game_server_repo.add(ip).into()
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

fn get_receiver_addr(ip_port: &str, port: String) -> Result<String, HttpResponse> {

    let ip = match ip_port.split(":").next() {
        None => return Err(HttpResponse::InternalServerError().body(
            "Error when splitting IP"
        )),
        Some(x) => x,
    };

    Ok(format!("{}:{}", ip, port))
}