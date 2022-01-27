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
) -> impl Responder {
    let game_id = params.into_inner();

    let mut game_repo = match safe_lock(&game_repo) {
        Err(response) => return response,
        Ok(repo) => repo,
    };
 
    let (game_id_from_token, player_name) = match authorization_repo.extract_data(&request) {
        Err(response) => return response,
        Ok(data) => data,
    };

    let game_id = match game_id_from_token.check(game_id) {
        Err(response) => return response,
        Ok(id) => id,
    };
    
    let game = match game_repo.find_game_by_id_mut(&game_id) {
        None => return ErrResp::game_not_found(game_id),
        Some(game) => game,
    };

    return match game.draw_cards(player_name.clone()) {
        Ok(drawn_cards) => {
            let next_player = match game.get_current_player() {
                None => {
                    return HttpResponse::InternalServerError().json( ErrResp::new("Current player not found") )
                }
                Some(player) => player,
            };

            HttpResponse::Ok().json(MessageResponse {
                cards: drawn_cards,
                next: next_player.name(),
            })
        }
        Err(DrawCardsError::PlayerTurnError(e)) => HttpResponse::Conflict().json(MessageResponseType{ type_of_error: TypeOfError::NotYourTurn.into_response_string(), message: e.to_string() }),
        Err(DrawCardsError::PlayerExistError(e)) => HttpResponse::BadRequest().json(ErrResp::new(&format!("{}",DrawCardsError::PlayerExistError(e)))),

        Err(DrawCardsError::PlayerCanPlayInstead) => HttpResponse::Conflict().json(MessageResponseType{ type_of_error: TypeOfError::CannotDraw.into_response_string(), message: format!("{}",DrawCardsError::PlayerCanPlayInstead) }),
        Err(DrawCardsError::PlayerMustPlayInstead(e)) => HttpResponse::Conflict().json(MessageResponseType {
                type_of_error: TypeOfError::CannotDraw.into_response_string(),
                message: format!("{}",DrawCardsError::PlayerMustPlayInstead(e))}),
    };
}
