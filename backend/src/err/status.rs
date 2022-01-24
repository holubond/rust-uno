use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub enum CreateStatusError {
    AuthorNotFound,
    CurrentPlayerNotFound,
}

impl Error for CreateStatusError {}

impl Display for CreateStatusError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use CreateStatusError::*;

        write!(
            f,
            "{}",
            match self {
                AuthorNotFound => "Impossible: cannot find author of game",
                CurrentPlayerNotFound => "Impossible: cannot find current player of game",
            }
        )
    }
}
