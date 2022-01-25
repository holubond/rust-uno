use crate::components::card::{CardInfo, CardType, Color};
use crate::components::myuser::MyUser;
use crate::components::oponent::Oponents;
use crate::pages::game::GameState::Lobby;
use crate::sample_data;
use crate::util::alert::alert;
use futures::StreamExt;
use gloo_console::log;
use gloo_storage::Storage;
use reqwasm::websocket::futures::WebSocket;
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use wasm_bindgen_futures::spawn_local;
use yew::html;
use yew::prelude::*;

pub enum Msg {
    UnoChanged,
    SubmitStart,
    SubmitSuccess,
    SubmitFailure,
    PlayCard(CardInfo),
    DrawCard,
    DrawSuccess(DrawResponse),
}

pub struct Game {
    client: Arc<Client>,
    game: GameStore,
    status: GameState,
    author: String,
    you: String,
    cards: Vec<CardInfo>,
    players: Vec<Player>,
    current_player: Option<String>,
    finished_players: Option<Vec<String>>,
    clockwise: bool,
    uno_bool: bool,
    discarted_card: CardInfo, //todo discarted card
}

#[derive(Debug, Deserialize)]
struct GameStore {
    gameID: String,
    server: String,
    token: String,
}

#[derive(Eq, PartialEq)]
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

#[derive(Serialize, Deserialize, Clone)]
pub struct DrawResponse {
    cards: Vec<CardInfo>,
    next: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub id: String,
}

impl Component for Game {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        let game: GameStore = gloo_storage::LocalStorage::get("timestampPH").unwrap();
        let ws = WebSocket::open("wss://echo.websocket.org").unwrap();
        let (mut _write, mut read) = ws.split();
        spawn_local(async move {
            while let Some(msg) = read.next().await {
                log!(format!("1. {:?}", msg))
            }
            log!("WebSocket Closed")
        });

