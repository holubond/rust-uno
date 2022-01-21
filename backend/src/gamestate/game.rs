use crate::cards::deck::Deck;
use crate::gamestate::player::Player;
use nanoid::nanoid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone, Copy)]
pub enum GameStatus {
    Lobby,
    Running,
    Finished,
}

#[derive(Clone)]
pub struct Game {
    status: GameStatus,
    pub players: Vec<Player>,
    deck: Deck,
    turns_played: usize,
    pub id: String,
}

impl Game {
    pub fn new(author_name: String) -> Game {
        let id = nanoid!(10);
        Game {
            status: GameStatus::Lobby,
            players: vec![Player::new(author_name, true)],
            deck: Deck::new(),
            turns_played: 0,
            id,
        }
    }

    pub fn find_player(&self, name: String) -> Option<&Player> {
        self.players.iter().find(|player| player.name == name)
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
            .filter(|player| player.position != None)
            .collect::<Vec<&Player>>();
        result.sort_by_key(|player| player.position.unwrap());
        result
    }

    pub fn get_current_player(&self) -> Option<&Player> {
        self.players.get(self.turns_played % self.players.len())
    }

    pub fn next_turn(&mut self) {
        self.turns_played += 1;
    }
}
