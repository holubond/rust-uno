use std::error::Error;

use actix_web::HttpResponse;
use serde::Serialize;

use crate::repo::game_repo::GameRepoError;

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

    pub fn new_from_scratch(type_of_error: &str, message: String) -> Self {
        Self {
            type_of_error: type_of_error.into(),
            message,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct ErrMsg {
    msg: String,
}

impl ErrMsg {
    pub fn new(err: impl Error) -> Self {
        Self {
            msg: err.to_string(),
        }
    }

    pub fn new_from_scratch(message: &str) -> Self {
        Self {
            msg: message.into(),
        }
    }
}

impl From<GameRepoError> for HttpResponse {
    fn from(error: GameRepoError) -> HttpResponse {
        use GameRepoError::*;
        match error {
            GameNotFound(_) => HttpResponse::NotFound().json(ErrMsg::new(error)),
        }
    }
}
