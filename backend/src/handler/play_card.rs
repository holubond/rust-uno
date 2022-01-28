use crate::cards::card::{Card, CardColor};
use crate::err::play_card::PlayCardError;
use crate::gamestate::game::GameStatus;
use crate::handler::util::response::{TypedErrMsg, ErrMsg};
use crate::handler::util::safe_lock::safe_lock;
use crate::{AuthService, InMemoryGameRepo};
use actix_web::{post, web, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;
use serde::Serialize;
use std::sync::Mutex;

#[derive(Deserialize, Debug)]
pub struct RequestBody {
    card: Card,
    #[serde(rename(serialize = "newColor", deserialize = "newColor"))]
    new_color: Option<CardColor>,
    #[serde(rename(serialize = "saidUno", deserialize = "saidUno"))]
    said_uno: bool,
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
    route_params: web::Path<String>,
    request: HttpRequest,
    request_body: web::Json<RequestBody>,
    auth_service: web::Data<AuthService>,
    game_repo: web::Data<Mutex<InMemoryGameRepo>>,
) -> impl Responder {
    match play_card_response(route_params, request, request_body, auth_service, game_repo) {
        Err(response) => response,
        Ok(response) => response,
    }
}

fn play_card_response(
    route_params: web::Path<String>,
    request: HttpRequest,
    request_body: web::Json<RequestBody>,
    auth_service: web::Data<AuthService>,
    game_repo: web::Data<Mutex<InMemoryGameRepo>>,
) -> Result<HttpResponse, HttpResponse> {
    let game_id = route_params.into_inner();
    let card = &request_body.card;
    let maybe_new_color = request_body.new_color;
    let said_uno = request_body.said_uno;

    let (game_id_from_token, player_name) = auth_service.extract_data(&request)?;

    let game_id = game_id_from_token.check(game_id)?;

    let mut game_repo = safe_lock(&game_repo)?;

    let game = game_repo.get_game_by_id_mut(game_id.clone())?;

    if game.status() != GameStatus::Running {
        return Err( HttpResponse::Conflict().json( 
            TypedErrMsg::new_from_scratch(
                "GAME_NOT_RUNNING", 
                format!("The game with id '{}' is not running", game_id)
            )
        ))
    }

    game.play_card(player_name, card.clone(), maybe_new_color, said_uno)?;

    Ok(HttpResponse::NoContent().finish())
}

impl From<PlayCardError> for HttpResponse {
    fn from(error: PlayCardError) -> HttpResponse {
        use PlayCardError::*;
        match error {
            PlayerHasNoSuchCard(_) =>
                HttpResponse::Conflict().json(
                    TypedErrMsg::new("CARD_NOT_IN_HAND", error)
                ),
            CardCannotBePlayed(_, _) =>
                HttpResponse::Conflict().json(
                    TypedErrMsg::new("CANNOT_PLAY_THIS", error)
                ),
            PlayerTurnError(_) =>
                HttpResponse::Conflict().json(
                    TypedErrMsg::new("NOT_YOUR_TURN", error)
                ),
            PlayerExistError(_) => 
                HttpResponse::NotFound().json(
                    ErrMsg::new(error)
                ),
            CreateStatusError(_) =>
                HttpResponse::InternalServerError().json(
                    ErrMsg::new(error)
                ),
            SaidUnoWhenShouldNotHave =>
                HttpResponse::BadRequest().json(
                    ErrMsg::new(error)
                )
        }
    }
}
