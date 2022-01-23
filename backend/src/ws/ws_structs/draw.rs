use crate::cards::card::Card;
use crate::ws::ws_structs::WsMessageWrapper;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DrawWSMessage {
    #[serde(rename = "type")]
    typee: String,
    who: String,
    next: String,
    cards: usize,
}

impl DrawWSMessage {
    pub fn new(target_player: String, next_player: String, cards_drawn: usize) -> DrawWSMessage {
        DrawWSMessage {
            typee: "DRAW".into(),
            who: target_player,
            next: next_player,
            cards: cards_drawn,
        }
    }
}

impl WsMessageWrapper for DrawWSMessage {}
