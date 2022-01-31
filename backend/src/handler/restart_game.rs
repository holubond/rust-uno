use super::util::response::ErrMsg;
use crate::err::game_start::GameStartError;
use crate::handler::util::safe_lock::safe_lock;
use crate::{AuthService, InMemoryGameRepo};
use actix_web::{post, web, HttpRequest, HttpResponse, Responder};
use std::sync::Mutex;

#[post("game/{gameID}/statusRunning")]
pub async fn start_game(
    route_params: web::Path<String>,
    request: HttpRequest,
    auth_service: web::Data<AuthService>,
    game_repo: web::Data<Mutex<InMemoryGameRepo>>,
) -> impl Responder {
    match start_game_response(route_params, request, auth_service, game_repo) {
        Err(response) => response,
        Ok(response) => response,
    }
}

pub fn start_game_response(
    route_params: web::Path<String>,
    request: HttpRequest,
    auth_service: web::Data<AuthService>,
    game_repo: web::Data<Mutex<InMemoryGameRepo>>,
) -> Result<HttpResponse, HttpResponse> {
    let game_id = route_params.into_inner();

    let (game_id_from_token, player_name_from_token) = auth_service.extract_data(&request)?;

    let game_id = game_id_from_token.check(game_id)?;

    let mut game_repo = safe_lock(&game_repo)?;

    let game = game_repo.get_game_by_id_mut(game_id)?;

    let author_name = match game.find_author() {
        None => {
            return Err(HttpResponse::InternalServerError()
                .json(ErrMsg::new_from_scratch("Author of the game not found")))
        }
        Some(author) => author.name(),
    };

    player_name_from_token.check(&author_name)?;

    game.start()?;

    Ok(HttpResponse::NoContent().finish())
}

impl From<GameStartError> for HttpResponse {
    fn from(error: GameStartError) -> Self {
        use GameStartError::*;
        match error {
            DeckEmptyWhenStartingGame => {
                HttpResponse::InternalServerError().json(ErrMsg::new(error))
            }
            GameAlreadyStarted => HttpResponse::Conflict().json(ErrMsg::new(error)),
            CreateStatusError(_) => HttpResponse::InternalServerError().json(ErrMsg::new(error)),
            _ => {
                todo!("React to ChainedAiError")
            }
        }
    }
}
