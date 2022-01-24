use crate::ws::ws_structs::WsMessageWrapper;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct FinishWSMessage {
    #[serde(rename = "type")]
    typee: String,
    who: String,
}

impl FinishWSMessage {
    pub fn new(finished_player_name: String) -> FinishWSMessage {
        FinishWSMessage {
            typee: "FINISH".into(),
            who: finished_player_name,
        }
    }
}

impl WsMessageWrapper for FinishWSMessage {}
