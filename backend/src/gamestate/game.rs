use crate::cards::card::{Card, CardColor, CardSymbol};
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
    pub id: String,
    status: GameStatus,
    players: Vec<Player>,
    deck: Deck,
    current_player: usize,
    /// An active card means that the current player must respond to that card, e.g. by being skipped, by drawing...
    is_top_card_active: bool,
    pub is_clockwise: bool,
}

impl Game {
    pub fn new(author_name: &String) -> Game {
        Game {
            id: nanoid!(10),
            status: GameStatus::Lobby,
            players: vec![Player::new(author_name.clone(), true)],
            deck: Deck::new(),
            current_player: 0,
            is_top_card_active: false,
            is_clockwise: true,
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
        self.current_player = rand::thread_rng().gen_range(0..self.players.len());
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

    pub fn deck(&self) -> &Deck {
        &self.deck
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
        self.players.get(self.current_player)
    }

    pub fn next_turn(&mut self) {
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

    pub fn reverse(&mut self) {
        self.is_clockwise = !self.is_clockwise
    }

    pub fn status(&self) -> GameStatus {
        self.status
    }

    pub fn message_all(&self, msg: WSMsg) {
        for player in self.players.iter() {
            player.message(msg.clone());
        }
    }

    pub fn can_play_card(&self, played_card: &Card) -> bool {
        let top_card = self.deck.top_discard_card();

        if self.active_cards.is_empty() {
            played_card.color == CardColor::Black
                || played_card.color == top_card.color
                || played_card.symbol == top_card.symbol
        } else {
            played_card.symbol == top_card.symbol
        }
    }

    fn does_player_exist(&self, player_name: String) -> anyhow::Result<&Player> {
        let player = self.find_player(player_name.clone());

        if player.is_none() {
            anyhow::bail!("Player of name {} does not exist!", player_name)
        }

        Ok(player.unwrap())
    }

    fn is_player_at_turn(&self, player: &Player) -> anyhow::Result<()> {
        match self.get_current_player() {
            None => anyhow::bail!("No player is currently playing?!"),
            Some(current_player) => {
                if player != current_player {
                    anyhow::bail!("It is not player {}'s turn right now!", player.name())
                } else {
                    Ok(())
                }
            }
        }
    }

    // Performs immutable checks whether the player is eligible to draw a card.
    fn can_player_draw(&self, player_name: String) -> anyhow::Result<()> {
        let player = self.does_player_exist(player_name.clone())?;
        self.is_player_at_turn(player)?;

        if player.cards().iter().any(|card| self.can_play_card(card)) {
            anyhow::bail!(
                "Player of name {} can play a card, no need to draw!",
                player_name
            )
        }

        Ok(())
    }

    /// Returns a cloned vector of what the player received as drawn cards.
    /// Returns an error if the player does not exist, is not the current player, or has a valid card to play.
    /// Should get called whenever a player clicks the draw card pile.
    pub fn draw_cards(&mut self, player_name: String) -> anyhow::Result<Vec<Card>> {
        self.can_player_draw(player_name.clone())?;

        let top_symbol = &self.deck.top_discard_card().symbol;
        let draw_count = if self.is_top_card_active && top_symbol == &CardSymbol::Draw2 {
            self.is_top_card_active = false;
            2
        } else if self.is_top_card_active && top_symbol == &CardSymbol::Draw4 {
            self.is_top_card_active = false;
            4
        } else {
            1
        };
        // Cannot be extracted to a method because the whole self will be borrowed mutably, not just self.players
        let player = self
            .players
            .iter_mut()
            .find(|player| player.name() == player_name)
            .unwrap(); // safe because of check_player_drawing()

        Ok(Game::draw_n_cards(player, &mut self.deck, draw_count))
    }

    // Again, the function's signature is like this due to mutability borrow-checker issues
    fn draw_n_cards(player: &mut Player, deck: &mut Deck, n: usize) -> Vec<Card> {
        let mut drawn_cards = vec![];

        for _ in 0..n {
            let drawn_card = deck.draw();
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

    // prerequisite for some other tests
    #[test]
    fn test_author_is_first_before_start() {
        let mut game = Game::new("Andy".into());
        assert_eq!(game.get_current_player().unwrap().name(), "Andy".to_string());
    }

    #[test]
    fn test_draw_cards_errors() {
        let mut game = Game::new("Andy".into());

        assert!(game.draw_cards("Bobby".into()).is_err()); // nonexistent player

        game.add_player("Bobby".into());
        assert!(game.draw_cards("Bobby".into()).is_err()); // not the current player

        let top_card = game.deck.top_discard_card().clone();
        game.players.get_mut(0).unwrap().give_card(top_card);
        assert!(game.draw_cards("Andy".into()).is_err()); // can definitely play the same card, doesn't need to draw

        game.deck.play(Card::new(CardColor::Black, CardSymbol::Draw4).unwrap().morph_black_card(CardColor::Blue).unwrap());
        game.players.get_mut(0).unwrap().give_card(Card::new(CardColor::Black, CardSymbol::Draw4).unwrap());
        assert!(game.draw_cards("Andy".into()).is_err()); // can definitely play +4 on a +4
    }

    #[test]
    fn test_draw_cards_draws() {
        let mut game = Game::new("Andy".into());
        game.deck.play(Card::new(CardColor::Blue, CardSymbol::Draw2).unwrap());
        game.is_top_card_active = true;

        assert_eq!(game.draw_cards("Andy".into()).unwrap().len(), 2);

        game.is_top_card_active = false;
        game.players.get_mut(0).unwrap().drop_all_cards();
        game.players.get_mut(0).unwrap().give_card(Card::new(CardColor::Red, CardSymbol::Value(2)).unwrap()); // cannot play this
        assert_eq!(game.draw_cards("Andy".into()).unwrap().len(), 1);
    }

    #[test]
    fn test_is_clockwise() {
        let mut game = Game::new("Andy".into());
        game.add_player("Bob".into());
        game.add_player("Candace".into());
        assert!(game.is_clockwise);

        assert_eq!(
            game.get_current_player().unwrap().name(),
            "Andy".to_string()
        );
        game.next_turn();
        assert_eq!(game.get_current_player().unwrap().name(), "Bob".to_string());
        game.next_turn();
        assert_eq!(
            game.get_current_player().unwrap().name(),
            "Candace".to_string()
        );
        game.next_turn();
        assert_eq!(
            game.get_current_player().unwrap().name(),
            "Andy".to_string()
        );

        game.reverse(); // Andy plays a reverse card
        assert!(!game.is_clockwise);

        game.next_turn();
        assert_eq!(
            game.get_current_player().unwrap().name(),
            "Candace".to_string()
        );
        game.next_turn();
        assert_eq!(game.get_current_player().unwrap().name(), "Bob".to_string());
        game.next_turn();
        assert_eq!(
            game.get_current_player().unwrap().name(),
            "Andy".to_string()
        );
        game.next_turn();
        assert_eq!(
            game.get_current_player().unwrap().name(),
            "Candace".to_string()
        );
    }

    #[test]
    fn test_can_play_card_without_context() {
        use CardColor::*;
        use CardSymbol::*;

        let mut game = Game::new("Andy".into());
        game.deck.play(Card::new(Red, Value(5)).unwrap());

        assert!(game.can_play_card(&Card::new(Red, Value(5)).unwrap()));
        assert!(game.can_play_card(&Card::new(Red, Value(6)).unwrap()));
        assert!(game.can_play_card(&Card::new(Blue, Value(5)).unwrap()));
        assert!(game.can_play_card(&Card::new(Red, Reverse).unwrap()));
        assert!(game.can_play_card(&Card::new(Black, Wild).unwrap()));
        assert!(game.can_play_card(&Card::new(Black, Draw4).unwrap()));

        assert!(!game.can_play_card(&Card::new(Blue, Value(6)).unwrap()));
        assert!(!game.can_play_card(&Card::new(Green, Draw2).unwrap()));
        assert!(!game.can_play_card(&Card::new(Yellow, Skip).unwrap()));

        game.deck.play(Card::new(Red, Draw2).unwrap());
        assert!(!game.is_top_card_active);
        assert!(game.can_play_card(&Card::new(Red, Draw2).unwrap()));
        assert!(game.can_play_card(&Card::new(Blue, Draw2).unwrap()));
        assert!(game.can_play_card(&Card::new(Red, Value(5)).unwrap()));
        assert!(game.can_play_card(&Card::new(Black, Wild).unwrap()));
        assert!(game.can_play_card(&Card::new(Black, Draw4).unwrap()));

        assert!(!game.can_play_card(&Card::new(Blue, Value(6)).unwrap()));
        assert!(!game.can_play_card(&Card::new(Green, Reverse).unwrap()));
        assert!(!game.can_play_card(&Card::new(Yellow, Skip).unwrap()));
    }

    #[test]
    fn test_can_play_card_with_context() {
        use CardColor::*;
        use CardSymbol::*;

        let mut game = Game::new("Andy".into());
        let plus_4 = Card::new(CardColor::Black, CardSymbol::Draw4)
            .unwrap()
            .morph_black_card(CardColor::Blue)
            .unwrap();
        game.deck.play(plus_4.clone());
        game.active_cards.push(plus_4.clone());

        assert!(game.can_play_card(&plus_4.clone()));
        assert!(game.can_play_card(&Card::new(Black, Draw4).unwrap()));

        assert!(!game.can_play_card(&Card::new(Red, Value(6)).unwrap()));
        assert!(!game.can_play_card(&Card::new(Blue, Value(5)).unwrap()));
        assert!(!game.can_play_card(&Card::new(Red, Reverse).unwrap()));
        assert!(!game.can_play_card(&Card::new(Black, Wild).unwrap()));
        assert!(!game.can_play_card(&Card::new(Blue, Value(6)).unwrap()));
        assert!(!game.can_play_card(&Card::new(Green, Draw2).unwrap()));
        assert!(!game.can_play_card(&Card::new(Yellow, Skip).unwrap()));

        game.deck.play(Card::new(Red, Draw2).unwrap());
        assert!(game.can_play_card(&Card::new(Red, Draw2).unwrap()));
        assert!(game.can_play_card(&Card::new(Blue, Draw2).unwrap()));
        assert!(game.can_play_card(&Card::new(Green, Draw2).unwrap()));
        assert!(game.can_play_card(&Card::new(Yellow, Draw2).unwrap()));

        assert!(!game.can_play_card(&Card::new(Red, Value(5)).unwrap()));
        assert!(!game.can_play_card(&Card::new(Black, Wild).unwrap()));
        assert!(!game.can_play_card(&Card::new(Black, Draw4).unwrap()));
        assert!(!game.can_play_card(&Card::new(Blue, Value(6)).unwrap()));
        assert!(!game.can_play_card(&Card::new(Green, Reverse).unwrap()));
        assert!(!game.can_play_card(&Card::new(Yellow, Skip).unwrap()));
    }
}
