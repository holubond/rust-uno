use crate::cards::card::Card;
use crate::ws::ws_structs::WsMessageWrapper;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PenaltyWSMessage {
    #[serde(rename = "type")]
    typee: String,
    who: String,
    cards: Vec<Card>,
}

impl PenaltyWSMessage {
    pub fn new(penalized_player_name: String, gained_cards: Vec<Card>) -> PenaltyWSMessage {
        PenaltyWSMessage {
            typee: "PENALTY".into(),
            who: penalized_player_name,
            cards: gained_cards,
        }
    }
}

impl WsMessageWrapper for PenaltyWSMessage {}
