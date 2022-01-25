use crate::{
    components::card::{CardInfo, CardType, Color},
    pages::game::Player,
};

pub fn players() -> Vec<Player> {
    let players = vec![
        ("Kája", 8),
        ("Grolig", 5),
        ("Holy", 0),
        ("End", 4),
        ("Were", 4),
    ];

    players
        .iter()
        .map(|(name, cards)| Player {
            name: name.to_string(),
            cards: *cards,
        })
        .collect()
}

pub fn cards() -> Vec<CardInfo> {
    let cards = vec![
        (Color::Blue, CardType::Value, Some(1)),
        (Color::Green, CardType::Value, Some(3)),
        (Color::Red, CardType::Value, Some(3)),
        (Color::Black, CardType::Wild, None),
        (Color::Green, CardType::Value, Some(3)),
        (Color::Red, CardType::Draw2, Some(3)),
        (Color::Red, CardType::Value, Some(3)),
    ];

    cards
        .iter()
        .map(|(c, t, v)| CardInfo {
            color: c.clone(),
            _type: t.clone(),
            value: v.clone(),
        })
        .collect()
}
