use crate::cards::card::Card;
use crate::err::draw_cards::DrawCardsError;
use crate::gamestate::game::GameStatus;
use crate::handler::util::response::ErrResp;
use crate::handler::util::safe_lock::safe_lock;
use crate::{AuthService, InMemoryGameRepo};
use actix_web::{post, web, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;
use serde::Serialize;
use std::sync::{Arc, Mutex};

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageResponse {
    cards: Vec<Card>,
    next: String,
}

pub enum TypeOfError {
    GameNotRunning,
    CannotDraw,
    NotYourTurn,
    PlayerExist,
}

impl TypeOfError {
    fn into_response_string(&self) -> String {
        match self {
            TypeOfError::GameNotRunning => "GAME_NOT_RUNNING".to_string(),
            TypeOfError::CannotDraw => "CANNOT_DRAW".to_string(),
            TypeOfError::NotYourTurn => "NOT_YOUR_TURN".to_string(),
            TypeOfError::PlayerExist => "PLAYER_DOES_NOT_EXIST".to_string(),
        }
    }
}

#[derive(Serialize)]
pub struct MessageResponseType {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    type_of_error: String,
    message: String,
}

#[post("/game/{gameID}/drawnCards")]
pub async fn draw_card(
    game_repo: web::Data<Arc<Mutex<InMemoryGameRepo>>>,
    authorization_repo: web::Data<Arc<AuthService>>,
    request: HttpRequest,
    params: web::Path<String>,
) -> HttpResponse {
    let game_id = params.into_inner();
    match draw_card_response(game_id, game_repo, authorization_repo, request) {
        Ok(r) => r,
        Err(r) => r
    }
}

fn draw_card_response(
    game_id: String,
    game_repo: web::Data<Arc<Mutex<InMemoryGameRepo>>>,
    authorization_repo: web::Data<Arc<AuthService>>,
    request: HttpRequest,
) -> Result<HttpResponse, HttpResponse> {
    let mut game_repo = safe_lock(&game_repo)?;

    let (game_id_from_token, player_name) = authorization_repo.extract_data(&request)?;

    let game_id = game_id_from_token.check(game_id)?;
    
    let game = match game_repo.find_game_by_id_mut(&game_id) {
        None => return Err(ErrResp::game_not_found(game_id)),
        Some(game) => game,
    };

    let drawn_cards = game.draw_cards(player_name.clone())?;

    let next_player = match game.get_current_player() {
        None => return Err(ErrResp::game_has_no_current_player()),
        Some(player) => player,
    };

    Ok(HttpResponse::Ok().json(MessageResponse {
        cards: drawn_cards,
        next: next_player.name(),
    }))
}

impl From<DrawCardsError> for HttpResponse {
    fn from(error: DrawCardsError) -> HttpResponse {
        use DrawCardsError::*;
        match error {
            PlayerTurnError(e) => HttpResponse::Conflict().json(MessageResponseType{ type_of_error: TypeOfError::NotYourTurn.into_response_string(), message: e.to_string() }),
            PlayerExistError(e) => HttpResponse::BadRequest().json(ErrResp::new(&format!("{}",DrawCardsError::PlayerExistError(e)))),

            PlayerCanPlayInstead => HttpResponse::Conflict().json(MessageResponseType{ type_of_error: TypeOfError::CannotDraw.into_response_string(), message: format!("{}",DrawCardsError::PlayerCanPlayInstead) }),
            PlayerMustPlayInstead(e) => HttpResponse::Conflict().json(MessageResponseType {
                type_of_error: TypeOfError::CannotDraw.into_response_string(),
                message: format!("{}",DrawCardsError::PlayerMustPlayInstead(e))}),
        }
    }
}
