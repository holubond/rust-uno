use crate::Route;
use gloo_console::log;
use gloo_storage::Storage;
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreateResponse {
    gameID: String,
    server: String,
    token: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JoinResponse {
    server: String,
    token: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MessageResponse {
    message: String,
}

pub enum Msg {
    InputChanged,
    SubmitCreate,
    SubmitJoin,
    SubmitCreateSuccess(CreateResponse),
    SubmitJoinSuccess(JoinResponse),
    SubmitFailure(String),
}

pub struct Home {
    client: Arc<Client>,
    name_create: NodeRef,
    name_join: NodeRef,
    game_id: NodeRef,
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            client: Arc::new(Client::new()),
            name_join: NodeRef::default(),
            name_create: NodeRef::default(),
            game_id: NodeRef::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::InputChanged => {}
            Msg::SubmitCreate => {
                if let Some(input) = self.name_create.cast::<HtmlInputElement>() {
                    let name_create = input.value();
                    let client = self.client.clone();
                    ctx.link().send_future(async {
                        match send_create_game_request(client, name_create).await {
                            Ok(result) => Msg::SubmitCreateSuccess(result),
                            Err(err) => Msg::SubmitFailure(err),
                        }
                    });
                } else {
                    return false;
                }
            }
            Msg::SubmitJoin => {
                if let Some(name) = self.name_join.cast::<HtmlInputElement>() {
                    if let Some(game) = self.game_id.cast::<HtmlInputElement>() {
                        let name_join = name.value();
                        let game_id = game.value();
                        let client = self.client.clone();
                        ctx.link().send_future(async {
                            match send_join_game_request(client, name_join, game_id).await {
                                Ok(result) => Msg::SubmitJoinSuccess(result),
                                Err(err) => Msg::SubmitFailure(err),
                            }
                        });
                    }
                }
                return false;
            }

            Msg::SubmitCreateSuccess(result) => {
                let id = result.gameID.clone();
                local_storage_set("timestampPH", result);
                ctx.link().history().unwrap().push(Route::Lobby { id });
            }
            Msg::SubmitJoinSuccess(result) => {
                if let Some(game) = self.game_id.cast::<HtmlInputElement>() {
                    let game_id = game.value();
                    let game_data = CreateResponse {
                        gameID: game_id.clone(),
                        token: result.token,
                        server: result.server,
                    };

                    local_storage_set("timestampPH", game_data);
                    
                    ctx.link()
                        .history()
                        .unwrap()
                        .push(Route::Lobby { id: game_id });
                }
            }
            Msg::SubmitFailure(err_msg) => {
                match web_sys::window().unwrap().alert_with_message(&err_msg) {
                    Ok(_) => (),
                    _ => log!("Alert failed to pop up!"),
                };
                log!("Got Err response sending create");
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onchange = ctx.link().callback(|_| Msg::InputChanged);
        return html! {
            <main class="w-screen h-screen flex justify-center items-center bg-gray-300">
                <div class="flex flex-col text-center p-12 rounded-lg bg-white shadow-md">
                    <div class="flex justify-center">
                        <img class="h-40 w-1/2" src="resources/Logo.png" alt="Uno"/>
                    </div>
                    <div class="flex flex-col text-center p-12 rounded-lg">
                        <h1 class="font-bold text-2xl border-b-2 border-blue-100 py-2 my-3">{ "Let's play some Uno!" }</h1>
                    </div>
                    <div class="flex">
                        <div class="flex-1 text-center p-12 rounded-lg">
                            <form onsubmit={ctx.link().callback(|e: FocusEvent| { e.prevent_default(); Msg::SubmitCreate })} class="w-full max-w-sm">
                                <div class="md:flex md:items-center mb-6">
                                    <div class="md:w-1/3">
                                        <label class="block text-Black-500 font-bold md:text-right mb-1 md:mb-0 pr-4" for="inline-full-name">
                                            {"Username"}
                                        </label>
                                    </div>
                                <div class="md:w-2/3">
                                    <input class="bg-gray-200 appearance-none border-2 border-gray-200 rounded w-full py-2 px-4 text-gray-700 leading-tight focus:outline-none focus:bg-white focus:border-purple-500" type="text"
                                        id="name"
                                        ref={self.name_create.clone()}
                                        {onchange}
                                        placeholder="Username" />
                                    </div>
                                </div>
                                <div class="md:flex md:items-center">
                                    <div class="md:w-1/3"></div>
                                    <div class="md:w-2/3">
                                        <button class="shadow bg-red-600 hover:bg-red-800 focus:shadow-outline focus:outline-none text-white font-bold py-2 px-4 rounded" type="submit">
                                            {"Create game"}
                                        </button>
                                    </div>
                                </div>
                            </form>
                        </div>
                        <div class="flex-1 text-center p-12 rounded-lg">
                            <form onsubmit={ctx.link().callback(|e: FocusEvent| { e.prevent_default(); Msg::SubmitJoin })} class="w-full max-w-sm">
                                <div class="md:flex md:items-center mb-6">
                                    <div class="md:w-1/3">
                                        <label class="block text-Black-500 font-bold md:text-right mb-1 md:mb-0 pr-4" for="inline-full-name">
                                            {"Username"}
                                        </label>
                                    </div>
                                    <div class="md:w-2/3">
                                        <input class="bg-gray-200 appearance-none border-2 border-gray-200 rounded w-full py-2 px-4 text-gray-700 leading-tight focus:outline-none focus:bg-white focus:border-purple-500" type="text"
                                            id="name1"
                                            ref={self.name_join.clone()}
                                            placeholder="Username" />
                                    </div>
                                </div>
                                <div class="md:flex md:items-center mb-6">
                                    <div class="md:w-1/3">
                                            <label class="block text-Black-500 font-bold md:text-right mb-1 md:mb-0 pr-4" for="text">
                                            {"Game ID"}
                                            </label>
                                            <p class="text-red-500 text-xs italic">{"If joining Game."}</p>
                                    </div>
                                    <div class="md:w-2/3">
                                        <input class="bg-gray-200 appearance-none border-2 border-gray-200 rounded w-full py-2 px-4 text-gray-700 leading-tight focus:outline-none focus:bg-white focus:border-purple-500" type="text"
                                        id="gameId"
                                        ref={self.game_id.clone()}
                                        placeholder="Game ID" />
                                    </div>
                                </div>
                                <div class="md:flex md:items-center">
                                    <div class="md:w-1/3"></div>
                                    <div class="md:w-2/3">
                                        <button class="shadow bg-red-600 hover:bg-red-800 focus:shadow-outline focus:outline-none text-white font-bold py-2 px-4 rounded" type="submit">
                                            {"Join game"}
                                        </button>
                                    </div>
                                </div>
                            </form>
                        </div>
                    </div>
                </div>
            </main>
        };
    }
}

fn local_storage_set<T>(key: impl AsRef<str>, value: T)
where
    T: Serialize
{
    if gloo_storage::LocalStorage::set(key, value).is_ok() {
        return;
    }

    let window = match web_sys::window() {
        None => panic!("Failed attempt to call web_sys::window() in local_storage_set()"),
        Some(x) => x,
    };

    if window.alert_with_message("Local storage error").is_err() {
        log!("Alert failed to pop up!")
    }
}

async fn send_create_game_request(
    client: Arc<Client>,
    name: String,
) -> Result<CreateResponse, String> {
    let mut request_body = HashMap::new();
    request_body.insert("name", name);
    let response = client
        .post("http://localhost:9000/game")
        .json(&request_body)
        .send()
        .await;
    let response = match response {
        Ok(x) => x,
        _ => return Err("Internal communication error.".to_string()),
    };
    return match response.status() {
        StatusCode::CREATED => match response.json::<CreateResponse>().await {
            Ok(x) => Ok(x),
            _ => Err("Error: message from server had bad struct.".to_string()),
        },
        _ => Err("Undefined error occurred.".to_string()),
    };
}

async fn send_join_game_request(
    client: Arc<Client>,
    name: String,
    game_id: String,
) -> Result<JoinResponse, String> {
    let mut request_body = HashMap::new();
    request_body.insert("name", name);
    let url = format!("http://localhost:9000/game/{}/player", game_id);
    let response = client.post(url).json(&request_body).send().await;
    let response = match response {
        Ok(x) => x,
        _ => return Err("Internal communication error.".to_string()),
    };
    return match response.status() {
        StatusCode::CREATED => match response.json::<JoinResponse>().await {
            Ok(x) => Ok(x),
            _ => Err("Error: message from server had bad struct.".to_string()),
        },
        StatusCode::BAD_REQUEST | StatusCode::NOT_FOUND | StatusCode::GONE => {
            match response.json::<MessageResponse>().await {
                Ok(x) => Err(x.message.clone()),
                _ => Err("Error: message from server had bad struct.".to_string()),
            }
        }
        _ => Err("Undefined error occurred.".to_string()),
    };
}
