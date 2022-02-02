use crate::err::ai::AiError;
use crate::err::player_exist::PlayerExistError;
use crate::err::player_turn::PlayerTurnError;
use crate::err::status::CreateStatusError;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum PlayerDrawError {
    TurnError(PlayerTurnError),
    PlayerExistError(PlayerExistError),
    ChainedAiError,
    CanPlayInstead,
    CreateStatusError(CreateStatusError),
}

impl Error for PlayerDrawError {}

impl Display for PlayerDrawError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use PlayerDrawError::*;

        match self {
            TurnError(err) => write!(f, "{}", err),
            PlayerExistError(err) => write!(f, "{}", err),
            CreateStatusError(err) => write!(f, "{}", err),
            CanPlayInstead => write!(f, "No need to draw, playing a card is possible"),
            ChainedAiError => write!(f, "Error occurred when AI played"),
        }
    }
}

impl From<PlayerTurnError> for PlayerDrawError {
    fn from(e: PlayerTurnError) -> Self {
        PlayerDrawError::TurnError(e)
    }
}

impl From<PlayerExistError> for PlayerDrawError {
    fn from(e: PlayerExistError) -> Self {
        PlayerDrawError::PlayerExistError(e)
    }
}

impl From<AiError> for PlayerDrawError {
    fn from(_e: AiError) -> Self {
        PlayerDrawError::ChainedAiError
    }
}

impl From<CreateStatusError> for PlayerDrawError {
    fn from(e: CreateStatusError) -> Self {
        PlayerDrawError::CreateStatusError(e)
    }
}
