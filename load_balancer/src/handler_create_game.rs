use actix_web::{
    client::Client, dev::HttpResponseBuilder, http::StatusCode, post, web, HttpResponse,
};
use serde::{Deserialize, Serialize};

use crate::{game_server_repo::{GameServerRepo, GetServerForNewGameError}, server_id::ServerId, err_msg::ErrMsg};

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestBody {
    name: String,
}

#[derive(Deserialize, Debug)]
pub struct SuccessResponseFromGameServer {
    #[serde(rename(serialize = "gameID", deserialize = "gameID"))]
    game_id: String,
    token: String,
}

#[derive(Serialize, Debug)]
pub struct SuccessResponseToClient {
    #[serde(rename(serialize = "gameID", deserialize = "gameID"))]
    game_id: String,
    server: String,
    token: String,
}

#[post("/game")]
async fn create_game(
    request_body: web::Json<RequestBody>,
    game_server_repo: web::Data<GameServerRepo>,
) -> HttpResponse {
    let (server_address, server_id) = match game_server_repo.get_server_for_new_game() {
        Err(error) => return error.into(),
        Ok(address) => address,
    };

    println!("Found server {} {}", server_address, server_id);

    let client = Client::default();

    // create request has to be sent to port 80 (we use http and emit port) as there is no SSL for server requests
    let ip = match server_address.split(":").next() {
        None => return HttpResponse::InternalServerError().json(
            ErrMsg::new("Error when splitting IP".to_string()),
        ),
        Some(ip) => ip,
    };

    let url = format!("http://{}/game", ip);

    let response = client
        .post(url)
        .header("User-Agent", "actix-web/3.0")
        .send_json(&request_body.into_inner())
        .await;

        
    let mut gs_response = match response {
        Err(error) => {
            game_server_repo.notify_about_false_game_create(server_id);
            return HttpResponse::InternalServerError().json(
                ErrMsg::new(format!("Error sending a request to the Game Server: {}", error))
            );
        }
        Ok(response) => response,
    };

    println!("Got response {:#?}", gs_response);
    
    if gs_response.status() != StatusCode::CREATED {
        game_server_repo.notify_about_false_game_create(server_id);
    }

    let gs_response_body = match gs_response.json::<SuccessResponseFromGameServer>().await {
        Err(err) => {
            game_server_repo.notify_about_false_game_create(server_id);
            return HttpResponse::ServiceUnavailable().json(
                ErrMsg::new(format!("Could not interpret response from a game server: {}", err))
            );
        }
        Ok(json) => json,
    };

    let full_game_id = ServerId::generate_full_id(gs_response_body.game_id, server_id);

    let response_body = SuccessResponseToClient {
        game_id: full_game_id,
        server: server_address,
        token: gs_response_body.token,
    };

    HttpResponseBuilder::new(gs_response.status()).json(response_body)
}

impl From<GetServerForNewGameError> for HttpResponse {
    fn from(error: GetServerForNewGameError) -> Self {
        use GetServerForNewGameError::*;
        match error {
            CouldNotGetLock => HttpResponse::InternalServerError().json(
                ErrMsg::new("Could not acquire lock.".to_string()),
            ),
            NoServerAvailable => HttpResponse::NotFound().json(
                ErrMsg::new("No server is currently available.".to_string()),
            ),
        }
    }
}
