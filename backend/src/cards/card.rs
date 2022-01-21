use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct Card {
    pub color: CardColor,
    pub symbol: CardSymbol,
}

impl Card {
    pub fn new(color: CardColor, symbol: CardSymbol) -> anyhow::Result<Card> {
        if (symbol == CardSymbol::Wild || symbol == CardSymbol::Draw4) && color != CardColor::Black
        {
            anyhow::bail!(
                "Invalid card combination: color: {:?} & symbol {:?}",
                color,
                symbol
            );
        }

        if let CardSymbol::Value(number) = symbol {
            if !(0..=9).contains(&number) {
                anyhow::bail!("Invalid card value: {} not between 0 and 9", number);
            }
        }

        Ok(Card { color, symbol })
    }
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone, Copy)]
pub enum CardColor {
    Red,
    Yellow,
    Green,
    Blue,
    Black,
}

impl CardColor {
    pub fn non_black_iter() -> impl Iterator<Item = CardColor> {
        use CardColor::*;

        [Red, Yellow, Green, Blue].iter().copied()
    }
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub enum CardSymbol {
    Value(i8),
    Skip,
    Reverse,
    Draw2,
    Draw4,
    Wild,
}
