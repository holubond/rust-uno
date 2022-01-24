use crate::gamestate::game::Game;

pub trait GameRepo {
    fn add_game(&mut self, name: Game);
}

#[derive(Clone)]
pub struct InMemoryGameRepo {
    pub(crate) games: Vec<Game>,
}

impl InMemoryGameRepo {
    pub fn new() -> Self {
        Self { 
            games: Vec::new()
        }
    }
}

impl GameRepo for InMemoryGameRepo {
    fn add_game(&mut self, game: Game){
        self.games.push(game);
    }
}
