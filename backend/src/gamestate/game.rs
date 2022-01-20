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
    pub players: Vec<Player>,
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
