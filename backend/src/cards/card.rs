use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Card {
    pub color: CardColor,
    pub symbol: CardSymbol
}

#[derive(Serialize, Deserialize)]
pub enum CardColor {
    Red,
    Yellow,
    Green,
    Blue,
    Black,
}

#[derive(Serialize, Deserialize)]
pub enum CardSymbol {
    Value(i8),
    Skip,
    Reverse,
    Draw2,
    Draw4,
    Wild
}