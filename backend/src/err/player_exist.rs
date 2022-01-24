use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum PlayerExistError {
    NoSuchPlayer(String)
}

impl Error for PlayerExistError {}

impl Display for PlayerExistError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use PlayerExistError::*;

        match self {
            NoSuchPlayer(name) => write!(f, "Player of name {} does not exist!", name),
        }
    }
}
