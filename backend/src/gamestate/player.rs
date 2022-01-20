use crate::cards::card::Card;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct Player {
    pub name: String,
    pub is_author: bool,
    pub cards: Vec<Card>,
    pub position: Option<usize>,
}

impl Player {
    pub fn new(name: String, is_author: bool) -> Player {
        Player {
            name,
            is_author,
            cards: vec![],
            position: None,
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
}
