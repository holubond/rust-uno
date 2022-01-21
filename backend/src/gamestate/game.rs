use crate::cards::deck::Deck;
use crate::gamestate::player::Player;
use crate::gamestate::serialization::{FinishedStatus, LobbyStatus, RunningStatus};
use crate::gamestate::WSMessage;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone, Copy)]
#[serde(rename_all = "UPPERCASE")]
pub enum GameStatus {
    Lobby,
    Running,
    Finished,
}

pub struct Game {
    status: GameStatus,
    pub players: Vec<Player>,
    deck: Deck,
    turns_played: usize,
}

impl Game {
    pub fn new(author_name: String) -> Game {
        Game {
            status: GameStatus::Lobby,
            players: vec![Player::new(author_name, true)],
            deck: Deck::new(),
            turns_played: 0,
        }
    }

    pub fn find_player(&self, name: String) -> Option<&Player> {
        self.players
            .iter()
            .find(|player| player.name() == name)
    }

    pub fn find_author(&self) -> Option<&Player> {
        self.players.iter().find(|player| player.is_author)
    }

    pub fn add_player(&mut self, name: String) {
        self.players.push(Player::new(name, false))
    }

    pub fn get_finished_players(&self) -> Vec<&Player> {
        let mut result = self
            .players
            .iter()
            .filter(|p| p.is_finished())
            .collect::<Vec<&Player>>();
        result.sort_by_key(|player| player.position().unwrap());
        result
    }

    pub fn get_current_player(&self) -> Option<&Player> {
        self.players.get(self.turns_played % self.players.len())
    }

    pub fn next_turn(&mut self) {
        self.turns_played += 1;
    }

    pub fn serialize_status_message(&self, target_player_name: String) -> WSMessage {
        match self.status {
            GameStatus::Lobby => serde_json::to_string(&LobbyStatus::new(self, target_player_name)),
            GameStatus::Running => {
                serde_json::to_string(&RunningStatus::new(self, target_player_name))
            }
            GameStatus::Finished => {
                serde_json::to_string(&FinishedStatus::new(self, target_player_name))
            }
        }
        .unwrap()
    }
}
