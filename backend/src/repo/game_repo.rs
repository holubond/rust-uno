use crate::gamestate::game::Game;

pub trait GameRepo {
    fn add_game(&mut self, name: Game);
    fn games(&mut self) -> &Vec<Game>;
    fn find_game_by_id(&self, game_id: String) -> Option<&Game>;
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

impl GameRepo for InMemoryGameRepo {
    fn add_game(&mut self, game: Game){
        self.games.push(game);
    }

    fn games(&mut self) -> &Vec<Game>{
        &self.games
    }

    fn find_game_by_id(&self, game_id: String) -> Option<&Game>{
        self.games.iter().find(|game| game.id == game_id)
    }
}
