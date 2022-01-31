use crate::cards::card::{Card, CardColor, CardSymbol};
use crate::gamestate::game::{Game, GameStatus};
use crate::gamestate::players::player::Player;
use crate::gamestate::CARDS_DEALT_TO_PLAYERS;

static CARDS_TOTAL_IN_GAME: usize = 108;

#[test]
fn test_find_player() {
    let mut game = Game::new("Andy".into());
    game.add_player("Bob".into()).unwrap();

    assert!(game.find_player("Andy".into()).is_some());
    assert!(game.find_player("Alice".into()).is_none());
}

#[test]
fn test_current_next_players() {
    let mut game = Game::new("Andy".into());
    game.add_player("Bob".into()).unwrap();

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
    let mut player = Player::new("Chuck".into(), true, true);

    assert!(player.play_card_by_index(0).is_err());

    player.give_card(Card::new(CardColor::Black, CardSymbol::Wild).unwrap());

    assert!(player.play_card_by_index(0).is_ok());
    assert!(player.play_card_by_index(1).is_err());
}

#[test]
fn test_finished_players() {
    let mut game = Game::new("Andy".into());
    game.add_player("Bob".into()).unwrap();
    game.add_player("Danny".into()).unwrap();

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
    let game = Game::new("Andy".into());
    assert_eq!(
        game.get_current_player().unwrap().name(),
        "Andy".to_string()
    );
}

#[test]
fn test_draw_cards_errors() {
    let mut game = Game::new("Andy".into());

    assert!(game.draw_cards("Bobby".into()).is_err()); // nonexistent player

    game.add_player("Bobby".into()).unwrap();
    assert!(game.draw_cards("Bobby".into()).is_err()); // not the current player

    let top_card = game.deck.top_discard_card().clone();
    game.players.get_mut(0).unwrap().give_card(top_card);
    assert!(game.draw_cards("Andy".into()).is_err()); // can definitely play the same card, doesn't need to draw

    game.deck.play(
        Card::new(CardColor::Black, CardSymbol::Draw4)
            .unwrap()
            .morph_black_card(CardColor::Blue)
            .unwrap(),
    );
    game.players
        .get_mut(0)
        .unwrap()
        .give_card(Card::new(CardColor::Black, CardSymbol::Draw4).unwrap());
    assert!(game.draw_cards("Andy".into()).is_err()); // can definitely play +4 on a +4
}

#[test]
fn test_draw_cards_draws() {
    let mut game = Game::new("Andy".into());
    game.deck
        .play(Card::new(CardColor::Blue, CardSymbol::Draw2).unwrap());
    assert!(game
        .active_cards
        .push(game.deck.top_discard_card().clone())
        .is_ok());

    assert_eq!(game.draw_cards("Andy".into()).unwrap().len(), 2);

    game.active_cards.clear();
    game.players.get_mut(0).unwrap().drop_all_cards();
    game.players
        .get_mut(0)
        .unwrap()
        .give_card(Card::new(CardColor::Red, CardSymbol::Value(2)).unwrap()); // cannot play this
    assert_eq!(game.draw_cards("Andy".into()).unwrap().len(), 1);
}

