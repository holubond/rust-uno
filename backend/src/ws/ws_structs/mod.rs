use crate::err::game_start::GameStartError;
use crate::gamestate::game::Game;
use crate::gamestate::WSMessage;
use serde::Serialize;
use crate::err::status::CreateStatusError;

pub(super) mod draw;
pub(super) mod finish;
pub(super) mod play_card;
pub(super) mod status;

pub trait WsMessageWrapper: Serialize {
    fn ws_serialize(&self) -> WSMessage {
        serde_json::to_string(self).unwrap()
    }
}

fn get_finished_player_names(game: &Game) -> Vec<String> {
    game.get_finished_players()
        .iter()
        .map(|p| p.name())
        .collect()
}

fn find_author_name(game: &Game) -> Result<String, CreateStatusError> {
    match game.find_author() {
        None => Err(CreateStatusError::AuthorNotFound),
        Some(author) => Ok(author.name()),
    }
}

fn get_current_player_name(game: &Game) -> Result<String, CreateStatusError> {
    match game.get_current_player() {
        None => Err(CreateStatusError::CurrentPlayerNotFound),
        Some(player) => Ok(player.name()),
    }
}
