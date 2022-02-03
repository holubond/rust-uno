use crate::components::card::{CardType, Color};
use crate::module::modul::{
    DrawCard, DrawMeCard, Finish, GainedCards, LobbyStatus, Penalty, PlayCard, RunningStatus,
};
use crate::pages::game::{GameState, Player};
use crate::Game;

pub fn ws_msg_handler(game: &mut Game, msg: String) -> Result<(), String> {
    if msg.contains("\"type\":\"STATUS\"") {
        if msg.contains("\"status\":\"LOBBY\"") {
            if let Ok(x) = serde_json::from_str::<LobbyStatus>(&msg) { handle_lobby(game, x) };
        } else if msg.contains("\"status\":\"RUNNING\"") {
            if let Ok(x) = serde_json::from_str::<RunningStatus>(&msg) { handle_running(game, x) };
        } else if msg.contains("\"status\":\"FINISHED\"") {
            if let Ok(x) = serde_json::from_str::<LobbyStatus>(&msg) { handle_finish_lobby(game, x) };
        } else {
            return Err("Message from server has not valid struct".to_string());
        }
    } else if msg.contains("\"type\":\"PLAY CARD\"") {
        if let Ok(x) = serde_json::from_str::<PlayCard>(&msg) { handle_play_card(game, x) };
    } else if msg.contains("\"type\":\"DRAW ME\"") {
        if let Ok(x) = serde_json::from_str::<DrawMeCard>(&msg) { handle_draw_cards_me(game, x) };
    } else if msg.contains("\"type\":\"DRAW\"") {
        if let Ok(x) = serde_json::from_str::<DrawCard>(&msg) { handle_draw_cards(game, x) };
    } else if msg.contains("\"type\":\"FINISH\"") {
        if let Ok(x) = serde_json::from_str::<Finish>(&msg) { handle_finish(game, x) };
    } else if msg.contains("\"type\":\"PENALTY\"") {
        if let Ok(x) = serde_json::from_str::<Penalty>(&msg) { handle_penalty(game, x) };
    } else if msg.contains("\"type\":\"GAINED CARD\"") {
        if let Ok(x) = serde_json::from_str::<GainedCards>(&msg) { handle_gained_cards(game, x) };
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
    let card_info = match new_data.card._type {
        CardType::Value => format!(
            "{} {}",
            new_data.card.color.to_str(),
            new_data.card.value.unwrap()
        ),
        CardType::Reverse | CardType::Draw2 | CardType::Skip => format!(
            "{} {}",
            new_data.card.color.to_str(),
            new_data.card._type.card_type_text()
        ),
        CardType::Wild | CardType::Draw4 => format!(
            "{} {}",
            new_data.card.color.to_str(),
            new_data.card._type.card_type_text()
        ),
    };
    let log_msg = format!(
        "{}: {} {}",
        new_data.who,
        Action::PlayCard.logger_string(),
        card_info
    );
    add_log(game, log_msg);
    if let Some(player) = game.players.iter_mut().find(|x| x.name == new_data.who) {
        player.cards -= 1;
    };
    if new_data.card._type == CardType::Reverse {
        game.clockwise = !game.clockwise;
    }
    if new_data.who == game.you {
        if new_data.card._type == CardType::Draw4 || new_data.card._type == CardType::Wild {
            let mut reconstructed_card = new_data.card.clone();
            reconstructed_card.color = Color::Black;
            let index = game
                .cards
                .iter()
                .position(|c| c == &reconstructed_card)
                .unwrap();
            game.cards.remove(index);
        } else {
            let index = game.cards.iter().position(|c| c == &new_data.card).unwrap();
            game.cards.remove(index);
        }
    }
    game.current_player = Some(new_data.next);
    game.discarted_card = new_data.card;
}

pub fn handle_draw_cards_me(game: &mut Game, new_data: DrawMeCard) {
    let action = match new_data.cards.len() {
        0 => "was skipped".to_string(),
        x => format!("drawn {} card(s)", x),
    };

    let log_msg = format!(
        "{}: {}",
        game.you,
        action
    );
    add_log(game, log_msg);
    new_data.cards.iter().for_each(|card| {
        game.cards.push(card.clone());
    });
    game.current_player = Some(new_data.next);
}

pub fn handle_draw_cards(game: &mut Game, new_data: DrawCard) {
    let action = match new_data.cards {
        0 => "was skipped".to_string(),
        x => format!("drawn {} card(s)", x),
    };

    let log_msg = format!(
        "{}: {}",
        new_data.who,
        action
    );
    add_log(game, log_msg);
    if let Some(player) = game.players.iter_mut().find(|x| x.name == new_data.who) {
        player.cards += new_data.cards;
    };
    game.current_player = Some(new_data.next);
}

pub fn handle_finish(game: &mut Game, new_data: Finish) {
    let log_msg = format!("{}: {}", new_data.who, Action::Finish.logger_string());
    add_log(game, log_msg);
    game.finished_players.push(new_data.who);
}

pub fn handle_penalty(game: &mut Game, new_data: Penalty) {
    let log_msg = format!(
        "{}: forgot to say UNO (gained {} card(s))",
        game.you,
        new_data.cards.len()
    );
    add_log(game, log_msg);

    new_data.cards.iter().for_each(|card| {
        game.cards.push(card.clone());
    });
}

pub fn handle_gained_cards(game: &mut Game, new_data: GainedCards) {
    let log_msg = format!(
        "{}: forgot to say UNO (gained {} card(s))",
        new_data.who,
        new_data.number
    );
    add_log(game, log_msg);
    if let Some(player) = game.players.iter_mut().find(|x| x.name == new_data.who) {
        player.cards += new_data.number;
    };
}

pub enum Action {
    PlayCard,
    Finish,
}

impl Action {
    pub fn logger_string(&self) -> String {
        match self {
            Action::PlayCard => "played".to_string(),
            Action::Finish => "finished!".to_string(),
        }
    }
}
pub fn add_log(game: &mut Game, log: String) {
    game.logs.push(log);
}
