use crate::cards::card::{Card, CardColor, CardSymbol};
use crate::cards::deck::Deck;
use crate::err::draw_cards::PlayerDrawError;
use crate::err::game_start::GameStartError;
use crate::err::play_card::PlayCardError;
use crate::err::player_exist::PlayerExistError;
use crate::err::player_turn::PlayerTurnError;
use crate::err::status::CreateStatusError;
use crate::gamestate::active_cards::ActiveCards;
use crate::gamestate::player::Player;
use crate::gamestate::{CARDS_DEALT_TO_PLAYERS, PENALTY_CARDS};
use crate::ws::ws_message::WSMsg;
use nanoid::nanoid;
use rand::seq::SliceRandom;
use rand::Rng;
use serde::{Deserialize, Serialize};

#[cfg(test)]
#[path = "../tests/game_test.rs"]
mod tests;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone, Copy)]
#[serde(rename_all = "UPPERCASE")]
pub enum GameStatus {
    Lobby,
    Running,
    Finished,
}

#[derive(Debug, Clone)]
pub struct Game {
    pub id: String,
    status: GameStatus,
    players: Vec<Player>,
    deck: Deck,
    current_player: usize,
    /// An active card means that the current player must respond to that card, i.e. by being skipped or by drawing.
    active_cards: ActiveCards,
    pub is_clockwise: bool,
}

impl Game {
    pub fn new(author_name: String) -> Game {
        Game {
            id: nanoid!(10),
            status: GameStatus::Lobby,
            players: vec![Player::new(author_name, true)],
            deck: Deck::new(),
            current_player: 0,
            active_cards: ActiveCards::new(),
            is_clockwise: true,
        }
    }

    /// Randomizes player order and start, clears positions from previous games, resets the deck and deals cards to players.
    /// Returns Err is the game is already Running.
    pub fn start(&mut self) -> Result<(), GameStartError> {
        if self.status == GameStatus::Running {
            return Err(GameStartError::GameAlreadyStarted);
        }

        self.randomize_player_order();
        self.randomize_starting_player();
        self.clear_player_positions();

        self.status = GameStatus::Running;
        self.deal_starting_cards()?;

        self.status_message_all()?;

        Ok(())
    }

    fn randomize_player_order(&mut self) {
        self.players.shuffle(&mut rand::thread_rng())
    }

    /// Imitates a random starting player by pretending that some rounds have already been played.
    fn randomize_starting_player(&mut self) {
        self.current_player = rand::thread_rng().gen_range(0..self.players.len());
    }

    fn clear_player_positions(&mut self) {
        for player in self.players.iter_mut() {
            player.clear_position();
        }
    }

    /// Clears all players' hands and gives them new cards from a new Deck.
    fn deal_starting_cards(&mut self) -> Result<(), GameStartError> {
        self.deck = Deck::new();

        for player in self.players.iter_mut() {
            player.drop_all_cards();

            for _ in 0..CARDS_DEALT_TO_PLAYERS {
                match self.deck.draw() {
                    None => return Err(GameStartError::DeckEmptyWhenStartingGame),
                    Some(card) => player.give_card(card),
                }
            }
        }

        Ok(())
    }

    pub fn find_player(&self, name: String) -> Option<&Player> {
        self.players.iter().find(|player| player.name() == name)
    }

    pub fn find_player_mut(&mut self, name: &str) -> Option<&mut Player> {
        self.players
            .iter_mut()
            .find(|player| player.name() == name)
    }

    pub fn find_author(&self) -> Option<&Player> {
        self.players.iter().find(|player| player.is_author)
    }

    /// Convenience method for accessing the reference to the game's Players.
    pub fn players(&self) -> &Vec<Player> {
        &self.players
    }

    /// Convenience method for accessing the reference to the game's Deck.
    pub fn deck(&self) -> &Deck {
        &self.deck
    }

    pub fn add_player(&mut self, name: String) -> Result<(), CreateStatusError> {
        self.players.push(Player::new(name, false));
        self.status_message_all()
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
        self.players.get(self.current_player)
    }

    fn next_turn(&mut self) {
        self.current_player = if self.is_clockwise {
            self.current_player + 1
        } else {
            match self.current_player.checked_sub(1) {
                None => self.players.len() - 1,
                Some(number) => number,
            }
        }
        .rem_euclid(self.players.len());
    }

