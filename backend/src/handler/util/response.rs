use actix_web::HttpResponse;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct ErrResp {
    msg: String,
}

impl ErrResp {
    pub fn new(message: &str) -> Self {
        Self { msg: message.into() }
    }

    pub fn game_not_found(id: String) -> HttpResponse {
        HttpResponse::NotFound().json( Self{ msg: format!("Game with id '{}' not found", id)})
    }
}