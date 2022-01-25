use crate::gamestate::game::Game;

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

impl InMemoryGameRepo {
    pub fn add_game(&mut self, game: Game){
        self.games.push(game);
    }

    pub fn find_game_by_id_mut(&mut self, game_id: &String) -> Option<&mut Game>{
        self.games.iter_mut().find(|game| &game.id == game_id)
    }
}
