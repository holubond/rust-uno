use crate::Game;
use crate::module::module::{DrawCard, Finish, GainedCards, LobbyStatus, Penalty, PlayCard, RunningStatus};
use crate::pages::game::{GameState, Player};

pub fn ws_msg_handler(game: &mut Game, msg: String) -> Result<(),String> {
    if msg.contains("\"type\":\"STATUS\"") {
        if msg.contains("\"status\":\"LOBBY\"") {
            let lobby = serde_json::from_str::<LobbyStatus>(&msg).unwrap();
            handle_lobby(game,lobby);
        } else if msg.contains("\"status\":\"RUNNING\"") {
            let running = serde_json::from_str::<RunningStatus>(&msg).unwrap();
            handle_running(game,running);
        } else if msg.contains("\"status\":\"FINISHED\"") {
            let finished = serde_json::from_str::<LobbyStatus>(&msg).unwrap();
            handle_lobby(game,finished);
        } else {
            return Err("Message from server has not valid struct".to_string());
        }
    } else if msg.contains("\"type\":\"PLAY CARD\"") {
        let play_card = serde_json::from_str::<PlayCard>(&msg).unwrap();
        handle_play_card(game,play_card);
    } else if msg.contains("\"type\":\"DRAW\"") {
        let draw_card = serde_json::from_str::<DrawCard>(&msg).unwrap();
        handle_draw_cards(game,draw_card);
    } else if msg.contains("\"type\":\"FINISH\"") {
        let finish = serde_json::from_str::<Finish>(&msg).unwrap();
        handle_finish(game,finish);
    } else if msg.contains("\"type\":\"PENALTY\"") {
        let penalty = serde_json::from_str::<Penalty>(&msg).unwrap();
        handle_penalty(game,penalty);
    } else if msg.contains("\"type\":\"GAINED CARDS\"") {
        let gained = serde_json::from_str::<GainedCards>(&msg).unwrap();
        handle_gained_cards(game,gained);
    } else {
        return Err("Message from server has not valid struct".to_string());
    }
    Ok(())
}


pub fn handle_lobby(game: &mut Game, new_data: LobbyStatus) {
    game.status = GameState::Lobby;
    game.author = lobby.author;
    game.you = lobby.you;
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
    game.current_player = Some(new_data.next);
    game.discarted_card = new_data.card;
}

pub fn handle_draw_cards(game: &mut Game, new_data: DrawCard) {
    game.players.iter().filter(|&mut x| x.name == new_data.who).for_each(|&mut mut x|x.cards += new_data.cards);
    game.current_player = Some(new_data.next);
}

pub fn handle_finish(game: &mut Game, new_data: Finish) {
    game.finished_players.push(new_data.who);
}

pub fn handle_penalty(game: &mut Game, new_data: Penalty) {
    new_data.cards.iter().for_each(|&card|{
       game.cards.push(card);
    });
}

pub fn handle_gained_cards(game: &mut Game, new_data: GainedCards) {
    game.players.iter().filter(|&mut x| x.name == new_data.who).for_each(|&mut mut x| x.cards += new_data.cards);
}
