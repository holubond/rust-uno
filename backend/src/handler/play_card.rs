use std::sync::{Arc, Mutex};
use actix_web::{HttpResponse, Responder, web};
use crate::{AddressRepo, AuthorizationRepo, InMemoryGameRepo};
use crate::cards::card::{Card, CardColor};
use crate::gamestate::game::GameStatus;


#[derive(Serialize, Deserialize, Debug)]
pub struct PlayCardData {
    card: Card,
    #[serde(rename(serialize = "newColor", deserialize = "new_color"))]
    new_color: CardColor,
    #[serde(rename(serialize = "saidUno", deserialize = "said_uno"))]
    said_uno: bool
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageResponse {
    message: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct TypeMessageResponse {
    #[serde(rename(serialize = "type", deserialize = "type_of_error"))]
    type_of_error: String,
    message: String,
}

#[post("/game/{gameID}/playCard")]
pub async fn create_game(
    game_repo: web::Data<Arc<Mutex<InMemoryGameRepo>>>,
    authorization_repo: web::Data<Arc<AuthorizationRepo>>,
    address_repo: web::Data<Arc<AddressRepo>>,
    body: web::Json<PlayCardData>,
    params: web::Path<String>,
) -> impl Responder {
    let game_id = params.into_inner();
    let card = &body.card;
    let new_color = body.new_color;
    let said_uno = body.said_uno;
    let mut game_repo = game_repo.lock().unwrap();

    let game = match game_repo.find_game_by_id(&game_id) {
        Some(game) => game,
        _=> return HttpResponse::NotFound().json(MessageResponse {message:"Game not found".to_string()})
    };
    let jwt = authorization_repo.parse_jwt(request);
    let jwt = match jwt {
        Ok(jwt) => jwt.to_string(),
        _ => return HttpResponse::Unauthorized().json(MessageResponse {message:"No auth token provided by the client".to_string()})
    };
    let claims = match authorization_repo.valid_jwt(&jwt)  {
        Ok(claims) => claims,
        _ => return HttpResponse::Unauthorized().json(MessageResponse {message:"Token is not valid".to_string()})
    };
    let username = authorization_repo.user_from_claims(&claims);

    let player = match game.find_player(username.clone()) {
        Some(player) => player,
        _ => return HttpResponse::InternalServerError().json(ErrorMessageResponse{message: "Game does not have player".to_string()})
    };
    if !authorization_repo.verify_jwt(username.clone(),game_id, claims) {
        return HttpResponse::Forbidden().json(ErrorMessageResponse {message:"Token does not prove client is the Author".to_string()});
    }
}