use crate::cards::card::Card;
use crate::gamestate::game::{Game, GameStatus};
use crate::ws::ws_structs::{
    find_author_name, get_current_player_name, get_finished_player_names, WsMessageWrapper,
};
use ::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LobbyStatusWSMessage {
    #[serde(rename = "type")]
    typee: String,
    status: GameStatus,
    author: String,
    you: String,
    players: Vec<String>,
}

impl LobbyStatusWSMessage {
    pub fn new(game: &Game, target_player_name: String) -> anyhow::Result<LobbyStatusWSMessage> {
        Ok(LobbyStatusWSMessage {
            typee: "STATUS".to_string(),
            status: GameStatus::Lobby,
            author: find_author_name(game)?,
            you: target_player_name,
            players: game.players.iter().map(|p| p.name()).collect(),
        })
    }
}

#[derive(Serialize, Deserialize)]
struct RunningPlayer {
    name: String,
    cards: usize,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RunningStatusWSMessage {
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

impl RunningStatusWSMessage {
    pub fn new(game: &Game, target_player_name: String) -> anyhow::Result<RunningStatusWSMessage> {
        Ok(RunningStatusWSMessage {
            typee: "STATUS".to_string(),
            status: GameStatus::Running,
            author: find_author_name(game)?,
            you: target_player_name.clone(),
            current_player: get_current_player_name(game)?,
            players: RunningStatusWSMessage::process_players(game),
            finished_players: get_finished_player_names(game),
            cards: match game.find_player(target_player_name.clone()) {
                None => vec![],
                Some(player) => player.cards(),
            },
        })
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
pub struct FinishedStatusWSMessage {
    #[serde(rename = "type")]
    typee: String,
    status: GameStatus,
    author: String,
    you: String,
    finished_players: Vec<String>,
}

impl FinishedStatusWSMessage {
    pub fn new(game: &Game, target_player_name: String) -> anyhow::Result<FinishedStatusWSMessage> {
        Ok(FinishedStatusWSMessage {
            typee: "STATUS".into(),
            status: GameStatus::Finished,
            author: find_author_name(game)?,
            you: target_player_name.clone(),
            finished_players: get_finished_player_names(game),
        })
    }
}

impl WsMessageWrapper for LobbyStatusWSMessage {}

impl WsMessageWrapper for RunningStatusWSMessage {}

impl WsMessageWrapper for FinishedStatusWSMessage {}