    /// Attempts to find the next player in line. Returns true if found, false otherwise.
    pub fn end_turn(&mut self) -> bool {
        if self.get_finished_players().len() == self.players.len() {
            return false;
        }

        loop {
            self.next_turn();

            if let Some(player) = self.get_current_player() {
                if !player.is_finished() {
                    break;
                }
            }
        }

        true
    }

    pub fn reverse(&mut self) {
        self.is_clockwise = !self.is_clockwise
    }

    pub fn status(&self) -> GameStatus {
        self.status
    }

    /// Sends a personalized (==containing name) STATUS WSMessage to all players.
    fn status_message_all(&self) -> Result<(), CreateStatusError> {
        for player in self.players.iter() {
            player.message(WSMsg::status(self, player.name())?);
        }

        Ok(())
    }

    pub fn message_all(&self, msg: WSMsg) {
        for player in self.players.iter() {
            player.message(msg.clone());
        }
    }

    pub fn message_all_but(&self, excluded_player_name: String, msg: WSMsg) {
        for player in self.players.iter() {
            if player.name() != excluded_player_name {
                player.message(msg.clone());
            }
        }
    }

    /// If there are any active cards, returns true only if the played_card's symbol matches:
    /// e.g. playing a Blue Skip on a Red Skip.
    /// If there are no active cards, returns true if the played_card's symbol OR color matches, or it is a Black card.
    pub fn can_play_card(&self, played_card: &Card) -> bool {
        let top_card = self.deck.top_discard_card();

        if self.active_cards.are_cards_active() {
            played_card.symbol == self.active_cards.active_symbol().unwrap()
        } else {
            played_card.color == CardColor::Black
                || played_card.color == top_card.color
                || played_card.symbol == top_card.symbol
        }
    }

    /// Returns reference to a player matching the provided name, Err if they do not exist.
    fn does_player_exist(&self, player_name: String) -> Result<&Player, PlayerExistError> {
        let maybe_player = self.find_player(player_name.clone());

        if let Some(player) = maybe_player {
            Ok(player)
        } else {
            Err(PlayerExistError::NoSuchPlayer(player_name))
        }
    }

    /// Returns Err if the passed player is not the current player, or if there is somehow no player playing.
    fn is_player_at_turn(&self, player: &Player) -> Result<(), PlayerTurnError> {
        match self.get_current_player() {
            None => Err(PlayerTurnError::NoOneIsPlaying),
            Some(current_player) => {
                if player != current_player {
                    Err(PlayerTurnError::PlayerOutOfTurn(player.name()))
                } else {
                    Ok(())
                }
            }
        }
    }

    /// Performs immutable checks whether the player is eligible to draw a card.
    fn can_player_draw(&self, player_name: String) -> Result<(), PlayerDrawError> {
        let player = self.does_player_exist(player_name)?;
        self.is_player_at_turn(player)?;

        if player.cards().iter().any(|card| self.can_play_card(card)) {
            return Err(PlayerDrawError::CanPlayInstead);
        }

        Ok(())
    }

    /// Returns a cloned vector of what the player received as drawn cards.
    /// Returns an error if the player does not exist, is not the current player, or has a valid card to play.
    /// Should get called whenever a player clicks the draw card pile.
    pub fn draw_cards(&mut self, player_name: String) -> Result<Vec<Card>, PlayerDrawError> {
        self.can_player_draw(player_name.clone())?;

        // Skip turn
        if self.active_cards.are_cards_active()
            && self.active_cards.active_symbol().unwrap() == CardSymbol::Skip
        {
            self.end_turn();
            self.active_cards.clear();
            self.message_all_but(
                player_name.clone(),
                WSMsg::draw(player_name, self.get_current_player().unwrap().name(), 0),
            );
            return Ok(vec![]);
        }

        let draw_count = if self.active_cards.are_cards_active() {
            let count = self.active_cards.sum_active_draw_cards().expect(
                "Impossible: player can draw, but there are active cards that are not Draw",
            );
            self.active_cards.clear();
            count
        } else {
            1
        };
        let drawn_cards = self.draw_n_cards(player_name.clone(), draw_count);

        self.end_turn();
        self.message_all_but(
            player_name.clone(),
            WSMsg::draw(
                player_name,
                self.get_current_player().unwrap().name(),
                draw_count,
            ),
        );

        Ok(drawn_cards)
    }

