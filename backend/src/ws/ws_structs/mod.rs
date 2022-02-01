use crate::gamestate::WSMessage;
use serde::Serialize;

pub(super) mod draw;
pub(super) mod finish;
pub(super) mod gained_cards;
pub(super) mod penalty;
pub(super) mod play_card;
pub(super) mod status;

pub trait WsMessageWrapper: Serialize {
    fn ws_serialize(&self) -> WSMessage {
        serde_json::to_string(self).unwrap() // safe since WsMessageWrapper requires self to also impl Serialize
    }
}
