use crate::pages::game::{GameState, GameStore};
use crate::{
    components::card::{CardInfo, CardType, Color},
    pages::game::Player,
    Game,
};
use reqwest::Client;
use std::sync::Arc;

pub fn test_session(game: GameStore) -> Game {
    Game {
        client: Arc::new(Client::new()),
        game,
        status: GameState::Lobby,
        author: "Were".to_string(),
        you: "Were".to_string(),
        cards: cards(),
        players: players(),
        current_player: Some("Holy".to_string()),
        finished_players: vec![],
        clockwise: true,
        uno_bool: false,
        discarted_card: CardInfo {
            color: Color::Red,
            _type: CardType::Value,
            value: Some(3),
        },
        real_game_id: "".to_string(),
        logs: vec![],
    }
}

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
            value: *v,
        })
        .collect()
}
