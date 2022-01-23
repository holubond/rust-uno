use crate::gamestate::game::Game;
use serde::Serialize;
use crate::gamestate::WSMessage;

pub(super) mod status;
pub(super) mod draw;
pub(super) mod play_card;
pub(super) mod finish;

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

fn find_author_name(game: &Game) -> String {
    match game.find_author() {
        None => "UnknownAuthor".into(),
        Some(author) => author.name(),
    }
}

fn get_current_player_name(game: &Game) -> String {
    match game.get_current_player() {
        None => "UnknownCurrentPlayer".into(),
        Some(player) => player.name(),
    }
}