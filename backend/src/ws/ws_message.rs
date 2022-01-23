use crate::gamestate::game::{Game, GameStatus};
use crate::ws::ws_structs::draw::DrawWSMessage;
use crate::ws::ws_structs::status::{
    FinishedStatusWSMessage, LobbyStatusWSMessage, RunningStatusWSMessage,
};
use crate::ws::ws_structs::WsMessageWrapper;
use actix::Message;

/// WebSocket message that can be sent to a WebSocket connection
#[derive(Message)]
#[rtype(result = "()")]
pub struct WSMsg {
    pub msg: String,
}

// TODO - implement all types of WS messages
impl WSMsg {
    // This is a sample function, delete after implementation of others
    pub fn custom(msg: String) -> Self {
        Self { msg: msg }
    }

    pub fn status(game: &Game, target_player_name: String) -> Self {
        let msg = match game.status() {
            GameStatus::Lobby => LobbyStatusWSMessage::new(game, target_player_name).ws_serialize(),
            GameStatus::Running => {
                RunningStatusWSMessage::new(game, target_player_name).ws_serialize()
            }
            GameStatus::Finished => {
                FinishedStatusWSMessage::new(game, target_player_name).ws_serialize()
            }
        };

        Self::custom(msg)
    }

    pub fn draw(target_player_name: String, next_player_name: String, cards_drawn: usize) -> Self {
        let msg = DrawWSMessage::new(target_player_name, next_player_name, cards_drawn);
        Self::custom(msg.ws_serialize())
    }
}
