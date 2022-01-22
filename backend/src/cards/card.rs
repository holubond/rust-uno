use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize, Serializer};
use serde::ser::SerializeStruct;

#[derive(Debug, Deserialize, Eq, PartialEq, Clone)]
pub struct Card {
    pub color: CardColor,
    pub symbol: CardSymbol,
}

impl Card {
    /// Returns error when symbol == Wild | Draw4 and color != Black,
    /// or when symbol == Value(n) where n is not in range (0..=9).
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

    /// Allows for in-place transformation of a black card's color.
    /// Returns Err when it is called on a non-black card.
    pub fn morph_black_card(mut self, new_color: CardColor) -> anyhow::Result<Card> {
        if self.should_be_black() {
            self.color = new_color;
            Ok(self)
        } else {
            anyhow::bail!("Cannot change color of a non-black card!")
        }
    }

    /// Based on a cards symbol, tells whether the card can/should be Black
    pub fn should_be_black(&self) -> bool {
        self.symbol == CardSymbol::Wild || self.symbol == CardSymbol::Draw4
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

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} Card", self.color, self.symbol)
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

impl Display for CardColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
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

impl Display for CardSymbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CardSymbol::Value(number) => number.to_string(),
                CardSymbol::Skip => "Skip".into(),
                CardSymbol::Reverse => "Reverse".into(),
                CardSymbol::Draw2 => "+2".into(),
                CardSymbol::Draw4 => "+4".into(),
                CardSymbol::Wild => "Wild".into(),
            }
        )
    }
}
