use crate::components::card::{CardType, Color};
use crate::module::module::{
    DrawCard, DrawMeCard, Finish, GainedCards, LobbyStatus, Penalty, PlayCard, RunningStatus,
};
use crate::pages::game::{GameState, Player};
use crate::Game;

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
                Ok(x) => handle_finish_lobby(game, x),
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
    } else if msg.contains("\"type\":\"DRAW ME\"") {
        match serde_json::from_str::<DrawMeCard>(&msg) {
            Ok(x) => handle_draw_cards_me(game, x),
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
    } else if msg.contains("\"type\":\"GAINED CARD\"") {
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

pub fn handle_finish_lobby(game: &mut Game, new_data: LobbyStatus) {
    game.status = GameState::Finished;
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

    let mut players = new_data.players;
    let player_position = players.iter().position(|x| x.name.eq(&game.you)).unwrap();
    let mut right_side = players.split_off(player_position);
    right_side.remove(0);
    right_side.append(&mut players);
    game.players = right_side;

    game.finished_players = new_data.finished_players;
    game.cards = new_data.cards;
    game.discarted_card = new_data.top_card;
    game.clockwise = new_data.is_clockwise_direction;
}

pub fn handle_play_card(game: &mut Game, new_data: PlayCard) {
    let log_msg = format!("{}: {}", new_data.who, Action::PlayCard.logger_string());
    add_log(game, log_msg);
    match game.players.iter_mut().find(|x| x.name == new_data.who) {
        Some(player) => {
            player.cards -= 1;
        }
        None => (),
    };
    if new_data.card._type == CardType::Reverse {
        game.clockwise = !game.clockwise;
    }
    if new_data.who == game.you {
        let mut index = 0;
        if new_data.card._type == CardType::Draw4 || new_data.card._type == CardType::Wild {
            let mut reconstructed_card = new_data.card.clone();
            reconstructed_card.color = Color::Black;
            index = game
                .cards
                .iter()
                .position(|c| c == &reconstructed_card)
                .unwrap();
        } else {
            index = game.cards.iter().position(|c| c == &new_data.card).unwrap();
        }
        game.cards.remove(index);
    }
    game.current_player = Some(new_data.next);
    game.discarted_card = new_data.card;
}

pub fn handle_draw_cards_me(game: &mut Game, new_data: DrawMeCard) {
    new_data.cards.iter().for_each(|card| {
        game.cards.push(card.clone());
    });
    game.current_player = Some(new_data.next);
}

pub fn handle_draw_cards(game: &mut Game, new_data: DrawCard) {
    let log_msg = format!("{}: {}", new_data.who, Action::Draw.logger_string());
    add_log(game, log_msg);
    match game.players.iter_mut().find(|x| x.name == new_data.who) {
        Some(player) => {
            player.cards += new_data.cards;
        }
        None => (),
    };
    game.current_player = Some(new_data.next);
}

pub fn handle_finish(game: &mut Game, new_data: Finish) {
    let log_msg = format!("{}: {}", new_data.who, Action::Finish.logger_string());
    add_log(game, log_msg);
    game.finished_players.push(new_data.who);
}

pub fn handle_penalty(game: &mut Game, new_data: Penalty) {
    new_data.cards.iter().for_each(|card| {
        game.cards.push(card.clone());
    });
}

pub fn handle_gained_cards(game: &mut Game, new_data: GainedCards) {
    let log_msg = format!(
        "{}: {} {}x cards",
        new_data.who,
        Action::Gained.logger_string(),
        new_data.number
    );
    add_log(game, log_msg);
    match game.players.iter_mut().find(|x| x.name == new_data.who) {
        Some(player) => {
            player.cards += new_data.number;
        }
        None => (),
    };
}

pub enum Action {
    PlayCard,
    Draw,
    Finish,
    Gained,
}

impl Action {
    pub fn logger_string(&self) -> String {
        match self {
            Action::PlayCard => "played card".to_string(),
            Action::Draw => "drawn card".to_string(),
            Action::Finish => "finished!".to_string(),
            Action::Gained => "gained".to_string(),
        }
    }
}
pub fn add_log(game: &mut Game, log: String) {
    if game.logs.len() == 5 {
        game.logs.remove(0);
    }
    game.logs.push(log);
}
