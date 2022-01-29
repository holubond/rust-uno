use crate::cards::card::Card;
use crate::err::play_card::PlayCardError;
use crate::gamestate::players::ai::Ai;
use crate::gamestate::players::player::Player;
use crate::ws::ws_conn::WSConn;
use crate::ws::ws_message::WSMsg;
use Participant::*;

pub mod ai;
pub mod player;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Participant {
    Human(Player),
    NonHuman(Ai),
}

impl Participant {
    pub fn is_author(&self) -> bool {
        match self {
            Human(player) => player.is_author(),
            NonHuman(_) => false,
        }
    }
    pub fn name(&self) -> String {
        match self {
            Human(player) => player.name(),
            NonHuman(ai) => ai.name(),
        }
    }

    pub fn message(&self, msg: WSMsg) {
        match self {
            Human(player) => player.message(msg),
            NonHuman(_) => {}
        }
    }
    pub fn set_connection(&mut self, connection: WSConn) {
        match self {
            Human(player) => player.set_connection(connection),
            NonHuman(_) => {}
        }
    }

    pub fn cards(&self) -> Vec<Card> {
        match self {
            Human(player) => player.cards(),
            NonHuman(ai) => ai.cards(),
        }
    }
    pub fn give_card(&mut self, card: Card) {
        match self {
            Human(player) => player.give_card(card),
            NonHuman(ai) => ai.give_card(card),
        }
    }
    pub fn play_card(&mut self, card: Card) -> Result<Card, PlayCardError> {
        match self {
            Human(player) => player.play_card_by_eq(card),
            NonHuman(ai) => ai.play_card_by_eq(card),
        }
    }
    pub fn drop_all_cards(&mut self) {
        match self {
            Human(player) => player.drop_all_cards(),
            NonHuman(ai) => ai.drop_all_cards(),
        }
    }
    pub fn get_card_count(&self) -> usize {
        match self {
            Human(player) => player.get_card_count(),
            NonHuman(ai) => ai.get_card_count(),
        }
    }

    pub fn should_say_uno(&self) -> bool {
        self.get_card_count() == 2
    }

    pub fn is_finished(&self) -> bool {
        match self {
            Human(player) => player.is_finished(),
            NonHuman(ai) => ai.is_finished(),
        }
    }
    pub fn position(&self) -> Option<usize> {
        match self {
            Human(player) => player.position(),
            NonHuman(ai) => ai.position(),
        }
    }
    pub fn set_position(&mut self, position: usize) {
        match self {
            Human(player) => player.set_position(position),
            NonHuman(ai) => ai.set_position(position),
        }
    }
    pub fn clear_position(&mut self) {
        match self {
            Human(player) => player.clear_position(),
            NonHuman(ai) => ai.clear_position(),
        }
    }
}
