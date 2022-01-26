use std::sync::{Arc, Mutex};
use actix_web::{HttpResponse, Responder, web, post, HttpRequest};
use crate::{AddressRepo, AuthorizationRepo, InMemoryGameRepo};
use crate::cards::card::{Card, CardColor};
use crate::gamestate::game::GameStatus;
use serde::Deserialize;
use serde::Serialize;
use crate::err::play_card::PlayCardError;


#[derive(Serialize, Deserialize, Debug)]
pub struct PlayCardData {
    card: Card,
    #[serde(rename(serialize = "newColor", deserialize = "newColor"))]
    new_color: CardColor,
    #[serde(rename(serialize = "saidUno", deserialize = "saidUno"))]
    said_uno: bool
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageResponse {
    message: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct TypeMessageResponse {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    type_of_error: String,
    message: String,
}

#[post("/game/{gameID}/playCard")]
pub async fn play_card(
    game_repo: web::Data<Arc<Mutex<InMemoryGameRepo>>>,
    authorization_repo: web::Data<Arc<AuthorizationRepo>>,
    body: web::Json<PlayCardData>,
    request: HttpRequest,
    params: web::Path<String>,
) -> impl Responder {
    let game_id = params.into_inner();
    let card = &body.card;
    let new_color = body.new_color;
    let said_uno = body.said_uno;
    let mut game_repo = game_repo.lock().unwrap();

    let game = match game_repo.find_game_by_id_mut(&game_id) {
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
        _ => return HttpResponse::InternalServerError().json(MessageResponse{message: "Game does not have player".to_string()})
    };
    if !authorization_repo.verify_jwt(username.clone(),game_id, claims) {
        return HttpResponse::Forbidden().json(MessageResponse {message:"Token does not prove client is the Author".to_string()});
    }

    if said_uno && player.get_card_count() > 1{
        return HttpResponse::BadRequest().json(MessageResponse{message: "Cannot play UNO".to_string()})
    }

    if game.status() != GameStatus::Running {
        return HttpResponse::Conflict().json(TypeMessageResponse{ type_of_error: "GAME_NOT_RUNNING".to_string(), message: "Game is not running".to_string() });
    }
    return match game.play_card(username, card.clone(), Option::Some(new_color),said_uno) {
        Err(PlayCardError::PlayerHasNoSuchCard(x)) =>
            HttpResponse::Conflict().json(TypeMessageResponse{ type_of_error: "CARD_NOT_IN_HAND".to_string(), message: PlayCardError::PlayerHasNoSuchCard(x).to_string()}),
        Err(PlayCardError::CardCannotBePlayed(x,y)) =>
            HttpResponse::Conflict().json(TypeMessageResponse{ type_of_error: "CANNOT_PLAY_THIS".to_string(), message: PlayCardError::CardCannotBePlayed(x,y).to_string()}),
        Err(PlayCardError::PlayerTurnError(x)) =>
            HttpResponse::Conflict().json(TypeMessageResponse{ type_of_error: "NOT_YOUR_TURN".to_string(), message: PlayCardError::PlayerTurnError(x).to_string()}),
        Err(PlayCardError::PlayerExistError(x)) =>
            HttpResponse::NotFound().json(MessageResponse{ message: PlayCardError::PlayerExistError(x).to_string()}),
        Err(PlayCardError::CreateStatusError(x)) =>
            HttpResponse::InternalServerError().json(MessageResponse{ message: x.to_string() }),
        Err(PlayCardError::SaidUnoWhenShouldNotHave) =>
            HttpResponse::BadRequest().json(MessageResponse{ message: "Cannot say UNO".to_string()}),
        Ok(_) => HttpResponse::NoContent().finish(),
    }

}