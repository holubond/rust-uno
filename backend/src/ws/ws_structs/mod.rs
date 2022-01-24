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
