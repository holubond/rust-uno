use crate::cards::card::Card;
use crate::err::player_exist::PlayerExistError;
use crate::err::player_turn::PlayerTurnError;
use crate::err::status::CreateStatusError;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum PlayCardError {
    PlayerTurnError(PlayerTurnError),
    PlayerExistError(PlayerExistError),
    CreateStatusError(CreateStatusError),
    PlayerHasNoSuchCard(Card),
    CardCannotBePlayed(Card, Card),
    SaidUnoWhenShouldNotHave,
}

impl Error for PlayCardError {}

impl Display for PlayCardError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use PlayCardError::*;

        match self {
            PlayerTurnError(err) => write!(f, "{}", err),
            PlayerExistError(err) => write!(f, "{}", err),
            CreateStatusError(err) => write!(f, "{}", err),
            PlayerHasNoSuchCard(card) => write!(f, "Player does not have a {}", card),
            CardCannotBePlayed(played, top) => {
                write!(f, "Cannot play a {} after a {}.", played, top)
            }
            SaidUnoWhenShouldNotHave => {
                write!(f, "UNO! was said when it shouldn't have been possible")
            }
        }
    }
}

impl From<PlayerTurnError> for PlayCardError {
    fn from(e: PlayerTurnError) -> Self {
        PlayCardError::PlayerTurnError(e)
    }
}

impl From<PlayerExistError> for PlayCardError {
    fn from(e: PlayerExistError) -> Self {
        PlayCardError::PlayerExistError(e)
    }
}

impl From<CreateStatusError> for PlayCardError {
    fn from(e: CreateStatusError) -> Self {
        PlayCardError::CreateStatusError(e)
    }
}
