use crate::cards::card::{Card, CardColor, CardSymbol};

pub struct Deck {
    draw_pile: Vec<Card>,
    discard_pile: Vec<Card>,
}
