use crate::gamestate::players::player::Player;
use crate::cards::card::{Card, CardColor, CardSymbol};
use crate::cards::random_color;

pub fn decide_new_color(card: &Card) -> Option<CardColor> {
    if card.should_be_black() {
        Some(random_color())
    } else {
        None
    }
}

pub fn first_card_of_symbol(player: &Player, symbol: CardSymbol) -> Option<Card> {
    player.cards()
        .iter()
        .find(|card| card.symbol == symbol)
        .cloned()
}

pub fn first_playable_card_against(player: &Player, top_card: &Card) -> Option<Card> {
    player.cards()
        .iter()
        .find(|card| {
            card.color == CardColor::Black
                || card.color == top_card.color
                || card.symbol == top_card.symbol
        })
        .cloned()
}