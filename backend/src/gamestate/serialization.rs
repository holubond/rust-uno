use crate::cards::card::{Card, CardSymbol};
use crate::gamestate::game::{Game, GameStatus};
use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize, Serializer};

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

#[derive(Serialize, Deserialize)]
pub struct LobbyStatus {
    #[serde(rename = "type")]
    typee: String,
    status: GameStatus,
    author: String,
    you: String,
    players: Vec<String>,
}

impl LobbyStatus {
    pub fn new(game: &Game, target_player_name: String) -> LobbyStatus {
        LobbyStatus {
            typee: "STATUS".to_string(),
            status: GameStatus::Lobby,
            author: find_author_name(game),
            you: target_player_name,
            players: game.players.iter().map(|p| p.name()).collect(),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct RunningPlayer {
    name: String,
    cards: usize,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RunningStatus {
    #[serde(rename = "type")]
    typee: String,
    status: GameStatus,
    author: String,
    you: String,
    current_player: String,
    players: Vec<RunningPlayer>,
    finished_players: Vec<String>,
    cards: Vec<Card>,
}

impl RunningStatus {
    pub fn new(game: &Game, target_player_name: String) -> RunningStatus {
        RunningStatus {
            typee: "STATUS".to_string(),
            status: GameStatus::Running,
            author: find_author_name(game),
            you: target_player_name.clone(),
            current_player: get_current_player_name(game),
            players: RunningStatus::process_players(game),
            finished_players: get_finished_player_names(game),
            cards: match game.find_player(target_player_name.clone()) {
                None => vec![],
                Some(player) => player.cards(),
            },
        }
    }

    fn process_players(game: &Game) -> Vec<RunningPlayer> {
        let mut players = Vec::new();

        for player in game.players.clone() {
            players.push(RunningPlayer {
                name: player.name(),
                cards: player.get_card_count(),
            });
        }

        players
    }
}

#[derive(Serialize, Deserialize)]
pub struct FinishedStatus {
    #[serde(rename = "type")]
    typee: String,
    status: GameStatus,
    author: String,
    you: String,
    finished_players: Vec<String>,
}

impl FinishedStatus {
    pub fn new(game: &Game, target_player_name: String) -> FinishedStatus {
        FinishedStatus {
            typee: "STATUS".into(),
            status: GameStatus::Finished,
            author: find_author_name(game),
            you: target_player_name.clone(),
            finished_players: get_finished_player_names(game),
        }
    }
}

fn get_finished_player_names(game: &Game) -> Vec<String> {
    game.get_finished_players()
        .iter()
        .map(|p| p.name())
        .collect()
}

fn find_author_name(game: &Game) -> String {
    match game.find_author() {
        None => "UnknownAuthor".into(),
        Some(author) => author.name(),
    }
}

fn get_current_player_name(game: &Game) -> String {
    match game.get_current_player() {
        None => "UnknownCurrentPlayer".into(),
        Some(player) => player.name(),
    }
}
