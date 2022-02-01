use crate::cards::card::CardColor;
use rand::Rng;

pub mod card;
pub mod deck;

pub fn random_color() -> CardColor {
    match rand::thread_rng().gen_range(0..4) {
        0 => CardColor::Red,
        1 => CardColor::Blue,
        2 => CardColor::Green,
        _ => CardColor::Yellow,
    }
}
