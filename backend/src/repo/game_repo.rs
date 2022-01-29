use std::{fmt::{Display, Formatter}, error::Error};

use crate::gamestate::game::Game;

#[derive(Debug)]
pub enum GameRepoError {
    GameNotFound(String),
}

impl Error for GameRepoError {}

impl Display for GameRepoError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use GameRepoError::*;

        match self {
            GameNotFound(id) => write!(f, "Game with id '{}' not found", id),
        }
    }
}

#[derive(Clone)]
pub struct InMemoryGameRepo {
    games: Vec<Game>,
}

impl InMemoryGameRepo {
    pub fn new() -> Self {
        Self { games: Vec::new() }
    }

    pub fn add_game(&mut self, game: Game) {
        self.games.push(game);
    }

    pub fn get_game_by_id_mut(&mut self, game_id: String) -> Result<&mut Game, GameRepoError> {
        match self.games.iter_mut().find(|game| game.id == game_id) {
            None => Err(GameRepoError::GameNotFound(game_id)),
            Some(game) => Ok(game),
        }
    }
}
