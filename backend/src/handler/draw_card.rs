use crate::cards::card::Card;
use crate::err::draw_cards::DrawCardsError;
use crate::gamestate::game::GameStatus;
use crate::handler::util::response::ErrResp;
use crate::handler::util::safe_lock::safe_lock;
use crate::{AuthorizationRepo, InMemoryGameRepo};
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
}

impl TypeOfError {
    fn into_response_string(&self) -> String {
        match self {
            TypeOfError::GameNotRunning => "GAME_NOT_RUNNING".to_string(),
            TypeOfError::CannotDraw => "CANNOT_DRAW".to_string(),
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
    authorization_repo: web::Data<Arc<AuthorizationRepo>>,
    request: HttpRequest,
    params: web::Path<String>,
) -> impl Responder {
    let game_id = params.into_inner();

    let mut game_repo = match safe_lock(&game_repo) {
        Err(response) => return response,
        Ok(repo) => repo,
    };

    let game = match game_repo.find_game_by_id(&game_id) {
        Some(game) => game,
        _ => {
            return HttpResponse::NotFound().json( ErrResp::new("Game not found") )
        }
    };

    let (game_id_from_token, player_name) = match authorization_repo.extract_data(&request) {
        Err(response) => return response,
        Ok(data) => data,
    };

    if game_id != game_id_from_token {
        return HttpResponse::Forbidden().json( ErrResp::new("Game id in the url does not match the one in JWT") )
    }

    if game.status() != GameStatus::Running {
        return HttpResponse::Conflict().json(MessageResponseType {
            type_of_error: TypeOfError::GameNotRunning.into_response_string(),
            message: "Game is not running ".to_string(),
        });
    }

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
        Err(DrawCardsError::PlayerCanPlayInstead)
        | Err(DrawCardsError::PlayerMustPlayInstead(_)) => {
            HttpResponse::Conflict().json(MessageResponseType {
                type_of_error: TypeOfError::CannotDraw.into_response_string(),
                message: "Player has to play has to play card instead".to_string(),
            })
        }
        _ => HttpResponse::InternalServerError().json( ErrResp::new("Error occurred during draw card") ),
    };
}
