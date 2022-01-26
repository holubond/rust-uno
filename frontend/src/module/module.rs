use crate::components::card::{CardInfo, Color};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct PlayCardRequest {
    pub(crate) card: CardInfo,
    #[serde(rename(serialize = "newColor", deserialize = "newColor"))]
    pub(crate) new_color: Option<String>,
    #[serde(rename(serialize = "saidUno", deserialize = "saidUno"))]
    pub(crate) said_uno: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MessageResponse {
    pub(crate) message: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CardConflictMessageResponse {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub typee: String,
    pub message: String,
}
