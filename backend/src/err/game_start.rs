use crate::err::status::CreateStatusError;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub enum GameStartError {
    DeckEmptyWhenStartingGame,
    GameAlreadyStarted,
    CreateStatusError(CreateStatusError),
}

impl Error for GameStartError {}

impl Display for GameStartError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use GameStartError::*;

        match self {
            DeckEmptyWhenStartingGame => {
                write!(f, "Impossible: deck empty when starting game")
            }
            GameAlreadyStarted => write!(f, "Cannot start an already running game"),
            CreateStatusError(err) => write!(f, "{}", err),
        }
    }
}

impl From<CreateStatusError> for GameStartError {
    fn from(e: CreateStatusError) -> Self {
        GameStartError::CreateStatusError(e)
    }
}
