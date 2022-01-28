use crate::cards::card::Card;
use crate::err::draw_cards::DrawCardsError;
use crate::handler::util::response::{ErrResp, TypedErrMsg, ErrMsg};
use crate::handler::util::safe_lock::safe_lock;
use crate::{AuthService, InMemoryGameRepo};
use actix_web::{post, web, HttpRequest, HttpResponse};
use serde::Deserialize;
use serde::Serialize;
use std::sync::{Arc, Mutex};

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageResponse {
    cards: Vec<Card>,
    next: String,
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
            PlayerTurnError(e) => 
                HttpResponse::Conflict().json( 
                    TypedErrMsg::not_your_turn(e)
                ),

            PlayerExistError(e) => 
                HttpResponse::BadRequest().json( 
                    ErrMsg::from(e) 
                ),

            PlayerCanPlayInstead => 
                HttpResponse::Conflict().json( 
                    TypedErrMsg::cannot_draw(error)
                ),
        }
    }
}
