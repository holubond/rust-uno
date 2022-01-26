use crate::components::card::CardInfo;
use crate::pages::game::Player;
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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LobbyStatus {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub typee: String,
    pub status: String,
    pub author: String,
    pub you: String,          //jméno hráče, který žádal o status
    pub players: Vec<String>, //jména všech hráčů
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RunningStatus {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub typee: String,
    status: String,
    author: String,
    you: String, //jméno hráče, který žádal o status
    #[serde(rename(serialize = "currentPlayer", deserialize = "currentPlayer"))]
    current_player: String, //jméno hráče na tahu
    players: Vec<Player>,
    #[serde(rename(serialize = "finishedPlayers", deserialize = "finishedPlayers"))]
    finished_players: Vec<String>, //hráči, kteří už se zbavili karet v pořadí, v jakém skončili
    cards: Vec<CardInfo>,
    #[serde(rename(serialize = "topCard", deserialize = "topCard"))]
    top_card: CardInfo,
    #[serde(rename(
        serialize = "isClockwiseDirection",
        deserialize = "isClockwiseDirection"
    ))]
    is_clockwise_direction: bool,
}
