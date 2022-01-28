use std::error::Error;

use actix_web::HttpResponse;
use serde::Serialize;

#[derive(Serialize)]
pub struct TypedErrMsg {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    type_of_error: String,
    message: String,
}

impl TypedErrMsg {

    pub fn new(type_of_error: &str, error: impl Error) -> Self {
        Self {
            type_of_error: type_of_error.into(),
            message: error.to_string(),
        }
    }
    
    pub fn not_your_turn(error: impl Error) -> Self {
        Self {
            type_of_error: "NOT_YOUR_TURN".into(),
            message: error.to_string(),
        }
    }

    pub fn cannot_draw(error: impl Error) -> Self {
        Self {
            type_of_error: "CANNOT_DRAW".into(),
            message: error.to_string(),
        }
    }

    pub fn game_not_running(game_id: String) -> Self {
        Self {
            type_of_error: "GAME_NOT_RUNNING".into(),
            message: format!("The game with id '{}' is not running", game_id),
        }
    }

}

#[derive(Serialize, Debug)]
pub struct ErrMsg {
    msg: String,
}

impl ErrMsg {
    pub fn new(message: &str) -> Self {
        Self {
            msg: message.into(),
        }
    }

    pub fn from(err: impl Error) -> Self {
        Self {
            msg: err.to_string(),
        }
    }
}

pub struct ErrResp {}
impl ErrResp {
    pub fn game_not_found(id: String) -> HttpResponse {
        HttpResponse::NotFound().json(ErrMsg {
            msg: format!("Game with id '{}' not found", id),
        })
    }

    pub fn jwt_game_id_does_not_match() -> HttpResponse {
        HttpResponse::Forbidden().json(ErrMsg::new(
            "Game id in the url does not match the one in JWT",
        ))
    }

    pub fn game_has_no_current_player() -> HttpResponse {
        HttpResponse::InternalServerError().json(ErrMsg::new("Current player not found"))
    }

    pub fn game_not_running(game_id: String) -> HttpResponse {
        HttpResponse::Conflict().json( TypedErrMsg::game_not_running(game_id) )
    }
}
