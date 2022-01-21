pub mod game;
pub mod player;
pub mod serialization;

pub type WSMessage = String;

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
        assert_eq!(current_player.unwrap().get_name_clone(), "Andy".to_string());

        game.next_turn();
        let current_player = game.get_current_player();
        assert!(current_player.is_some());
        assert_eq!(current_player.unwrap().get_name_clone(), "Bob".to_string());

        game.next_turn();
        let current_player = game.get_current_player();
        assert!(current_player.is_some());
        assert_eq!(current_player.unwrap().get_name_clone(), "Andy".to_string());
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
                .map(|p| p.get_name_clone())
                .collect::<Vec<String>>(),
            vec!["Bob".to_string(), "Andy".to_string()]
        );
    }
}
