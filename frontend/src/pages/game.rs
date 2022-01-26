use crate::components::card::{CardInfo, CardType, Color};
use crate::components::myuser::MyUser;
use crate::components::oponent::Oponents;
use crate::module::module::{
    CardConflictMessageResponse, LobbyStatus, MessageResponse, PlayCardRequest,
};
use crate::sample_data::test_session;
use crate::url::game_ws;
use crate::util::alert::alert;
use crate::{sample_data, url};
use futures::StreamExt;
use gloo_console::log;
use gloo_storage::Storage;
use reqwasm::websocket::futures::WebSocket;
use reqwasm::websocket::{Message, State};
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use wasm_bindgen_futures::spawn_local;
use yew::html;
use yew::prelude::*;

pub enum Msg {
    UnoChanged,
    SubmitStart,
    SubmitSuccess,
    SubmitFailure(String),
    PlayCard(PlayCardRequest),
    DrawCard,
    DrawSuccess(DrawResponse),
    UpdateStatus(Message),
}

pub struct Game {
    pub(crate) client: Arc<Client>,
    pub(crate) game: GameStore,
    pub(crate) status: GameState,
    pub(crate) author: String,
    pub(crate) you: String,
    pub(crate) cards: Vec<CardInfo>,
    pub(crate) players: Vec<Player>,
    pub(crate) current_player: Option<String>,
    pub(crate) finished_players: Option<Vec<String>>,
    pub(crate) clockwise: bool,
    pub(crate) uno_bool: bool,
    pub(crate) discarted_card: CardInfo, //todo discarted card
}

#[derive(Debug, Deserialize)]
pub struct GameStore {
    #[serde(rename = "gameID")]
    game_id: String,
    server: String,
    token: String,
}

#[derive(Eq, PartialEq)]
pub enum GameState {
    Lobby,
    Running,
    Finished,
    Loading,
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
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

