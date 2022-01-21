use crate::cards::deck::Deck;
use crate::gamestate::player::Player;
use crate::gamestate::CARDS_DEALT_AT_GAME_START;
use crate::ws::ws_message::WSMsg;
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use rand::Rng;
use rand::seq::SliceRandom;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone, Copy)]
#[serde(rename_all = "UPPERCASE")]
pub enum GameStatus {
    Lobby,
    Running,
    Finished,
}

#[derive(Clone)]
pub struct Game {
    status: GameStatus,
    players: Vec<Player>,
    deck: Deck,
    turns_played: usize,
    pub id: String,
}

impl Game {
    pub fn new(author_name: &String) -> Game {
        Game {
            status: GameStatus::Lobby,
            players: vec![Player::new(author_name.clone(), true)],
            deck: Deck::new(),
            turns_played: 0,
            id: nanoid!(10)
        }
    }

    pub fn start(&mut self) -> anyhow::Result<()> {
        if self.status == GameStatus::Running {
            anyhow::bail!("Attempted to start an already running game.")
        }

        self.randomize_player_order();
        self.randomize_starting_player();
        self.clear_player_positions();

        self.status = GameStatus::Running;
        self.deal_starting_cards()?; // must be called after clear_players(), of course

        // todo!("Send STATUS WSMessages to all players, don't have the API yet");

        Ok(())
    }

    fn randomize_player_order(&mut self) {
        self.players.shuffle(&mut rand::thread_rng())
    }

    /// Imitates a random starting player by pretending that some rounds have already been played.
    fn randomize_starting_player(&mut self) {
        self.turns_played = rand::thread_rng().gen_range(0..self.players.len());
    }

    fn clear_player_positions(&mut self) {
        for player in self.players.iter_mut() {
            player.clear_position();
        }
    }

    /// Clears all players' hands and gives them new cards from a new Deck.
    fn deal_starting_cards(&mut self) -> anyhow::Result<()> {
        self.deck = Deck::new();

        for player in self.players.iter_mut() {
            player.drop_all_cards();

            for _ in 0..CARDS_DEALT_AT_GAME_START {
                match self.deck.draw() {
                    None => anyhow::bail!("Draw pile is empty when starting game, this should not happen."),
                    Some(card) => player.give_card(card)
                }
            }
        }

        Ok(())
    }

    pub fn find_player(&self, name: String) -> Option<&Player> {
        self.players.iter().find(|player| player.name() == name)
    }

    pub fn find_author(&self) -> Option<&Player> {
        self.players.iter().find(|player| player.is_author)
    }

    pub fn players(&self) -> &Vec<Player> {
        &self.players
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

    pub fn status(&self) -> GameStatus {
        self.status
    }

    pub fn message_all(&self, msg: WSMsg) {
        for player in self.players.iter() {
            player.message(msg.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cards::card::{Card, CardColor, CardSymbol};
    use crate::gamestate::game::Game;
    use crate::gamestate::player::Player;

    #[test]
    fn test_find_player() {
        let mut game = Game::new("Andy".into());
        game.add_player("Bob".into());

        assert!(game.find_player("Andy".into()).is_some());
        assert!(game.find_player("Alice".into()).is_none());
    }

    #[test]
    fn test_current_next_players() {
        let mut game = Game::new("Andy".into());
        game.add_player("Bob".into());

        let current_player = game.get_current_player();
        assert!(current_player.is_some());
        assert_eq!(current_player.unwrap().name(), "Andy".to_string());

        game.next_turn();
        let current_player = game.get_current_player();
        assert!(current_player.is_some());
        assert_eq!(current_player.unwrap().name(), "Bob".to_string());

        game.next_turn();
        let current_player = game.get_current_player();
        assert!(current_player.is_some());
        assert_eq!(current_player.unwrap().name(), "Andy".to_string());
    }

    #[test]
    fn test_play_card() {
        let mut player = Player::new("Chuck".into(), true);

        assert!(player.play_card_by_index(0).is_err());

        player.give_card(Card::new(CardColor::Black, CardSymbol::Wild).unwrap());

        assert!(player.play_card_by_index(0).is_ok());
        assert!(player.play_card_by_index(1).is_err());
    }

    #[test]
    fn test_finished_players() {
        let mut game = Game::new("Andy".into());
        game.add_player("Bob".into());
        game.add_player("Danny".into());

        assert!(game.get_finished_players().is_empty());

        game.players.get_mut(0).unwrap().set_position(2); // Andy is second
        game.players.get_mut(1).unwrap().set_position(1); // Bob is first

        let finished = game.get_finished_players();
        assert_eq!(
            finished
                .into_iter()
                .map(|p| p.name())
                .collect::<Vec<String>>(),
            vec!["Bob".to_string(), "Andy".to_string()]
        );
    }
}