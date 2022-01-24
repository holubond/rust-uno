use std::sync::{Arc, Mutex};
use actix_web::{HttpResponse, Responder, web};
use crate::{AddressRepo, InMemoryGameRepo};
use crate::cards::card::{Card, CardColor};

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayCardData {
    card: Card,
    newColor: CardColor
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageResponse {
    message: String,
}

#[post("/game/{gameID}/playCard")]
pub async fn create_game(
    game_repo: web::Data<Arc<Mutex<InMemoryGameRepo>>>,
    address_repo: web::Data<Arc<AddressRepo>>,
    body: web::Json<PlayCardData>,
    params: web::Path<String>,
) -> impl Responder {
    let gameID = params.into_inner();

    let game = match game_repo.lock().unwrap().find_game_by_id(gameID.clone()).clone() {
        Some(game) => game.clone(),
        _=> return HttpResponse::NotFound().json(MessageResponse {message:"Game not found".to_string()})
    };
}