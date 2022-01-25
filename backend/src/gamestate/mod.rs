mod active_cards;
pub mod game;
pub mod player;

pub type WSMessage = String;

pub static CARDS_DEALT_TO_PLAYERS: usize = 7;
pub static PENALTY_CARDS: usize = 2;