    /// Draws n cards from the deck and gives them to the named player.
    /// Returns a clone of the cards drawn.
    fn draw_n_cards(&mut self, player_name: String, n: usize) -> Vec<Card> {
        let player = self
            .players
            .iter_mut()
            .find(|player| player.name() == player_name)
            .unwrap(); // safe because of check_player_drawing()
        let mut drawn_cards = vec![];

        for _ in 0..n {
            let drawn_card = self.deck.draw();
            if drawn_card.is_none() {
                // there are no cards on the table at all
                break;
            }
            let drawn_card = drawn_card.unwrap();

            drawn_cards.push(drawn_card.clone());
            player.give_card(drawn_card);
        }

        drawn_cards
    }

    /// Performs immutable checks whether the player is eligible to play a card.
    fn can_player_play(
        &self,
        player_name: String,
        card: &Card,
        said_uno: bool,
    ) -> Result<(), PlayCardError> {
        let player = self.does_player_exist(player_name)?;

        self.is_player_at_turn(player)?;

        if !player.should_say_uno() && said_uno {
            return Err(PlayCardError::SaidUnoWhenShouldNotHave);
        }

        if !self.can_play_card(card) {
            return Err(PlayCardError::CardCannotBePlayed(
                card.clone(),
                self.deck.top_discard_card().clone(),
            ));
        }

        Ok(())
    }

    pub fn play_card(
        &mut self,
        player_name: String,
        card: Card,
        maybe_new_color: Option<CardColor>,
        said_uno: bool,
    ) -> Result<(), PlayCardError> {
        self.can_player_play(player_name.clone(), &card, said_uno)?;

        // required to be borrowed before mutable section
        let possible_position = self.get_finished_players().len();
        let (played_card, player_finished, should_say_uno) =
            self.mutate_player(&player_name, card, maybe_new_color, possible_position)?;

        self.handle_played_card(&played_card);
        self.deck.play(played_card.clone());
        self.end_turn();
        self.play_card_messages(
            player_finished,
            player_name,
            played_card,
            should_say_uno && !said_uno,
        )?;

        Ok(())
    }

    fn mutate_player(
        &mut self,
        player_name: &str,
        wanted_card: Card,
        maybe_new_color: Option<CardColor>,
        possible_position: usize,
    ) -> Result<(Card, bool, bool), PlayCardError> {
        let player = self
            .players
            .iter_mut()
            .find(|player| player.name() == *player_name)
            .unwrap();

        let should_have_said_uno = player.should_say_uno(); // acquired before removing a card from players' hands
        let mut played_card = player.play_card_by_eq(wanted_card)?;

        if played_card.should_be_black() {
            if let Some(color) = maybe_new_color {
                played_card = played_card.morph_black_card(color).unwrap();
            }
        }

        let player_finished = player.cards().is_empty();
        if player_finished {
            player.set_position(possible_position);
        }

        Ok((played_card, player_finished, should_have_said_uno))
    }

    fn handle_played_card(&mut self, played_card: &Card) {
        match played_card.symbol {
            CardSymbol::Value(_) | CardSymbol::Wild => self.active_cards.clear(),
            CardSymbol::Reverse => {
                self.reverse();
                self.active_cards.clear();
            }
            CardSymbol::Draw2 | CardSymbol::Draw4 | CardSymbol::Skip => {
                self.active_cards.push(played_card.clone()).unwrap();
            }
        }
    }

    /// Assumes player_name is a valid player name, meaning that such a player exists.
    fn play_card_messages(
        &mut self,
        player_finished: bool,
        player_name: String,
        played_card: Card,
        player_penalized: bool,
    ) -> Result<(), CreateStatusError> {
        self.message_all(WSMsg::play_card(
            player_name.clone(),
            self.get_current_player().unwrap().name(),
            played_card,
        ));

        if player_penalized {
            let gained_cards = self.draw_n_cards(player_name.clone(), PENALTY_CARDS);
            self.message_all_but(
                player_name.clone(),
                WSMsg::gained_cards(player_name.clone(), gained_cards.len()),
            );
            self.find_player(player_name.clone())
                .unwrap()
                .message(WSMsg::penalty(player_name.clone(), gained_cards));
        }

        if player_finished {
            self.message_all(WSMsg::finish(player_name.clone()));
        }

        if self.players.len().saturating_sub(self.get_finished_players().len()) <= 1 {
            // == after end_turn(), the same player got the turn
            self.status = GameStatus::Finished;
            self.status_message_all()?;
        }

        Ok(())
    }
}
