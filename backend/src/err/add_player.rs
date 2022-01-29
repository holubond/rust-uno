use crate::err::status::CreateStatusError;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq)]
pub enum AddPlayerError {
    AlreadyExists(String),
    CreateStatusError(CreateStatusError),
}

impl Error for AddPlayerError {}

impl Display for AddPlayerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use AddPlayerError::*;

        match self {
            AlreadyExists(name) => {
                write!(f, "Player of name {} already exists in this game.", name)
            }
            CreateStatusError(err) => write!(f, "{}", err),
        }
    }
}

impl From<CreateStatusError> for AddPlayerError {
    fn from(e: CreateStatusError) -> Self {
        AddPlayerError::CreateStatusError(e)
    }
}
