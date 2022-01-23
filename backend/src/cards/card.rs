use serde::{Deserialize, Serialize, Serializer};
use serde::ser::SerializeStruct;

#[derive(Debug, Deserialize, Eq, PartialEq, Clone)]
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

impl Serialize for Card {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut state = serializer.serialize_struct("Card", 3)?;
        state.serialize_field("color", &self.color)?;
        state.serialize_field("type", &self.symbol)?;
        match self.symbol {
            CardSymbol::Value(number) => state.serialize_field("value", &number),
            _ => state.serialize_field("value", &Option::<i8>::None),
        }?;

        state.end()
    }
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone, Copy)]
#[serde(rename_all = "UPPERCASE")]
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
#[serde(rename_all = "UPPERCASE")]
pub enum CardSymbol {
    Value(#[serde(skip)] i8),
    Skip,
    Reverse,
    Draw2,
    Draw4,
    Wild,
}
