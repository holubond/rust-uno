use crate::gamestate::game::GameStatus;
use crate::{AuthorizationRepo, InMemoryGameRepo};
use actix_web::{post, web, HttpRequest, HttpResponse, Responder};
use actix_web_httpauth::headers::authorization::{Authorization, Bearer};
use std::sync::{Arc, Mutex};
use actix_web::http::header::Header;

#[post("game/{gameID}/statusRunning")]
pub async fn start_game(
    data: web::Data<Arc<Mutex<InMemoryGameRepo>>>,
    authorization_repo: web::Data<Arc<AuthorizationRepo>>,
    request: HttpRequest,
    params: web::Path<String>,
) -> impl Responder {
    let gameID = params.into_inner();
    if data
        .lock()
        .unwrap()
        .games
        .iter()
        .position(|x| x.id == gameID)
        .is_none()
    {
        return HttpResponse::NotFound();
    }

    let game_index = data
        .lock()
        .unwrap()
        .games
        .iter()
        .position(|x| x.id == gameID)
        .unwrap();

    if Authorization::<Bearer>::parse(&request).is_err() {
        return HttpResponse::Unauthorized();
    }
    let jwt = Authorization::<Bearer>::parse(&request)
        .unwrap()
        .to_string();
    let author_name = data.lock().unwrap().games.get(game_index).unwrap().find_author().unwrap().clone().name;
    if !authorization_repo.verify_jwt(author_name,gameID, jwt) {
        return HttpResponse::Forbidden();
    }

    if data.lock().unwrap().games.get(game_index).unwrap().status == GameStatus::Running {
        return HttpResponse::Conflict();
    }

    HttpResponse::Ok()
}
