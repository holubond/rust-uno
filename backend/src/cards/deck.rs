use crate::cards::card::{Card, CardColor, CardSymbol};
use crate::cards::random_color;
use rand::seq::SliceRandom;

#[derive(Debug, Clone)]
pub struct Deck {
    draw_pile: Vec<Card>,
    discard_pile: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        let mut draw_pile = Vec::new();

        insert_number_cards(&mut draw_pile);
        insert_colored_symbol_cards(&mut draw_pile);
        insert_black_symbol_cards(&mut draw_pile);

        let mut deck = Deck {
            draw_pile,
            discard_pile: Vec::new(),
        };

        deck.shuffle_draw_pile();

        // ensure discard pile starts with one random card
        let mut new_top_card = deck.draw_pile.pop().unwrap(); // safe since we just created the draw_pile using the insert_* functions
        if new_top_card.should_be_black() {
            // safe since explicit should_be_black() test
            new_top_card = new_top_card.morph_black_card(random_color()).unwrap();
        }
        deck.discard_pile.push(new_top_card);

        deck
    }

    fn shuffle_draw_pile(&mut self) {
        self.draw_pile.shuffle(&mut rand::thread_rng());
    }

    fn switching_piles_wont_move_card(&self) -> bool {
        self.discard_pile.is_empty() || self.discard_pile.len() == 1
    }

    pub fn draw(&mut self) -> Option<Card> {
        match self.draw_pile.pop() {
            None => {
                if self.switching_piles_wont_move_card() {
                    return None;
                }

                self.switch_piles();

                Some(self.draw_pile.pop().unwrap()) // safe since both piles being empty is handled & switch_piles() moves at least one card then
            }
            Some(card) => Some(card),
        }
    }

    fn switch_piles(&mut self) {
        if self.switching_piles_wont_move_card() {
            return;
        }

        let last_discarded_card = self.discard_pile.pop().unwrap(); // safe since check above

        for card in self.discard_pile.iter_mut() {
            if card.should_be_black() {
                card.color = CardColor::Black;
            }
        }

        self.draw_pile.append(&mut self.discard_pile);
        self.shuffle_draw_pile();

        self.discard_pile.push(last_discarded_card);
    }

    pub fn play(&mut self, card: Card) {
        self.discard_pile.push(card);
    }

    pub fn top_discard_card(&self) -> &Card {
        self.discard_pile.last().unwrap() // safe since draw pile should always have at least one card
    }

    // Used in a test
    #[allow(dead_code)]
    pub fn draw_pile_size(&self) -> usize {
        self.draw_pile.len()
    }

    // Used in a test
    #[allow(dead_code)]
    pub fn discard_pile_size(&self) -> usize {
        self.discard_pile.len()
    }
}

// use of Card::new(...).unwrap() in this function is safe at the time of this commit
fn insert_number_cards(card_stack: &mut Vec<Card>) {
    for color in CardColor::non_black_iter() {
        // only one value=0 card of each color
        card_stack.push(Card::new(color, CardSymbol::Value(0)).unwrap());

        for _ in 0..2 {
            for value in 1..=9 {
                card_stack.push(Card::new(color, CardSymbol::Value(value)).unwrap())
            }
        }
    }
}

// use of Card::new(...).unwrap() in this function is safe at the time of this commit
fn insert_colored_symbol_cards(card_stack: &mut Vec<Card>) {
    for color in CardColor::non_black_iter() {
        for _ in 0..2 {
            for symbol in [CardSymbol::Skip, CardSymbol::Reverse, CardSymbol::Draw2] {
                card_stack.push(Card::new(color, symbol).unwrap())
            }
        }
    }
}

// use of Card::new(...).unwrap() in this function is safe at the time of this commit
fn insert_black_symbol_cards(card_stack: &mut Vec<Card>) {
    for _ in 0..4 {
        card_stack.push(Card::new(CardColor::Black, CardSymbol::Wild).unwrap());
        card_stack.push(Card::new(CardColor::Black, CardSymbol::Draw4).unwrap());
    }
}

#[cfg(test)]
#[path = "../tests/deck_test.rs"]
mod tests;
