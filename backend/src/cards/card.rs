use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize, Serializer, Deserializer, de};
use std::fmt::{Display, Formatter};
use serde::de::{Visitor, MapAccess, SeqAccess};
use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone)]
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

impl<'de> Deserialize<'de> for Card {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        enum Field { Color, Symbol, Value }

        // This part could also be generated independently by:
        //
        //    #[derive(Deserialize)]
        //    #[serde(field_identifier, rename_all = "lowercase")]
        //    enum Field { Secs, Nanos }
        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
                where
                    D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`color` or `symbol` or `value`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                        where
                            E: de::Error,
                    {
                        match value {
                            "color" => Ok(Field::Color),
                            "type" => Ok(Field::Symbol),
                            "value" => Ok(Field::Value),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct CardVisitor;

        impl<'de> Visitor<'de> for CardVisitor {
            type Value = Card;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Card")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Card, V::Error>
                where
                    V: SeqAccess<'de>,
            {
                let color = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let symbol = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;

                let symbol = if let Ok(maybe_value) = seq.next_element() {
                    if let Some(value) = maybe_value {
                        CardSymbol::Value(value)
                    } else {
                        symbol
                    }
                } else {
                    symbol
                };
                // todo not unwrap
                Ok(Card::new(color, symbol).unwrap())
            }

            fn visit_map<V>(self, mut map: V) -> Result<Card, V::Error>
                where
                    V: MapAccess<'de>,
            {
                let mut color = None;
                let mut symbol = None;
                let mut value = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Color => {
                            if color.is_some() {
                                return Err(de::Error::duplicate_field("color"));
                            }
                            color = Some(map.next_value()?);
                        }
                        Field::Symbol => {
                            if symbol.is_some() {
                                return Err(de::Error::duplicate_field("symbol"));
                            }
                            symbol = Some(map.next_value()?);
                        }
                        Field::Value => {
                            if value.is_some() {
                                return Err(de::Error::duplicate_field("value"));
                            }
                            let maybe_value = map.next_value()?;
                            if let Some(i8_value) = maybe_value {
                                value = Some(i8_value);
                            }
                        }
                    }
                }
                let color = color.ok_or_else(|| de::Error::missing_field("color"))?;
                let symbol = symbol.ok_or_else(|| de::Error::missing_field("symbol"))?;
                let symbol = if let Some(value) = value {
                    CardSymbol::Value(value)
                } else {
                    symbol
                };

                // todo not unwrap
                Ok(Card::new(color, symbol).unwrap())
            }
        }

        const FIELDS: &'static [&'static str] = &["color", "symbol"];
        deserializer.deserialize_struct("Card", FIELDS, CardVisitor)
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