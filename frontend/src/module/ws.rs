use crate::Game;
use crate::module::module::{LobbyStatus, RunningStatus};
use crate::pages::game::{GameState, Player};

pub fn handle_lobby(game: &mut Game, new_data: LobbyStatus) {
    game.status = GameState::Lobby;
    game.author = lobby.author;
    game.you = lobby.you;
    game.players = vec![];
    new_data.players.iter().for_each(|p| {
        game.players.push(Player {
            name: p.to_string(),
            cards: 0,
        })
    });
}

pub fn handle_running(game: &mut Game, new_data: RunningStatus) {
    game.status = GameState::Running;
    game.author = new_data.author;
    game.you = new_data.you;
    game.current_player = Some(new_data.current_player);
    game.players = new_data.players;
    game.cards = new_data.cards;
    game.discarted_card = new_data.top_card;
    game.clockwise = new_data.is_clockwise_direction;
}
