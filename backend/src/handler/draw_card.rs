use std::sync::{Arc, Mutex};
use actix_web::{HttpResponse, Responder, web};
use crate::{AddressRepo, InMemoryGameRepo};
use crate::gamestate::game::GameStatus;

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageResponse {
    message: String,
}

pub enum TypeOfError {
    GAME_NOT_RUNNING,
    CANNOT_DRAW
}

pub struct MessageResponseType {
    type_of_error: TypeOfError,
    message: String,
}

#[post("/game/{gameID}/drawnCards")]
pub async fn create_game(
    game_repo: web::Data<Arc<Mutex<InMemoryGameRepo>>>,
    address_repo: web::Data<Arc<AddressRepo>>,
    params: web::Path<String>
) -> impl Responder {
    let gameID = params.into_inner();

    let game = match game_repo.lock().unwrap().find_game_by_id(gameID.clone()).clone() {
        Some(game) => game.clone(),
        _=> return HttpResponse::NotFound().json(MessageResponse {message:"Game not found".to_string()})
    };

    let jwt = authorization_repo.parse_jwt(request);

    let jwt = match jwt {
        Ok(jwt) => jwt.to_string(),
        _ => return HttpResponse::Unauthorized().json(MessageResponse {message:"No auth token provided by the client".to_string()})
    };

    let author_name = game.find_author().unwrap().clone().name();
    if !authorization_repo.verify_jwt(author_name,gameID, jwt) {
        return HttpResponse::Forbidden().json(MessageResponse {message:"Token does not prove client is the Author".to_string()});
    }

    if game.status() != GameStatus::Running {
        return HttpResponse::Conflict().json(MessageResponseType { type_of_error: TypeOfError::GAME_NOT_RUNNING, message:"Game is not running ".to_string()});
    }

    //TODO draw card

    HttpResponse::NoContent().finish()

}