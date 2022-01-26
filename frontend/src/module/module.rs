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
    pub status: String,
    pub author: String,
    pub you: String, //jméno hráče, který žádal o status
    #[serde(rename(serialize = "currentPlayer", deserialize = "currentPlayer"))]
    pub current_player: String, //jméno hráče na tahu
    pub players: Vec<Player>,
    #[serde(rename(serialize = "finishedPlayers", deserialize = "finishedPlayers"))]
    pub finished_players: Vec<String>, //hráči, kteří už se zbavili karet v pořadí, v jakém skončili
    pub cards: Vec<CardInfo>,
    #[serde(rename(serialize = "topCard", deserialize = "topCard"))]
    pub top_card: CardInfo,
    #[serde(rename(
        serialize = "isClockwise",
        deserialize = "isClockwise"
    ))]
    pub is_clockwise_direction: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PlayCard {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub typee: String,
    pub who: String,
    pub next: String,
    pub card: CardInfo,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DrawCard {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub typee: String,
    pub who: String,
    pub next: String,
    pub cards: u32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Finish {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub typee: String,
    pub who: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Penalty {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub typee: String,
    pub cards: Vec<CardInfo>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GainedCards {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub typee: String,
    pub who: String,
    pub cards: u32,
}
