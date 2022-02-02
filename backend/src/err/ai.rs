use crate::err::draw_cards::PlayerDrawError;
use crate::err::play_card::PlayCardError;
use crate::err::status::CreateStatusError;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum AiError {
    PlayCard(PlayCardError),
    DrawCard(PlayerDrawError),
    CreateStatusError(CreateStatusError),
}

impl Error for AiError {}

impl Display for AiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use AiError::*;

        match self {
            PlayCard(err) => write!(f, "{}", err),
            DrawCard(err) => write!(f, "{}", err),
            CreateStatusError(err) => write!(f, "{}", err),
        }
    }
}

impl From<PlayCardError> for AiError {
    fn from(e: PlayCardError) -> Self {
        AiError::PlayCard(e)
    }
}

impl From<PlayerDrawError> for AiError {
    fn from(e: PlayerDrawError) -> Self {
        AiError::DrawCard(e)
    }
}

impl From<CreateStatusError> for AiError {
    fn from(e: CreateStatusError) -> Self {
        AiError::CreateStatusError(e)
    }
}
