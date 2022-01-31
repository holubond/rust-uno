use crate::cards::card::Card;
use crate::err::play_card::PlayCardError;
use crate::ws::ws_conn::WSConn;
use crate::ws::ws_message::WSMsg;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Player {
    name: String,
    is_author: bool,
    is_human: bool,
    cards: Vec<Card>,
    position: Option<usize>,
    connection: Option<WSConn>,
}

impl Player {
    pub fn new(name: String, is_author: bool, is_human: bool) -> Player {
        Player {
            name,
            is_author,
            is_human,
            cards: vec![],
            position: None,
            connection: None,
        }
    }

    /// Function returns Err if index is out of bounds
    pub fn play_card_by_index(&mut self, index: usize) -> anyhow::Result<Card> {
        if index >= self.cards.len() {
            anyhow::bail!(
                "Index {} out of bounds for card vec size of {}",
                index,
                self.cards.len()
            )
        }

        Ok(self.cards.remove(index))
    }

    /// Function returns Err if card is not owned by the player.
    pub fn play_card(&mut self, card: Card) -> Result<Card, PlayCardError> {
        let maybe_position = self.cards.iter().position(|c| c == &card);

        match maybe_position {
            None => Err(PlayCardError::PlayerHasNoSuchCard(card)),
            Some(position) => Ok(self.play_card_by_index(position).unwrap()),
        }
    }

    pub fn give_card(&mut self, card: Card) {
        self.cards.push(card)
    }

    pub fn drop_all_cards(&mut self) {
        self.cards.clear();
    }

    pub fn is_finished(&self) -> bool {
        self.position != None
    }

    pub fn should_say_uno(&self) -> bool {
        self.cards.len() == 2
    }

    pub fn position(&self) -> Option<usize> {
        self.position
    }

    pub fn set_position(&mut self, position: usize) {
        self.position = Some(position)
    }

    pub fn clear_position(&mut self) {
        self.position = None
    }

    pub fn get_card_count(&self) -> usize {
        self.cards.len()
    }

    /// Clones the vector of cards of the player.
    pub fn cards(&self) -> Vec<Card> {
        self.cards.clone()
    }

    /// Clones the name of the player.
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn is_author(&self) -> bool {
        self.is_author
    }

    pub fn is_human(&self) -> bool {
        self.is_human
    }

    pub fn message(&self, msg: WSMsg) {
        match &self.connection {
            Some(conn) => conn.send(msg),
            None => (),
        }
    }

    pub fn set_connection(&mut self, connection: WSConn) {
        self.connection = Option::Some(connection)
    }
}