        Self {
            client: Arc::new(Client::new()),
            game,
            status: Lobby,
            //author: String::new(),
            author: "Were".to_string(),
            you: "Were".to_string(),
            //you: String::new(),
            cards: sample_data::cards(),
            players: sample_data::players(),
            current_player: Some("Holy".to_string()),
            finished_players: None,
            clockwise: true,
            uno_bool: false,
            discarted_card: CardInfo {
                color: Color::Red,
                _type: CardType::Value,
                value: Some(3),
            },
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UnoChanged => {
                self.uno_bool = !self.uno_bool;
                let card1 = CardInfo {
                    color: Color::Blue,
                    _type: CardType::Value,
                    value: Some(1),
                };
                self.cards.push(card1.clone());
                self.cards.push(card1.clone());
                self.cards.push(card1.clone());
                self.cards.push(card1.clone());
                self.cards.push(card1.clone());
                self.cards.push(card1.clone());
                self.cards.push(card1.clone());
                self.cards.push(card1.clone());
                self.cards.push(card1.clone());
                self.cards.push(card1.clone());

                return false;
            }

            Msg::SubmitStart => {
                let client = self.client.clone();
                let id = self.game.gameID.clone();
                let token = self.game.token.clone();
                log!("Start game sending");
                ctx.link().send_future(async {
                    match submit_start_game(client, id, token).await {
                        Ok(_) => Msg::SubmitSuccess,
                        _ => Msg::SubmitFailure,
                    }
                });
            }

            Msg::PlayCard(card) => {
                log!("PLAY CARD");
                // todo send ret api play card
                let client = self.client.clone();
                let id = self.game.gameID.clone();
                let token = self.game.token.clone();
                let said_uno = self.uno_bool.clone();
                log!("Start game sending");
                ctx.link().send_future(async move {
                    match play_card_request(client, id, token, card, None, said_uno.clone()).await {
                        Ok(_) => Msg::SubmitSuccess,
                        _ => Msg::SubmitFailure,
                    }
                });
            }

            Msg::DrawCard => {
                log!("DRAW CARD");
                let client = self.client.clone();
                let id = self.game.gameID.clone();
                let token = self.game.token.clone();
                log!("Start sending draw card");
                ctx.link().send_future(async {
                    match draw_card_request(client, id, token).await {
                        Ok(result) => Msg::DrawSuccess(result),
                        _ => Msg::SubmitFailure,
                    }
                });
            }

            Msg::DrawSuccess(response) => {
                response.cards.iter().for_each(|card| {
                    self.cards.push(card.clone());
                });
                self.current_player = Some(response.next);
            }

            Msg::SubmitSuccess => {
                //todo start game
            }

            Msg::SubmitFailure => {
                alert("Error occured during starting game.");
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let _props = ctx.props();
        let card_on_click = ctx.link().callback(|card: CardInfo| {
            log!("parent callback.");
            Msg::PlayCard(card)
        });
        let draw_pile_on_click = ctx.link().callback(|_: MouseEvent| Msg::DrawCard);
        /*
        // loby screen
        if self.status.eq(&GameState::Lobby) {
            return html!{
                <main class="w-screen h-screen flex flex-col justify-center items-center bg-gray-300">
                    <div class="flex flex-col rounded-lg bg-white shadow-md w-1/3 h-3/4">
                        <div class="h-1/2">
                            <p class="font-mono text-7xl font-bold text-center">{"Uno game lobby"}</p>
                            //todo start button
                            {
                                if self.author == self.you {
                                    html!{
                                        <button class="bg-transparent hover:bg-red-500 text-red-700 font-semibold hover:text-white m-8 w-16 h-16 border border-red-500 hover:border-transparent rounded"
                                            onclick={ctx.link().callback(|_| { Msg::SubmitStart })}>
                                            {"Start game"}
                                        </button>
                                    }
                                } else {
                                    html!{}
                                }
                            }
                        </div>
                        <div class="h-1/6">
                            <p class="text-xl font-bold text-center">{"Joined players:"}</p>
                            {
                                self.players.iter().map(|x|{
                                    html!{
                                        <p class="text-l font-bold text-center">{&x.name}</p>
                                    }
                                }).collect::<Html>()
                            }
                        </div>
                    </div>
                </main>
            }
        }
        */
        /*
        //todo finish screen
        if self.status.eq(&GameState::Finished) {
            return html!{}
        }*/
        return html! {
            <main class="w-screen h-screen flex flex-col justify-center items-center bg-gray-300">
                <div class="w-screen flex flex-row justify-between">
                    <Oponents players={self.players.clone()} you={self.you.clone()} current={self.current_player.clone()}/>
                </div>

                <div class="w-screen h-48 flex justify-around">
                    <div>
                        <img onclick={draw_pile_on_click} class="h-full w-full" src="../resources/draw_pile.png" alt="card"/>
                    </div>

                    <div class="-mt-16 -mb-16 opacity-10">
                        {
                            if self.clockwise {
                                html!{
                                    <img class="h-full w-full" src="../resources/rotate_arrow-R.png" alt="turn"/>
                                }
                            } else {
                                html!{
                                    <img class="h-full w-full" src="../resources/rotate_arrow-L.png" alt="turn"/>
                                }
                            }
                        }
                    </div>

                    <div class="rounded-lg w-32 bg-black shadow-md">
                        {
                            print_discarded_card(self.discarted_card.clone())
                        }
                    </div>
                </div>

                <div class="w-screen flex flex-row justify-between">
                    <div>
                        <input 
                            id="uno"
                            class="bg-gray-200 w-full py-2 px-4"
                            type="checkbox"
                            onchange={ctx.link().callback(|_| Msg::UnoChanged)}
                        />
                        
                        <label for="uno">{"UNO!"}</label>
                    </div>
                    
                    <MyUser 
                        username={self.you.clone()} 
                        current_username={self.current_player.clone()} 
                        cards={self.cards.clone()} 
                        card_on_click={card_on_click} 
                    />
                </div>
            </main>
        };
    }
}

fn print_discarded_card(card: CardInfo) -> Html {
    let use_color = card.color.to_str();
    let print_value = card.value_to_string();
    return html! {
        <div
            class="w-full h-full flex flex-col rounded-lg shadow-md"
            style={format!("background-color: {}", use_color)}
        >
            <div class="h-1/3">
                <p class="text-4xl text-left text-White-500 font-bold">
                    {format!("{}",print_value.clone())}
                </p>
            </div>
        
            <div class="h-1/3 flex justify-center">
                <p class="text-4xl text-center bg-gray-300 text-Black-500 font-bold">
                    {format!("{}",print_value.clone())}
                </p>
            </div>
        
            <div class="h-1/3">
                <p class="text-4xl text-right text-White-500 font-bold">
                    {format!{"{}",print_value.clone()}}
                </p>
            </div>
        </div>
    };
}

async fn submit_start_game(
    client: Arc<Client>,
    game_id: String,
    token: String,
) -> Result<(), &'static str> {
    let url = format!("http://localhost:9000/game/{}/statusRunning", game_id);
    let response = client.post(url).bearer_auth(token).send().await;
    let response = match response {
        Ok(x) => x,
        _ => return Err("Internal comunication error"),
    };
    match response.status() {
        StatusCode::NO_CONTENT => Ok(()),
        _ => return Err("Error"),
    }
}

async fn draw_card_request(
    client: Arc<Client>,
    game_id: String,
    token: String,
) -> Result<DrawResponse, &'static str> {
    let url = format!("http://localhost:9000/game/{}/drawnCards", game_id);
    let response = client.post(url).bearer_auth(token).send().await;
    let response = match response {
        Ok(x) => x,
        _ => return Err("Internal comunication error"),
    };
    match response.status() {
        StatusCode::OK => match response.json::<DrawResponse>().await {
            Ok(x) => return Ok(x),
            _ => return Err("Error: msg prom server has bad struct."),
        },
        _ => return Err("Error"),
    }
}

async fn play_card_request(
    client: Arc<Client>,
    game_id: String,
    token: String,
    card: CardInfo,
    new_color: Option<Color>,
    said_uno: bool,
) -> Result<(), &'static str> {
    let mut request_body = HashMap::new();
    request_body.insert("card", serde_json::to_string(&card).unwrap());
    if new_color.is_some() {
        request_body.insert("newColor", new_color.unwrap().to_str().to_uppercase());
    }
    request_body.insert("saidUno", said_uno.clone().to_string());
    let url = format!("http://localhost:9000/game/{}/playCard", game_id);
    let response = client
        .post(url)
        .json(&request_body)
        .bearer_auth(token)
        .send()
        .await;
    let response = match response {
        Ok(x) => x,
        _ => return Err("Internal comunication error"),
    };
    match response.status() {
        StatusCode::NO_CONTENT => Ok(()),
        _ => return Err("Error"),
    }
}
