use crate::cards::card::{Card, CardColor};
use crate::cards::deck::{
    insert_black_symbol_cards, insert_colored_symbol_cards, insert_number_cards, Deck,
};
use crate::cards::random_color;

#[test]
fn test_card_symbol_eq() {
    use crate::cards::card::CardSymbol::Value;

    assert_ne!(Value(8), Value(9));
    assert_eq!(Value(8), Value(8));
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
    assert!(deck.draw_pile.iter().all(|card| if card.should_be_black() {
        card.color == Black
    } else {
        true
    }));
    assert_eq!(deck.top_discard_card(), &leftover_card);
}

#[test]
fn test_ser_de() {
    let mut draw_pile = Vec::new();
    let mut transmuted_pile: Vec<Card> = Vec::new();

    insert_number_cards(&mut draw_pile);
    insert_colored_symbol_cards(&mut draw_pile);
    insert_black_symbol_cards(&mut draw_pile);

    for card in draw_pile.iter() {
        transmuted_pile
            .push(serde_json::from_str(&serde_json::to_string(&card.clone()).unwrap()).unwrap())
    }

    assert_eq!(draw_pile, transmuted_pile);
}


#[test]
fn test_one_card_left() {
    let mut deck = Deck::new();
    deck.draw_pile.clear();

    assert_eq!(deck.draw_pile.len(), 0);
    assert_eq!(deck.discard_pile.len(), 1);

    assert!(deck.draw().is_none());
}