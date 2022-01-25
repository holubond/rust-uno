use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum PlayerTurnError {
    NoOneIsPlaying,
    PlayerOutOfTurn(String),
}

impl Error for PlayerTurnError {}

impl Display for PlayerTurnError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use PlayerTurnError::*;

        match self {
            NoOneIsPlaying => write!(f, "Impossible: no player is currently playing!"),
            PlayerOutOfTurn(name) => write!(f, "It is not player {}'s turn right now!", name),
        }
    }
}