    fn create(ctx: &Context<Self>) -> Self {
        let game: GameStore = gloo_storage::LocalStorage::get("lastGame").unwrap();
        let link = ctx.link().clone();
        let mut ws = WebSocket::open(&game_ws(&game.token.clone())).unwrap();
        let (mut write, mut read) = ws.split();
        let default_data = Self {
            client: Arc::new(Client::new()),
            game,
            status: GameState::Loading,
            author: String::new(),
            you: String::new(),
            cards: vec![],
            players: vec![],
            current_player: None,
            finished_players: None,
            clockwise: true,
            uno_bool: false,
            discarted_card: CardInfo {
                color: Color::Red,
                _type: CardType::Value,
                value: Some(1),
            },
        };
        spawn_local(async move {
            while let Some(msg) = read.next().await {
                match msg {
                    Ok(x) => {
                        link.send_message(Msg::UpdateStatus(x.clone()));
                        log!(format!("got msg in ws: {:?}", x))
                    }
                    Err(_) => (),
                }
            }
            log!("WebSocket Closed")
        });

        //test purposes data
        //test_session(game,)

        return default_data;
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UnoChanged => {
                self.uno_bool = !self.uno_bool;

                return true;
            }

            Msg::SubmitStart => {
                let client = self.client.clone();
                let id = self.game.game_id.clone();
                let token = self.game.token.clone();
                log!("Start game sending");
                ctx.link().send_future(async {
                    match submit_start_game(client, id, token).await {
                        Ok(_) => Msg::SubmitSuccess,
                        Err(err) => Msg::SubmitFailure(err),
                    }
                });
            }

            Msg::PlayCard(card) => {
                log!("PLAY CARD");
                let client = self.client.clone();
                let id = self.game.game_id.clone();
                let token = self.game.token.clone();
                let said_uno = self.uno_bool.clone();
                log!("Start game sending");
                ctx.link().send_future(async move {
                    match play_card_request(client, id, token, card, said_uno.clone()).await {
                        Ok(_) => Msg::SubmitSuccess,
                        Err(err) => Msg::SubmitFailure(err),
                    }
                });
            }

            Msg::DrawCard => {
                log!("DRAW CARD");
                let client = self.client.clone();
                let id = self.game.game_id.clone();
                let token = self.game.token.clone();
                log!("Start sending draw card");
                ctx.link().send_future(async {
                    match draw_card_request(client, id, token).await {
                        Ok(result) => Msg::DrawSuccess(result),
                        Err(err) => Msg::SubmitFailure(err),
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
                //todo render changes
            }

            Msg::SubmitFailure(err_msg) => {
                alert(&err_msg);
                log!("Got Err response sending create");
            }

            Msg::UpdateStatus(msg) => {
                match msg {
                    Message::Text(text) => {
                        if text.contains("\"type\":\"STATUS\"") {
                            log!("contains status");
                            let lobby = serde_json::from_str::<LobbyStatus>(&text).unwrap();
                            self.status = GameState::Lobby;
                            self.author = lobby.author;
                            self.you = lobby.you;
                            self.players = vec![];
                            lobby.players.iter().for_each(|p| {
                                self.players.push(Player {
                                    name: p.to_string(),
                                    cards: 0,
                                })
                            });
                        }
                    }
                    Message::Bytes(bytes) => (),
                }
                log!("Updating status msg");
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if self.status.eq(&GameState::Loading) {
            return html! {
                <main class="w-screen h-screen flex flex-col justify-center items-center bg-gray-300">
                    <div>
                        <p>{"Waiting for server .... loading data ..."}</p>
                    </div>
                </main>
            };
        }
        let _props = ctx.props();
        let card_on_click = ctx.link().callback(|card: PlayCardRequest| {
            log!("parent callback.");
            Msg::PlayCard(card)
        });
        let draw_pile_on_click = ctx.link().callback(|_: MouseEvent| Msg::DrawCard);

        // loby screen
        if self.status.eq(&GameState::Lobby) {
            return html! {
                <main class="w-screen h-screen flex flex-col justify-center items-center bg-gray-300">
                    <div class="flex flex-col rounded-lg bg-white shadow-md w-1/3 h-3/4">
                        <div class="h-1/2">
                            <p class="font-mono text-7xl font-bold text-center">{"Uno game lobby"}</p>
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
                        <div class="h-1/2">
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
            };
        }

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

                    <div class="opacity-10">
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
                            checked={self.uno_bool.clone()}
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
            style={ format!("background-color: {}", use_color) }
        >
            <div class="h-1/3">
                <p class="text-4xl text-left text-White-500 font-bold">
                    { format!("{}", print_value) }
                </p>
            </div>

            <div class="h-1/3 flex justify-center">
                <p class="text-4xl text-center bg-gray-300 text-Black-500 font-bold">
                    { format!("{}", print_value) }
                </p>
            </div>

            <div class="h-1/3">
                <p class="text-4xl text-right text-White-500 font-bold">
                    { format!{"{}", print_value} }
                </p>
            </div>
        </div>
    };
}

async fn submit_start_game(
    client: Arc<Client>,
    game_id: String,
    token: String,
) -> Result<(), String> {
    let url = url::status_running(game_id);
    let response = client.post(url).bearer_auth(token).send().await;
    let response = match response {
        Ok(x) => x,
        _ => return Err("Server is not responding.".to_string()),
    };
    return match response.status() {
        StatusCode::NO_CONTENT => Ok(()),
        StatusCode::UNAUTHORIZED
        | StatusCode::FORBIDDEN
        | StatusCode::NOT_FOUND
        | StatusCode::CONFLICT => match response.json::<MessageResponse>().await {
            Ok(x) => Err(x.message.clone()),
            _ => Err("Error: message from server had bad struct.".to_string()),
        },
        _ => Err("Undefined error occurred.".to_string()),
    };
}

async fn draw_card_request(
    client: Arc<Client>,
    game_id: String,
    token: String,
) -> Result<DrawResponse, String> {
    let url = url::drawn_cards(game_id);
    let response = client.post(url).bearer_auth(token).send().await;
    let response = match response {
        Ok(x) => x,
        _ => return Err("Server is not responding.".to_string()),
    };

    return match response.status() {
        StatusCode::OK => match response.json::<DrawResponse>().await {
            Ok(x) => return Ok(x),
            _ => Err("Error: message from server had bad struct.".to_string()),
        },
        StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN | StatusCode::NOT_FOUND => {
            match response.json::<MessageResponse>().await {
                Ok(x) => Err(x.message.clone()),
                _ => Err("Error: message from server had bad struct.".to_string()),
            }
        }
        StatusCode::CONFLICT => match response.json::<CardConflictMessageResponse>().await {
            Ok(x) => Err(x.message.clone()),
            _ => Err("Error: message from server had bad struct.".to_string()),
        },
        _ => Err("Undefined error occurred.".to_string()),
    };
}

async fn play_card_request(
    client: Arc<Client>,
    game_id: String,
    token: String,
    card: PlayCardRequest,
    said_uno: bool,
) -> Result<(), String> {
    let mut request_body = HashMap::new();
    request_body.insert("card", serde_json::to_string(&card.card).unwrap());
    if card.new_color.is_some() {
        request_body.insert("newColor", card.new_color.unwrap().to_uppercase());
    }
    request_body.insert("saidUno", said_uno.clone().to_string());
    let url = url::play_card(game_id);
    let response = client
        .post(url)
        .json(&request_body)
        .bearer_auth(token)
        .send()
        .await;
    let response = match response {
        Ok(x) => x,
        _ => return Err("Server is not responding.".to_string()),
    };
    return match response.status() {
        StatusCode::NO_CONTENT => Ok(()),
        StatusCode::BAD_REQUEST
        | StatusCode::UNAUTHORIZED
        | StatusCode::FORBIDDEN
        | StatusCode::NOT_FOUND => match response.json::<MessageResponse>().await {
            Ok(x) => Err(x.message.clone()),
            _ => Err("Error: message from server had bad struct.".to_string()),
        },
        StatusCode::CONFLICT => match response.json::<CardConflictMessageResponse>().await {
            Ok(x) => Err(x.message.clone()),
            _ => Err("Error: message from server had bad struct.".to_string()),
        },
        _ => Err("Undefined error occurred.".to_string()),
    };
}
