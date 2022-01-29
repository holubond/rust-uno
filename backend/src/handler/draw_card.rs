use crate::cards::card::Card;
use crate::err::draw_cards::PlayerDrawError;
use crate::handler::util::response::{ErrMsg, TypedErrMsg};
use crate::handler::util::safe_lock::safe_lock;
use crate::{AuthService, InMemoryGameRepo};
use actix_web::{post, web, HttpRequest, HttpResponse};
use serde::Serialize;
use std::sync::Mutex;

#[derive(Serialize, Debug)]
pub struct SuccessResponse {
    cards: Vec<Card>,
    next: String,
}

#[post("/game/{gameID}/drawnCards")]
pub async fn draw_card(
    route_params: web::Path<String>,
    request: HttpRequest,
    auth_service: web::Data<AuthService>,
    game_repo: web::Data<Mutex<InMemoryGameRepo>>,
) -> HttpResponse {
    let game_id = route_params.into_inner();
    match draw_card_response(game_id, game_repo, auth_service, request) {
        Ok(r) => r,
        Err(r) => r,
    }
}

fn draw_card_response(
    game_id: String,
    game_repo: web::Data<Mutex<InMemoryGameRepo>>,
    auth_service: web::Data<AuthService>,
    request: HttpRequest,
) -> Result<HttpResponse, HttpResponse> {
    let mut game_repo = safe_lock(&game_repo)?;

    let (game_id_from_token, player_name) = auth_service.extract_data(&request)?;

    let game_id = game_id_from_token.check(game_id)?;

    let game = game_repo.get_game_by_id_mut(game_id)?;

    let drawn_cards = game.draw_cards(player_name.into_inner())?;

    let next_player = match game.get_current_player() {
        None => return Err(
            HttpResponse::InternalServerError().json(
                ErrMsg::new_from_scratch("Current player not found")
            )
        ),
        Some(player) => player,
    };

    Ok(HttpResponse::Ok().json(SuccessResponse {
        cards: drawn_cards,
        next: next_player.name(),
    }))
}

impl From<PlayerDrawError> for HttpResponse {
    fn from(error: PlayerDrawError) -> HttpResponse {
        use PlayerDrawError::*;
        match error {
            TurnError(_) =>
                HttpResponse::Conflict().json(
                    TypedErrMsg::new("NOT_YOUR_TURN", error)
                ),
            PlayerExistError(_) => 
                HttpResponse::BadRequest().json(
                    ErrMsg::new(error)
                ),
            CanPlayInstead => 
                HttpResponse::Conflict().json(
                    TypedErrMsg::new("CANNOT_DRAW", error)
                ),
        }
    }
}
