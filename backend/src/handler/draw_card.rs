use crate::cards::card::Card;
use crate::err::draw_cards::DrawCardsError;
use crate::gamestate::game::GameStatus;
use crate::{AuthorizationRepo, InMemoryGameRepo};
use actix_web::{post, web, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;
use serde::Serialize;
use std::sync::{Arc, Mutex};

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorMessageResponse {
    message: String,
}
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

    let mut game_repo = game_repo.lock().unwrap();

    let game = match game_repo.find_game_by_id(&game_id) {
        Some(game) => game,
        _ => {
            return HttpResponse::NotFound().json(ErrorMessageResponse {
                message: "Game not found".to_string(),
            })
        }
    };

    let jwt = authorization_repo.parse_jwt(request);

    let jwt = match jwt {
        Ok(jwt) => jwt.to_string(),
        _ => {
            return HttpResponse::Unauthorized().json(ErrorMessageResponse {
                message: "No auth token provided by the client".to_string(),
            })
        }
    };

    let claims = match authorization_repo.valid_jwt(&jwt) {
        Ok(claims) => claims,
        _ => {
            return HttpResponse::Unauthorized().json(ErrorMessageResponse {
                message: "Token is not valid".to_string(),
            })
        }
    };
    let username = authorization_repo.user_from_claims(&claims);

    let player = match game.find_player(username.clone()) {
        Some(player) => player,
        _ => {
            return HttpResponse::InternalServerError().json(ErrorMessageResponse {
                message: "Game does not have player".to_string(),
            })
        }
    };
    if !authorization_repo.verify_jwt(username.clone(), game_id, claims) {
        return HttpResponse::Forbidden().json(ErrorMessageResponse {
            message: "Token does not prove client is the Author".to_string(),
        });
    }

    if game.status() != GameStatus::Running {
        return HttpResponse::Conflict().json(MessageResponseType {
            type_of_error: TypeOfError::GameNotRunning.into_response_string(),
            message: "Game is not running ".to_string(),
        });
    }

    return match game.draw_cards(username.clone()) {
        Ok(drawn_cards) => {
            let next_player = match game.get_current_player() {
                None => {
                    return HttpResponse::InternalServerError().json(ErrorMessageResponse {
                        message: "Current player not found".to_string(),
                    })
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
        _ => HttpResponse::InternalServerError().json(ErrorMessageResponse {
            message: "Error occurred during draw card".to_string(),
        }),
    };
}
