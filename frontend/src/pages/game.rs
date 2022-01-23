use yew::prelude::*;
use yew::{function_component, html};
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::thread::current;
use gloo_console::log;
use reqwasm::websocket::{Message, futures::WebSocket};
use wasm_bindgen_futures::spawn_local;
use futures::{SinkExt, StreamExt};
use gloo_storage::Storage;
use crate::components::card::{Card, CardProps, CardType, Color};
use crate::pages::game::GameState::Lobby;
use crate::components::myuser::MyUser;
use crate::components::oponent::{Oponents, Oponent};

pub enum Msg {
    SubmitStart,
    SubmitSuccess,
    SubmitFailure,
}

pub struct Game {
    client: Arc<Client>,
    game: GameStore,
    status: GameState,
    author: String,
    you: String,
    cards: Vec<CardProps>,
    players: Vec<Player>,
    current_player: Option<String>,
    finished_players: Option<Vec<String>>,
    clockwise: bool,
    //todo discarted card

}
#[derive(Debug, Deserialize)]
struct GameStore {
    gameID: String,
    server: String,
    token: String,
}
enum GameState {
    Lobby,
    Running,
    Finished,
}
#[derive(PartialEq, Clone)]
pub struct Player {
    pub name: String,
    pub cards: u32,
}
#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub id: String,
}
impl Component for Game {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let game: GameStore = gloo_storage::LocalStorage::get("timestampPH").unwrap();
        let mut ws = WebSocket::open("wss://echo.websocket.org").unwrap();
        let (mut _write,mut read) = ws.split();
        spawn_local(async move {
            while let Some(msg) = read.next().await {
                log!(format!("1. {:?}", msg))
            }
            log!("WebSocket Closed")
        });
        //todo delete test data
        let test1= Player{
            name: "Kája".to_string(),
            cards: 8
        };
        let test2= Player{
            name: "Grolig".to_string(),
            cards: 5
        };
        let test3= Player{
            name: "Holy".to_string(),
            cards: 0
        };
        let test4= Player{
            name: "End".to_string(),
            cards: 4
        };
        let card1 = CardProps{
            color: Color::Blue,
            _type: CardType::Value,
            value: Some(1),
        };
        let card2 = CardProps{
            color: Color::Green,
            _type: CardType::Value,
            value: Some(3),
        };
        let card3 = CardProps{
            color: Color::Red,
            _type: CardType::Value,
            value: Some(3),
        };
        let card4 = CardProps{
            color: Color::Black,
            _type: CardType::Wild,
            value: None,
        };
        let card5 = CardProps{
            color: Color::Green,
            _type: CardType::Value,
            value: Some(3),
        };
        let card6 = CardProps{
            color: Color::Red,
            _type: CardType::Draw2,
            value: Some(3),
        };
        Self {
            client: Arc::new(Client::new()),
            game,
            status: Lobby,
            author: String::new(),
            you: "Were".to_string(),
            //you: String::new(),
            cards: vec![card1,card2,card3,card4,card5,card6],
            players: vec![test1, test2, test3, test4],
            current_player: Some("Holy".to_string()),
            finished_players: None,
            clockwise: true,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SubmitStart => {
                let client = self.client.clone();
                let id = self.game.gameID.clone();
                let token = self.game.token.clone();
                log!("Start game sending");
                _ctx.link().send_future(async {
                    match submit_start_game(client,id, token).await {
                        Ok(result) => Msg::SubmitSuccess,
                        _ => Msg::SubmitFailure,
                    }
                });
            }
            Msg::SubmitSuccess => {

            }
            Msg::SubmitFailure => {
                web_sys::window().unwrap().alert_with_message("Error occured during starting game.");
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        return html! {
            <main class="w-screen h-screen flex flex-col justify-center items-center bg-gray-300">
                <div class="w-screen flex flex-row justify-between">
                    <Oponents players={self.players.clone()} current={self.current_player.clone()}/>
                </div>
                <div class="w-screen h-48 flex justify-around">
                    <div class="rounded-lg bg-white shadow-md">
                        <img class="h-full w-full" src="../resources/draw_pile.png" alt="card"/>
                    </div>
                    <div class="-mt-16 -mb-16 opacity-10">
                        <img class="h-full w-full" src="../resources/rotate_arrow-L.png" alt="turn"/>
                    </div>
                    <div class="rounded-lg bg-black shadow-md">
                        <img class="h-full w-full" src="../resources/deck/r1.png" alt="top_of_deck_card"/>
                    </div>
                </div>
                <div class="w-screen flex flex-row justify-between">
                    <button class="bg-transparent hover:bg-red-500 text-red-700 font-semibold hover:text-white m-8 w-32 h-32 border border-red-500 hover:border-transparent rounded"
                    onclick={ctx.link().callback(|_| { Msg::SubmitStart })}>
                        {"Start game"}
                    </button>
                    <button class="bg-transparent hover:bg-red-500 text-red-700 font-semibold hover:text-white m-8 w-32 h-32 border border-red-500 hover:border-transparent rounded">
                        {"Uno"}
                    </button>
                    <MyUser name={self.you.clone()} current={self.current_player.clone()} cards={self.cards.clone()} />
                    <p></p>
                </div>
            </main>
        };
    }
}

async fn submit_start_game(client: Arc<Client>, game_id: String, token: String) -> Result<(), &'static str> {
    let url = format!("http://localhost:9000/game/{}/statusRunning",game_id);
    let response = client.post(url).bearer_auth(token).send().await;
    if response.is_err() {
        return Err("Error");
    }
    let response = response.unwrap();
    match response.status() {
        StatusCode::OK => {
            Ok(())
        },
        _ => return Err("Error")
    }
}

