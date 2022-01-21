use crate::cards::card::Card;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct Player {
    name: String,
    pub is_author: bool,
    pub jwt: String,
    cards: Vec<Card>,
    position: Option<usize>,
}

impl Player {
    pub fn new(name: String, is_author: bool) -> Player {
        Player {
            name,
            is_author,
            cards: vec![],
            position: None,
            jwt: "".to_string(),
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
    pub fn play_card_by_eq(&mut self, card: Card) -> anyhow::Result<Card> {
        let maybe_position = self.cards.iter().position(|c| c == &card);

        match maybe_position {
            None => anyhow::bail!(
                "Card {:?} was not found in player {}'s hands.",
                card,
                self.name
            ),
            Some(position) => self.play_card_by_index(position),
        }
    }

    pub fn give_card(&mut self, card: Card) {
        self.cards.push(card)
    }

    pub fn is_finished(&self) -> bool {
        self.position != None
    }

    pub fn position(&self) -> Option<usize> {
        self.position
    }

    pub fn set_position(&mut self, position: usize) {
        self.position = Some(position)
    }

    pub fn get_card_count(&self) -> usize {
        self.cards.len()
    }

    /// Clones the name of the player.
    pub fn cards(&self) -> Vec<Card> {
        self.cards.clone()
    }

    /// Clones the name of the player.
    pub fn name(&self) -> String {
        self.name.clone()
    }
}