#[test]
fn test_is_clockwise() {
    let mut game = Game::new("Andy".into());
    game.add_player("Bob".into()).unwrap();
    game.add_player("Candace".into()).unwrap();
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
    assert!(!game.active_cards.are_cards_active());
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
    game.active_cards.push(plus_4.clone()).unwrap();

    assert!(game.can_play_card(&plus_4.clone()));
    assert!(game.can_play_card(&Card::new(Black, Draw4).unwrap()));

    assert!(!game.can_play_card(&Card::new(Red, Value(6)).unwrap()));
    assert!(!game.can_play_card(&Card::new(Blue, Value(5)).unwrap()));
    assert!(!game.can_play_card(&Card::new(Red, Reverse).unwrap()));
    assert!(!game.can_play_card(&Card::new(Black, Wild).unwrap()));
    assert!(!game.can_play_card(&Card::new(Blue, Value(6)).unwrap()));
    assert!(!game.can_play_card(&Card::new(Green, Draw2).unwrap()));
    assert!(!game.can_play_card(&Card::new(Yellow, Skip).unwrap()));

    let plus_2 = Card::new(Red, Draw2).unwrap();
    game.deck.play(plus_2.clone());
    game.active_cards.clear();
    game.active_cards.push(plus_2.clone()).unwrap();

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

#[test]
fn test_start_game() {
    let mut game = Game::new("Andy".into());
    game.add_player("Bob".into()).unwrap();
    game.add_player("Candace".into()).unwrap();

    assert!(game.start().is_ok());
    for player in game.players() {
        assert_eq!(player.cards().len(), CARDS_DEALT_TO_PLAYERS);
    }
    assert_eq!(game.deck.discard_pile_size(), 1);
    assert_eq!(
        game.deck.draw_pile_size(),
        CARDS_TOTAL_IN_GAME - (game.players.len() * CARDS_DEALT_TO_PLAYERS) - 1
    );
}

#[test]
fn test_start_game_errors() {
    let mut game = Game::new("Andy".into());
    game.add_player("Bob".into()).unwrap();
    game.add_player("Candace".into()).unwrap();

    game.status = GameStatus::Running;
    assert!(game.start().is_err()); // cannot start running game

    game.status = GameStatus::Lobby; // reset
    for _ in 0..106 {
        // simulate cards leaving deck completely
        let _card = game.deck.draw().unwrap();
    }
    assert!(game.start().is_ok()); // game creates a completely new deck, does not rely on previous one
}

#[test]
fn test_active_cards() {
    use CardColor::*;
    use CardSymbol::*;

    let mut game = Game::new("Andy".into());
    assert_eq!(game.active_cards.active_symbol(), None);
    assert_eq!(game.active_cards.sum_active_draw_cards(), None);

    let red_plus_2 = Card::new(Red, Draw2).unwrap();
    game.deck.play(red_plus_2.clone());
    game.active_cards.clear();
    game.active_cards.push(red_plus_2.clone()).unwrap();

    // different symbol
    assert!(game
        .active_cards
        .push(Card::new(Red, Skip).unwrap())
        .is_err());

    let blu_plus_2 = Card::new(Blue, Draw2).unwrap();
    let blu_skip = Card::new(Blue, Skip).unwrap();
    let green_skip = Card::new(Green, Skip).unwrap();
    {
        let andy = game.players.get_mut(0).unwrap();
        andy.give_card(blu_plus_2.clone());
        andy.give_card(blu_skip.clone());
        andy.give_card(green_skip.clone());
    }
    assert_eq!(
        game.find_player("Andy".into()).unwrap().cards(),
        vec![blu_plus_2.clone(), blu_skip.clone(), green_skip.clone()]
    );
    assert!(game
        .play_card("Andy".into(), blu_skip.clone(), None, false)
        .is_err()); // must respond to draw2
    assert!(game
        .play_card("Andy".into(), blu_plus_2.clone(), None, false)
        .is_ok());
    assert_eq!(
        game.find_player("Andy".into()).unwrap().cards(),
        vec![blu_skip.clone(), green_skip.clone()]
    );
    assert_eq!(game.active_cards.active_symbol().unwrap(), Draw2);
    assert_eq!(game.active_cards.sum_active_draw_cards(), Some(4)); // 2 from before + 2 from Andy

    let eight = Card::new(Blue, Value(8)).unwrap();
    game.deck.play(eight.clone());
    game.active_cards.clear();
    assert!(game.active_cards.push(eight.clone()).is_err());
    assert!(!game.active_cards.are_cards_active());

    assert!(game
        .play_card("Andy".into(), blu_skip.clone(), None, true)
        .is_ok());
    assert_eq!(
        game.find_player("Andy".into()).unwrap().cards(),
        vec![green_skip.clone()]
    );
    assert_eq!(
        game.deck.top_discard_card().symbol,
        game.active_cards.active_symbol().unwrap()
    );
    {
        let game_before = game.clone();
        let andy = game.players.get_mut(0).unwrap();
        assert!(
            andy.play_card(blu_skip.clone()).is_err(),
            "Before: \n{:?}\nAfter: \n{:?}",
            game_before,
            game.clone()
        ); // card is no longer in Andy's hand
    }
    assert_eq!(game.active_cards.active_symbol(), Some(Skip));
    assert_eq!(game.active_cards.sum_active_draw_cards(), None);

    assert!(game
        .play_card("Andy".into(), green_skip.clone(), None, false)
        .is_ok());
    assert_eq!(game.active_cards.active_symbol(), Some(Skip));
    assert_eq!(game.active_cards.sum_active_draw_cards(), None);
}

#[test]
fn test_end_turn() {
    let mut game = Game::new("Andy".into());
    game.add_player("Bob".into()).unwrap();
    game.add_player("Candace".into()).unwrap();
    game.add_player("Danny".into()).unwrap();
    game.add_player("Eli".into()).unwrap();
    game.add_player("Farquaad".into()).unwrap();

    assert!(game.is_clockwise);
    assert_eq!(
        game.get_current_player().unwrap().name(),
        "Andy".to_string()
    );

    assert!(game.end_turn());
    assert_eq!(game.get_current_player().unwrap().name(), "Bob".to_string());
    assert!(game.end_turn());
    assert_eq!(
        game.get_current_player().unwrap().name(),
        "Candace".to_string()
    );
    assert!(game.end_turn());
    assert_eq!(
        game.get_current_player().unwrap().name(),
        "Danny".to_string()
    );
    assert!(game.end_turn());
    assert_eq!(game.get_current_player().unwrap().name(), "Eli".to_string());
    assert!(game.end_turn());
    assert_eq!(
        game.get_current_player().unwrap().name(),
        "Farquaad".to_string()
    );
    assert!(game.end_turn());
    assert_eq!(
        game.get_current_player().unwrap().name(),
        "Andy".to_string()
    );

    // simulate Bob finishing
    game.players.get_mut(1).unwrap().set_position(1);

    assert!(game.end_turn());
    assert_eq!(
        game.get_current_player().unwrap().name(),
        "Candace".to_string()
    );
    assert!(game.end_turn());
    assert_eq!(
        game.get_current_player().unwrap().name(),
        "Danny".to_string()
    );
    assert!(game.end_turn());
    assert_eq!(game.get_current_player().unwrap().name(), "Eli".to_string());
    assert!(game.end_turn());
    assert_eq!(
        game.get_current_player().unwrap().name(),
        "Farquaad".to_string()
    );
    assert!(game.end_turn());
    assert_eq!(
        game.get_current_player().unwrap().name(),
        "Andy".to_string()
    );

    // simulate everyone but Candace finishing
    game.players.get_mut(0).unwrap().set_position(2);
    game.players.get_mut(3).unwrap().set_position(3);
    game.players.get_mut(4).unwrap().set_position(4);
    game.players.get_mut(5).unwrap().set_position(5);
    assert!(game.end_turn());
    assert_eq!(
        game.get_current_player().unwrap().name(),
        "Candace".to_string()
    );

    // the game should end by this point, but lets make sure the end_turn doesn't loop endlessly
    assert!(game.end_turn());
    assert_eq!(
        game.get_current_player().unwrap().name(),
        "Candace".to_string()
    );
    game.players.get_mut(2).unwrap().set_position(6);
    assert!(!game.end_turn());
}

#[test]
fn test_should_say_uno() {
    let mut game = Game::new("Andy".into());
    assert!(!game.find_player("Andy".into()).unwrap().should_say_uno()); // 0

    game.draw_n_cards("Andy".into(), 1);
    assert!(!game.find_player("Andy".into()).unwrap().should_say_uno()); // 1

    game.draw_n_cards("Andy".into(), 1);
    assert!(game.find_player("Andy".into()).unwrap().should_say_uno()); // 2

    game.draw_n_cards("Andy".into(), 1);
    assert!(!game.find_player("Andy".into()).unwrap().should_say_uno()); // 3...
}

#[test]
fn test_say_uno() {
    use CardColor::*;
    use CardSymbol::*;

    let mut game = Game::new("Andy".into());
    game.add_player("Bob".into()).unwrap();
    game.add_player("Candace".into()).unwrap();
    game.add_player("Danny".into()).unwrap();
    let eight = Card::new(Blue, Value(8)).unwrap();
    game.deck.play(eight.clone());

    // didn't say uno and shouldn't have
    game.players.get_mut(0).unwrap().give_card(eight.clone()); // should be playable
    assert!(game
        .play_card("Andy".into(), eight.clone(), None, false)
        .is_ok());
    assert!(game.players.get(0).unwrap().is_finished());
    assert_eq!(game.players.get(0).unwrap().get_card_count(), 0); // should not receive penalty cards

    // didn't say uno and should have
    game.players.get_mut(1).unwrap().give_card(eight.clone());
    game.players.get_mut(1).unwrap().give_card(eight.clone());
    assert!(game
        .play_card("Bob".into(), eight.clone(), None, false)
        .is_ok());
    assert!(!game.players.get(1).unwrap().is_finished());
    assert_eq!(game.players.get(1).unwrap().get_card_count(), 3); // should receive penalty cards

    // said uno and should have
    game.players.get_mut(2).unwrap().give_card(eight.clone());
    game.players.get_mut(2).unwrap().give_card(eight.clone());
    assert!(game
        .play_card("Candace".into(), eight.clone(), None, true)
        .is_ok());
    assert!(!game.players.get(2).unwrap().is_finished());
    assert_eq!(game.players.get(2).unwrap().get_card_count(), 1); // should not receive penalty cards

    // said uno and should have
    game.players.get_mut(3).unwrap().give_card(eight.clone());
    game.players.get_mut(3).unwrap().give_card(eight.clone());
    game.players.get_mut(3).unwrap().give_card(eight.clone());
    assert!(game
        .play_card("Danny".into(), eight.clone(), None, true)
        .is_err());
    assert!(!game.players.get(3).unwrap().is_finished());
    assert_eq!(game.players.get(3).unwrap().get_card_count(), 3); // cards should not change
}

#[test]
fn test_skip() {
    use CardColor::*;
    use CardSymbol::*;

    let mut game = Game::new("Andy".into());
    game.add_player("Bob".into()).unwrap();
    game.add_player("Candace".into()).unwrap();

    // give some starting cards to players to not trigger endgame
    for player in game.players.iter_mut() {
        player.give_card(Card::new(Blue, Value(1)).unwrap());
        player.give_card(Card::new(Blue, Value(2)).unwrap());
        player.give_card(Card::new(Blue, Value(3)).unwrap());
    }

    assert_eq!(game.players.get(0).unwrap().get_card_count(), 3);
    assert_eq!(game.players.get(1).unwrap().get_card_count(), 3);

    let skip = Card::new(Blue, Skip).unwrap();
    game.deck.play(skip.clone());

    // give skips that will be used
    game.players.get_mut(0).unwrap().give_card(skip.clone());
    game.players.get_mut(1).unwrap().give_card(skip.clone());
    assert_eq!(game.players.get(0).unwrap().get_card_count(), 3 + 1);
    assert_eq!(game.players.get(1).unwrap().get_card_count(), 3 + 1);

    assert!(game
        .play_card("Andy".into(), skip.clone(), None, false)
        .is_ok());
    assert_eq!(game.players.get(0).unwrap().get_card_count(), 3); // playing actually happened
    assert!(game.active_cards.are_cards_active());
    assert_eq!(game.active_cards.active_symbol().unwrap(), Skip);
    assert_eq!(game.get_current_player().unwrap().name(), "Bob");

    assert!(game
        .play_card("Bob".into(), skip.clone(), None, false)
        .is_ok());
    assert!(game.active_cards.are_cards_active());
    assert_eq!(game.active_cards.active_symbol().unwrap(), Skip);
    assert_eq!(game.get_current_player().unwrap().name(), "Candace");

    assert!(game.draw_cards("Candace".into()).is_ok());
    assert_eq!(game.players.get(2).unwrap().get_card_count(), 3); // no drawing happened
    assert!(game.active_cards.active_symbol().is_none());
    assert!(!game.active_cards.are_cards_active());
    assert_eq!(game.get_current_player().unwrap().name(), "Andy"); // candice gets skipped
}

#[test]
fn test_game_end() {
    use CardColor::*;
    use CardSymbol::*;

    let mut game = Game::new("Andy".into());
    game.add_player("Bob".into()).unwrap();
    game.add_player("Candace".into()).unwrap();

    assert!(game.start().is_ok()); // simulate game start, set status to Running

    // give only 2 starting cards to players
    for player in game.players.iter_mut() {
        player.drop_all_cards();
        player.give_card(Card::new(Blue, Value(1)).unwrap());
        player.give_card(Card::new(Blue, Value(2)).unwrap());
    }

    game.deck.play(Card::new(Blue, Value(1)).unwrap());

    assert!(game.play_card(game.get_current_player().unwrap().name(), Card::new(Blue, Value(1)).unwrap(), None, true).is_ok());
    assert!(game.play_card(game.get_current_player().unwrap().name(), Card::new(Blue, Value(1)).unwrap(), None, true).is_ok());
    assert!(game.play_card(game.get_current_player().unwrap().name(), Card::new(Blue, Value(1)).unwrap(), None, true).is_ok());

    for player in game.players.iter() {
        assert_eq!(player.get_card_count(), 1usize);
    }

    assert_eq!(game.status, GameStatus::Running);
    assert!(game.play_card(game.get_current_player().unwrap().name(), Card::new(Blue, Value(2)).unwrap(), None, false).is_ok());
    assert_eq!(game.status, GameStatus::Running);
    assert!(game.play_card(game.get_current_player().unwrap().name(), Card::new(Blue, Value(2)).unwrap(), None, false).is_ok());
    assert_eq!(game.status, GameStatus::Finished);
}

#[test]
fn test_ai_does_not_hold() {
    use CardColor::*;
    use CardSymbol::*;

    let mut game = Game::new_with_ai("Andy".into(), 6);

    // simulate game start without the random order
    assert!(game.deal_starting_cards().is_ok());

    // let Andy play
    let skip = Card::new(Blue, Skip).unwrap();
    game.deck.play(skip.clone());
    game.players.get_mut(0).unwrap().give_card(skip.clone());

    let skip = Card::new(Blue, Skip).unwrap();
    // this will cause all the other AI players to play too
    assert!(game.play_card("Andy".into(), skip.clone(), None, false).is_ok());

    // we should be back at Andy
    assert!(game.get_current_player().unwrap().is_human());
    assert_eq!(game.get_current_player().unwrap().name(), "Andy".to_string());
}

#[test]
fn test_only_ai_finishes_game() {
    use CardColor::*;
    use CardSymbol::*;

    let mut game = Game::new_with_ai("Andy".into(), 6);

    // let Andy play his only one card
    let skip = Card::new(Blue, Skip).unwrap();
    game.deck.play(skip.clone());
    game.players.get_mut(0).unwrap().give_card(skip.clone());

    let skip = Card::new(Blue, Skip).unwrap();
    // this will cause all the other AI players to play too, but th
    assert!(game.play_card("Andy".into(), skip.clone(), None, false).is_ok());

    // all humans finished => game finished
    assert_eq!(game.status, GameStatus::Finished);
}
