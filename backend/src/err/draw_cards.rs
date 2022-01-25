use crate::cards::card::Card;
use crate::err::player_exist::PlayerExistError;
use crate::err::player_turn::PlayerTurnError;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum DrawCardsError {
    PlayerTurnError(PlayerTurnError),
    PlayerExistError(PlayerExistError),
    PlayerCanPlayInstead,
    PlayerMustPlayInstead(Card),
}

impl Error for DrawCardsError {}

impl Display for DrawCardsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use DrawCardsError::*;

        match self {
            PlayerTurnError(err) => write!(f, "{}", err),
            PlayerExistError(err) => write!(f, "{}", err),
            PlayerCanPlayInstead => write!(f, "{}", "No need to draw, playing a card is possible"),
            PlayerMustPlayInstead(top_card) => {
                write!(f, "Cannot draw, must respond to the {}", top_card)
            }
        }
    }
}

impl From<PlayerTurnError> for DrawCardsError {
    fn from(e: PlayerTurnError) -> Self {
        DrawCardsError::PlayerTurnError(e)
    }
}

impl From<PlayerExistError> for DrawCardsError {
    fn from(e: PlayerExistError) -> Self {
        DrawCardsError::PlayerExistError(e)
    }
}
