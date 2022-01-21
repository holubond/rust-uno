use crate::cards::card::{Card, CardColor, CardSymbol};
use rand::seq::SliceRandom;

#[derive(Clone)]
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

        assert_eq!(draw_pile.len(), 108);

        let mut deck = Deck {
            draw_pile,
            discard_pile: Vec::new(),
        };

        deck.shuffle_draw_pile();
        deck
    }

    fn shuffle_draw_pile(&mut self) {
        self.draw_pile.shuffle(&mut rand::thread_rng());
    }

    pub fn draw(&mut self) -> Option<Card> {
        match self.draw_pile.pop() {
            None => {
                if self.discard_pile.is_empty() {
                    return None;
                }

                self.draw_pile.append(&mut self.discard_pile);
                self.shuffle_draw_pile();

                // should definitely be a safe operation
                Some(self.draw_pile.pop().unwrap())
            }
            Some(card) => Some(card),
        }
    }

    pub fn play(&mut self, card: Card) {
        self.discard_pile.push(card);
    }
}

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

fn insert_colored_symbol_cards(card_stack: &mut Vec<Card>) {
    for color in CardColor::non_black_iter() {
        for _ in 0..2 {
            for symbol in [CardSymbol::Skip, CardSymbol::Reverse, CardSymbol::Draw2] {
                card_stack.push(Card::new(color, symbol).unwrap())
            }
        }
    }
}

fn insert_black_symbol_cards(card_stack: &mut Vec<Card>) {
    for _ in 0..4 {
        card_stack.push(Card::new(CardColor::Black, CardSymbol::Wild).unwrap());
        card_stack.push(Card::new(CardColor::Black, CardSymbol::Draw4).unwrap());
    }
}
