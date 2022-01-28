use std::error::Error;

use actix_web::HttpResponse;
use serde::Serialize;

#[derive(Serialize)]
pub struct TypedMsg {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    type_of_error: String,
    message: String,
}

impl TypedMsg {
    pub fn not_your_turn(error: impl Error) -> Self {
        Self{type_of_error: "NOT_YOUR_TURN".into(), message: error.to_string()}
    }
    
    pub fn cannot_draw(error: impl Error) -> Self {
        Self{type_of_error: "CANNOT_DRAW".into(), message: error.to_string()}
    }
}

#[derive(Serialize, Debug)]
pub struct ErrResp {
    msg: String,
}


impl ErrResp {
    pub fn new(message: &str) -> Self {
        Self { msg: message.into() }
    }

    pub fn from(err: impl Error) -> Self {
        Self {msg: err.to_string()}
    }

    pub fn game_not_found(id: String) -> HttpResponse {
        HttpResponse::NotFound().json( Self{ msg: format!("Game with id '{}' not found", id)})
    }

    pub fn jwt_game_id_does_not_match() -> HttpResponse {
        HttpResponse::Forbidden().json( ErrResp::new("Game id in the url does not match the one in JWT") )
    }

    pub fn game_has_no_current_player() -> HttpResponse {
        HttpResponse::InternalServerError().json( ErrResp::new("Current player not found") )
    }
}
