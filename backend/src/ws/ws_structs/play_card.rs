use crate::cards::card::Card;
use crate::ws::ws_structs::WsMessageWrapper;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PlayCardWSMessage {
    #[serde(rename = "type")]
    typee: String,
    who: String,
    next: String,
    card: Card,
}

impl PlayCardWSMessage {
    pub fn new(target_player: String, next_player: String, card_drawn: Card) -> PlayCardWSMessage {
        PlayCardWSMessage {
            typee: "PLAY CARD".into(),
            who: target_player,
            next: next_player,
            card: card_drawn,
        }
    }
}

impl WsMessageWrapper for PlayCardWSMessage {}
