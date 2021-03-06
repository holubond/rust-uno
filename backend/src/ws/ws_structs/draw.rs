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
    pub fn new(
        drawing_player_name: String,
        next_player_name: String,
        cards_drawn: usize,
    ) -> DrawWSMessage {
        DrawWSMessage {
            typee: "DRAW".into(),
            who: drawing_player_name,
            next: next_player_name,
            cards: cards_drawn,
        }
    }
}

impl WsMessageWrapper for DrawWSMessage {}

#[derive(Serialize, Deserialize)]
pub struct DrawMeWSMessage {
    #[serde(rename = "type")]
    typee: String,
    next: String,
    cards: Vec<Card>,
}

impl DrawMeWSMessage {
    pub fn new(next_player_name: String, cards_drawn: Vec<Card>) -> DrawMeWSMessage {
        DrawMeWSMessage {
            typee: "DRAW ME".into(),
            next: next_player_name,
            cards: cards_drawn,
        }
    }
}

impl WsMessageWrapper for DrawMeWSMessage {}
