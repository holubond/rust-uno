use crate::cards::card::{Card, CardSymbol};
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};

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
