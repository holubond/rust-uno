use crate::ws::ws_structs::WsMessageWrapper;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GainedCardsWSMessage {
    #[serde(rename = "type")]
    typee: String,
    who: String,
    number: usize,
}

impl GainedCardsWSMessage {
    pub fn new(penalized_player_name: String, gained_cards_count: usize) -> GainedCardsWSMessage {
        GainedCardsWSMessage {
            typee: "GAINED CARD".into(),
            who: penalized_player_name,
            number: gained_cards_count,
        }
    }
}

impl WsMessageWrapper for GainedCardsWSMessage {}
