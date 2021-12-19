pub struct Card {
    pub color: CardColor,
    pub symbol: CardSymbol
}

pub enum CardColor {
    Red,
    Yellow,
    Green,
    Blue,
    Black,
}

pub enum CardSymbol {
    Value(i8),
    Skip,
    Reverse,
    Draw2,
    Draw4,
    Wild
}