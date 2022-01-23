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
    pub fn new(playing_player_name: String, next_player_name: String, card_drawn: Card) -> PlayCardWSMessage {
        PlayCardWSMessage {
            typee: "PLAY CARD".into(),
            who: playing_player_name,
            next: next_player_name,
            card: card_drawn,
        }
    }
}

impl WsMessageWrapper for PlayCardWSMessage {}
