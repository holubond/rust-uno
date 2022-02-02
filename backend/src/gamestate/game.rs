use crate::cards::card::{Card, CardColor, CardSymbol};
use crate::cards::deck::Deck;
use crate::err::add_player::AddPlayerError;
use crate::err::ai::AiError;
use crate::err::draw_cards::PlayerDrawError;
use crate::err::game_start::GameStartError;
use crate::err::play_card::PlayCardError;
use crate::err::player_exist::PlayerExistError;
use crate::err::player_turn::PlayerTurnError;
use crate::err::status::CreateStatusError;
use crate::gamestate::active_cards::ActiveCards;
use crate::gamestate::players::ai::{
    decide_new_color, decide_sleep_time, first_card_of_symbol, first_playable_card_against,
};
use crate::gamestate::players::player::Player;
use crate::gamestate::{CARDS_DEALT_TO_PLAYERS, PENALTY_CARDS};
use crate::ws::ws_message::WSMsg;
use nanoid::nanoid;
use rand::seq::SliceRandom;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::thread;

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
            players: vec![Player::new(author_name, true, true)],
            deck: Deck::new(),
            current_player: 0,
            active_cards: ActiveCards::new(),
            is_clockwise: true,
        }
    }

    pub fn new_with_ai(author_name: String, ai_count: usize) -> Game {
        let mut game = Game::new(author_name);
        (0..ai_count).for_each(|_| game.add_ai());
        game
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

        self.maybe_ai_turn()?;

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
        self.players.iter_mut().find(|player| player.name() == name)
    }

    pub fn find_author(&self) -> Option<&Player> {
        self.players.iter().find(|player| player.is_author())
    }

    /// Convenience method for accessing the reference to the game's Players.
    pub fn players(&self) -> &Vec<Player> {
        &self.players
    }

    /// Convenience method for accessing the reference to the game's Deck.
    pub fn deck(&self) -> &Deck {
        &self.deck
    }

    pub fn add_player(&mut self, name: String) -> Result<(), AddPlayerError> {
        if self.find_player(name.clone()).is_some() {
            return Err(AddPlayerError::AlreadyExists(name.clone()));
        }

        self.players.push(Player::new(name, false, true));
        self.status_message_all()?;

        Ok(())
    }

    pub fn add_ai(&mut self) {
        self.players.push(Player::new_ai())
    }

    pub fn get_finished_players(&self) -> Vec<&Player> {
        let mut result = self
            .players
            .iter()
            .filter(|p| p.is_finished())
            .collect::<Vec<&Player>>();
        result.sort_by_key(|player| player.position().unwrap()); // safe due to p.is_finished() filter above
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
            played_card.symbol == self.active_cards.active_symbol().unwrap() // save due to are_cards_active() check above
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

    fn end_drawing(
        &mut self,
        drawing_player: String,
        cards_drawn: Vec<Card>,
    ) -> Result<(), PlayerDrawError> {
        self.end_turn();

        // player name after end_turn == next player
        let next_player_name = match self.get_current_player() {
            None => return Err(PlayerDrawError::from(CreateStatusError::CurrentPlayerNotFound)),
            Some(player) => player.name()
        };
        self.message_all_but(
            drawing_player.clone(),
            WSMsg::draw(
                drawing_player.clone(),
                next_player_name.clone(),
                cards_drawn.len(),
            ),
        );
        match self.find_player(drawing_player.clone()) {
            None => return Err(PlayerExistError::NoSuchPlayer(drawing_player).into()),
            Some(player) => player.message(WSMsg::draw_me(next_player_name, cards_drawn))
        }

        self.maybe_ai_turn()?;

        Ok(())
    }

    /// Returns a cloned vector of what the player received as drawn cards.
    /// Returns an error if the player does not exist, is not the current player, or has a valid card to play.
    /// Should get called whenever a player clicks the draw card pile.
    pub fn draw_cards(&mut self, player_name: String) -> Result<(), PlayerDrawError> {
        self.can_player_draw(player_name.clone())?;

        // Skip turn
        if self.active_cards.are_cards_active()
            && self.active_cards.active_symbol().unwrap() == CardSymbol::Skip // safe since are_cards_active() check above
        {
            self.active_cards.clear();
            return self.end_drawing(player_name, vec![]);
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

        debug_assert_eq!(draw_count, drawn_cards.len());
        self.end_drawing(player_name, drawn_cards)
    }

    /// Draws n cards from the deck and gives them to the named player.
    /// Returns a clone of the cards drawn.
    fn draw_n_cards(&mut self, player_name: String, n: usize) -> Vec<Card> {
        let player = self
            .players
            .iter_mut()
            .find(|player| player.name() == player_name)
            .unwrap(); // safe because of can_player_draw() in draw_cards()
        let mut drawn_cards = vec![];

        for _ in 0..n {
            let drawn_card = self.deck.draw();
            if drawn_card.is_none() {
                // there are no cards on the table at all
                break;
            }
            let drawn_card = drawn_card.unwrap(); // safe since is_none() check + early break above

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
        self.maybe_ai_turn()?;

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
            .unwrap(); // safe because of can_player_play() in play_card()

        let should_have_said_uno = player.should_say_uno(); // acquired before removing a card from players' hands
        let mut played_card = player.play_card(wanted_card)?;

        if played_card.should_be_black() {
            if let Some(color) = maybe_new_color {
                played_card = played_card.morph_black_card(color).unwrap(); // safe because of should_be_black() check above
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
                self.active_cards.push(played_card.clone()).unwrap(); // match guard corresponds with ALLOWED_ACTIVE_CARDS in ActiveCards
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
        let next_player_name = match self.get_current_player() {
            None => return Err(CreateStatusError::CurrentPlayerNotFound),
            Some(player) => player.name(),
        };
        self.message_all(WSMsg::play_card(
            player_name.clone(),
            next_player_name,
            played_card,
        ));

        if player_penalized {
            let gained_cards = self.draw_n_cards(player_name.clone(), PENALTY_CARDS);
            self.message_all_but(
                player_name.clone(),
                WSMsg::gained_cards(player_name.clone(), gained_cards.len()),
            );
            self.find_player(player_name.clone())
                .unwrap() // safe because of can_player_play() in play_card()
                .message(WSMsg::penalty(player_name.clone(), gained_cards));
        }

        if player_finished {
            self.message_all(WSMsg::finish(player_name));
        }

        if self
            .players
            .len()
            .saturating_sub(self.get_finished_players().len())
            <= 1
        {
            // == the difference between all players and finished players is 0 or 1
            self.finish_all_unfinished_players();

            self.status = GameStatus::Finished;
            self.status_message_all()?;
        }

        Ok(())
    }

    fn human_iter(&self) -> impl Iterator<Item=&Player> {
        self.players.iter().filter(|player| player.is_human())
    }

    /// Finishes all players that are not yet finished.
    /// Sends Finish WS Messages to all players appropriately.
    fn finish_all_unfinished_players(&mut self) {
        if self.get_finished_players().is_empty() {
            return;
        }

        let last_position = self
            .get_finished_players()
            .last()
            .unwrap() // safe since get_finished_players().is_empty() check above (not empty => last will succeed)
            .position()
            .unwrap(); // safe since we are iterating get_finished_players()

        let mut newly_finished = vec![];
        for (index, ai) in self.players.iter_mut().filter(|p| !p.is_finished()).enumerate() {
            ai.set_position(last_position + index + 1);
            newly_finished.push(ai.name());
        }

        // has to be a separate for-loop due tu mutability reasons
        for ai_name in newly_finished {
            self.message_all(WSMsg::finish(ai_name));
        }
    }

    fn maybe_ai_turn(&mut self) -> Result<(), AiError> {
        {
            // inner scope due to mutable borrowing later
            let maybe_current_player = self.get_current_player();
            if maybe_current_player.is_none() {
                // if there is no current player, don't attempt to play
                return Ok(());
            }

            let current_player = maybe_current_player.unwrap(); // safe since is_none() check + early return above
            if current_player.is_human() {
                return Ok(());
            }
        }

        if !self.get_finished_players().is_empty()
            && self.human_iter().all(|player| player.is_finished())
        {
            // if all humans are finished, finish all other (i.e. ai) players
            self.finish_all_unfinished_players();

            self.status = GameStatus::Finished;
            self.status_message_all()?;

            return Ok(());
        }

        // todo!("simulate AI decision making with non-blocking sleep");
        thread::sleep(decide_sleep_time());

        let current_player = match self.get_current_player() {
            None => return Err(AiError::from(CreateStatusError::CurrentPlayerNotFound)),
            Some(player) => player
        };
        let ai_name = current_player.name();

        if let Some(card) = match self.active_cards.are_cards_active() {
            true => {
                first_card_of_symbol(current_player, self.active_cards.active_symbol().unwrap()) // safe since are_cards_active() check above
            }
            false => first_playable_card_against(current_player, self.deck.top_discard_card()),
        } {
            let should_say_uno = current_player.should_say_uno();
            let new_color = decide_new_color(&card);

            self.play_card(ai_name, card, new_color, should_say_uno)?;
        } else {
            self.draw_cards(ai_name)?;
        }

        Ok(())
    }
}
