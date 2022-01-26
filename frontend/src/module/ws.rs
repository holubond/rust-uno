use crate::module::module::{
    DrawCard, Finish, GainedCards, LobbyStatus, Penalty, PlayCard, RunningStatus,
};
use crate::pages::game::{GameState, Player};
use crate::Game;
use web_sys::alert;

pub fn ws_msg_handler(game: &mut Game, msg: String) -> Result<(), String> {
    if msg.contains("\"type\":\"STATUS\"") {
        if msg.contains("\"status\":\"LOBBY\"") {
            match serde_json::from_str::<LobbyStatus>(&msg) {
                Ok(x) => handle_lobby(game, x),
                Err(_) => (),
            };
        } else if msg.contains("\"status\":\"RUNNING\"") {
            match serde_json::from_str::<RunningStatus>(&msg) {
                Ok(x) => handle_running(game, x),
                Err(_) => (),
            };
        } else if msg.contains("\"status\":\"FINISHED\"") {
            match serde_json::from_str::<LobbyStatus>(&msg) {
                Ok(x) => handle_lobby(game, x),
                Err(_) => (),
            };
        } else {
            return Err("Message from server has not valid struct".to_string());
        }
    } else if msg.contains("\"type\":\"PLAY CARD\"") {
        match serde_json::from_str::<PlayCard>(&msg) {
            Ok(x) => handle_play_card(game, x),
            Err(_) => (),
        };
    } else if msg.contains("\"type\":\"DRAW\"") {
        match serde_json::from_str::<DrawCard>(&msg) {
            Ok(x) => handle_draw_cards(game, x),
            Err(_) => (),
        };
    } else if msg.contains("\"type\":\"FINISH\"") {
        match serde_json::from_str::<Finish>(&msg) {
            Ok(x) => handle_finish(game, x),
            Err(_) => (),
        };
    } else if msg.contains("\"type\":\"PENALTY\"") {
        match serde_json::from_str::<Penalty>(&msg) {
            Ok(x) => handle_penalty(game, x),
            Err(_) => (),
        };
    } else if msg.contains("\"type\":\"GAINED CARDS\"") {
        match serde_json::from_str::<GainedCards>(&msg) {
            Ok(x) => handle_gained_cards(game, x),
            Err(_) => (),
        };
    } else {
        return Err("Message from server has not valid struct".to_string());
    }
    Ok(())
}

pub fn handle_lobby(game: &mut Game, new_data: LobbyStatus) {
    game.status = GameState::Lobby;
    game.author = new_data.author;
    game.you = new_data.you;
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
    game.finished_players = new_data.finished_players;
    game.cards = new_data.cards;
    game.discarted_card = new_data.top_card;
    game.clockwise = new_data.is_clockwise_direction;
}

pub fn handle_play_card(game: &mut Game, new_data: PlayCard) {
    match game.players.iter_mut().find(|x| x.name == new_data.who) {
        Some(player) => {
            player.cards -= 1;
        }
        None => (),
    };
    game.current_player = Some(new_data.next);
    game.discarted_card = new_data.card;
}

pub fn handle_draw_cards(game: &mut Game, new_data: DrawCard) {
    match game.players.iter_mut().find(|x| x.name == new_data.who) {
        Some(player) => {
            player.cards += new_data.cards;
        }
        None => (),
    };
    game.current_player = Some(new_data.next);
}

pub fn handle_finish(game: &mut Game, new_data: Finish) {
    game.finished_players.push(new_data.who);
}

pub fn handle_penalty(game: &mut Game, new_data: Penalty) {
    new_data.cards.iter().for_each(|card| {
        game.cards.push(card.clone());
    });
}

pub fn handle_gained_cards(game: &mut Game, new_data: GainedCards) {
    match game.players.iter_mut().find(|x| x.name == new_data.who) {
        Some(player) => {
            player.cards += new_data.cards;
        }
        None => (),
    };
}
