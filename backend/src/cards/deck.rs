use crate::cards::card::{Card, CardColor, CardSymbol};
use rand::seq::SliceRandom;
use crate::cards::card::CardColor::{Red, Blue, Green, Yellow};
use rand::Rng;

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

        let mut deck = Deck {
            draw_pile,
            discard_pile: Vec::new(),
        };

        deck.shuffle_draw_pile();

        // ensure discard pile starts with one random card
        let mut new_top_card = deck.draw_pile.pop().unwrap();
        if new_top_card.should_be_black() {
            new_top_card = new_top_card.morph_black_card(random_color()).unwrap();
        }
        deck.discard_pile.push(new_top_card);

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

                self.switch_piles();

                // should definitely be a safe operation
                Some(self.draw_pile.pop().unwrap())
            }
            Some(card) => Some(card),
        }
    }

    fn switch_piles(&mut self) {
        let last_discarded_card = self.discard_pile.pop().unwrap();

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
        // draw pile should always have at least one card
        self.discard_pile.last().unwrap()
    }

    pub fn can_play_card(&self, played_card: &Card) -> bool {
        use CardColor::*;
        use CardSymbol::*;

        let top_card = self.top_discard_card();

        played_card.color == Black
            || played_card.color == top_card.color
            || played_card.symbol == top_card.symbol
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

#[cfg(test)]
mod tests {
    use crate::cards::card::{Card, CardColor, CardSymbol};
    use crate::cards::deck::{Deck, random_color};
    use rand::Rng;

    #[test]
    fn test_card_symbol_eq() {
        use crate::cards::card::CardSymbol::Value;

        assert_ne!(Value(8), Value(9));
        assert_eq!(Value(8), Value(8));
    }

    #[test]
    fn test_can_play_card() {
        use CardColor::*;
        use CardSymbol::*;

        let mut deck = Deck::new();
        deck.discard_pile.push(Card::new(Red, Value(5)).unwrap());

        assert!(deck.can_play_card(&Card::new(Red, Value(5)).unwrap()));
        assert!(deck.can_play_card(&Card::new(Red, Value(6)).unwrap()));
        assert!(deck.can_play_card(&Card::new(Blue, Value(5)).unwrap()));
        assert!(deck.can_play_card(&Card::new(Red, Reverse).unwrap()));
        assert!(deck.can_play_card(&Card::new(Black, Wild).unwrap()));
        assert!(deck.can_play_card(&Card::new(Black, Draw4).unwrap()));

        assert!(!deck.can_play_card(&Card::new(Blue, Value(6)).unwrap()));
        assert!(!deck.can_play_card(&Card::new(Green, Draw2).unwrap()));
        assert!(!deck.can_play_card(&Card::new(Yellow, Skip).unwrap()));

        deck.discard_pile.push(Card::new(Red, Draw2).unwrap());
        assert!(deck.can_play_card(&Card::new(Red, Draw2).unwrap()));
        assert!(deck.can_play_card(&Card::new(Blue, Draw2).unwrap()));
        assert!(deck.can_play_card(&Card::new(Red, Value(5)).unwrap()));
        assert!(deck.can_play_card(&Card::new(Black, Wild).unwrap()));
        assert!(deck.can_play_card(&Card::new(Black, Draw4).unwrap()));

        assert!(!deck.can_play_card(&Card::new(Blue, Value(6)).unwrap()));
        assert!(!deck.can_play_card(&Card::new(Green, Reverse).unwrap()));
        assert!(!deck.can_play_card(&Card::new(Yellow, Skip).unwrap()));
    }

    #[test]
    fn test_108_new_cards() {
        let deck = Deck::new();
        assert_eq!(deck.draw_pile.len(), 107);
        assert_eq!(deck.discard_pile.len(), 1);
    }

    #[test]
    fn test_switch_piles() {
        use CardColor::*;

        let mut deck = Deck::new();

        for _ in 0..106 {
            let mut drawn = deck.draw().unwrap();
            // simulate a full deck being used
            if drawn.should_be_black() {
                drawn.color = random_color();
            }

            deck.play(drawn);
        }

        let leftover_card = deck.top_discard_card().clone();
        assert_eq!(deck.discard_pile.len(), 107);
        assert_eq!(deck.draw_pile.len(), 1); // drawing any more would cause an automatic switch
        assert!(deck.discard_pile.iter().all(|card| card.color != Black));

        deck.switch_piles();

        assert_eq!(deck.discard_pile.len(), 1);
        assert_eq!(deck.draw_pile.len(), 107);
        assert!(deck.draw_pile.iter().all(|card| if card.should_be_black() {card.color == Black} else {true}));
        assert_eq!(deck.top_discard_card(), &leftover_card);
    }

}

fn random_color() -> CardColor {
    match rand::thread_rng().gen_range(0..4) {
        0 => Red,
        1 => Blue,
        2 => Green,
        _ => Yellow
    }
}
