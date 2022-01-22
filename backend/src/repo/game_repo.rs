use crate::gamestate::game::Game;
use async_trait::async_trait;

#[async_trait]
pub trait GameRepo {
    async fn create_game(&mut self, name: String) -> anyhow::Result<Game>;
}

#[derive(Clone)]
pub struct InMemoryGameRepo {
    games: Vec<Game>,
}

impl InMemoryGameRepo {
    pub fn new() -> Self {
        Self { 
            games: Vec::new()
        }
    }
}

#[async_trait]
impl GameRepo for InMemoryGameRepo {
    async fn create_game(&mut self, author_name: String) -> anyhow::Result<Game> {
        let game = Game::new(author_name);
        self.games.push(game.clone());

        return Ok(game);
    }
}
