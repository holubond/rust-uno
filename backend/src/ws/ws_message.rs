use crate::cards::card::Card;
use crate::err::status::CreateStatusError;
use crate::gamestate::game::{Game, GameStatus};
use crate::ws::ws_structs::draw::{DrawMeWSMessage, DrawWSMessage};
use crate::ws::ws_structs::finish::FinishWSMessage;
use crate::ws::ws_structs::gained_cards::GainedCardsWSMessage;
use crate::ws::ws_structs::penalty::PenaltyWSMessage;
use crate::ws::ws_structs::play_card::PlayCardWSMessage;
use crate::ws::ws_structs::status::{
    FinishedStatusWSMessage, LobbyStatusWSMessage, RunningStatusWSMessage,
};
use crate::ws::ws_structs::WsMessageWrapper;
use actix::Message;

/// WebSocket message that can be sent to a WebSocket connection
#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct WSMsg {
    pub msg: String,
}

impl WSMsg {
    fn new(msg: String) -> Self {
        Self { msg }
    }

    pub fn status(game: &Game, target_player_name: String) -> Result<Self, CreateStatusError> {
        let msg = match game.status() {
            GameStatus::Lobby => {
                LobbyStatusWSMessage::new(game, target_player_name)?.ws_serialize()
            }
            GameStatus::Running => {
                RunningStatusWSMessage::new(game, target_player_name)?.ws_serialize()
            }
            GameStatus::Finished => {
                FinishedStatusWSMessage::new(game, target_player_name)?.ws_serialize()
            }
        };

        Ok(Self::new(msg))
    }

    pub fn draw(drawing_player_name: String, next_player_name: String, cards_drawn: usize) -> Self {
        let msg = DrawWSMessage::new(drawing_player_name, next_player_name, cards_drawn);
        Self::new(msg.ws_serialize())
    }

    pub fn draw_me(next_player_name: String, cards_drawn: Vec<Card>) -> Self {
        let msg = DrawMeWSMessage::new(next_player_name, cards_drawn);
        Self::new(msg.ws_serialize())
    }

    pub fn play_card(
        playing_player_name: String,
        next_player_name: String,
        card_drawn: Card,
    ) -> Self {
        let msg = PlayCardWSMessage::new(playing_player_name, next_player_name, card_drawn);
        Self::new(msg.ws_serialize())
    }

    pub fn finish(finished_player_name: String) -> Self {
        let msg = FinishWSMessage::new(finished_player_name);
        Self::new(msg.ws_serialize())
    }

    pub fn penalty(penalized_player_name: String, gained_cards: Vec<Card>) -> Self {
        let msg = PenaltyWSMessage::new(penalized_player_name, gained_cards);
        Self::new(msg.ws_serialize())
    }

    pub fn gained_cards(penalized_player_name: String, gained_cards_count: usize) -> Self {
        let msg = GainedCardsWSMessage::new(penalized_player_name, gained_cards_count);
        Self::new(msg.ws_serialize())
    }
}
