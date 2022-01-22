use std::collections::HashMap;
use yew::prelude::*;
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use web_sys::{Window, HtmlInputElement};
use yew_router::prelude::*;
use gloo_console::log;
use gloo_storage::{LocalStorage, Storage};
use crate::Route;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct create_response {
    gameID: String,
    server: String,
    token: String,
}

pub enum Msg {
    InputChanged,
    SubmitCreate,
    SubmitCreateSuccess(create_response),
    SubmitFailure,
}

pub struct Home {
    client: Arc<Client>,
    name: NodeRef,
    game_id: NodeRef,
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            client: Arc::new(Client::new()),
            name: NodeRef::default(),
            game_id: NodeRef::default(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::InputChanged => {
            }
            Msg::SubmitCreate => {
                let client = self.client.clone();
                if let Some(input) = self.name.cast::<HtmlInputElement>() {
                    let name = input.value();
                    _ctx.link().send_future(async {
                        match submit_create_form(client, name).await {
                            Ok(result) => Msg::SubmitCreateSuccess(result),
                            _ => Msg::SubmitFailure,
                        }
                    });
                } else {
                    return false;
                }
            }

            Msg::SubmitCreateSuccess(result) => {
                let id = result.gameID.clone();
                gloo_storage::LocalStorage::set("timestampPH",result);
                _ctx.link().history().unwrap().push(Route::Lobby {id});
            }
            Msg::SubmitFailure => {
                web_sys::window().unwrap().alert_with_message("Error occured during sending data");
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
                                    ref={self.name.clone()}
                                    {onchange}
                                    placeholder="Filter products" />
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
                            <form class="w-full max-w-sm">
                              <div class="md:flex md:items-center mb-6">
                                <div class="md:w-1/3">
                                  <label class="block text-Black-500 font-bold md:text-right mb-1 md:mb-0 pr-4" for="inline-full-name">
                                    {"Username"}
                                  </label>
                                </div>
                                <div class="md:w-2/3">
                                  <input class="bg-gray-200 appearance-none border-2 border-gray-200 rounded w-full py-2 px-4 text-gray-700 leading-tight focus:outline-none focus:bg-white focus:border-purple-500" type="text" />
                                </div>
                              </div>
                              <div class="md:flex md:items-center mb-6">
                                <div class="md:w-1/3">
                                  <label class="block text-Black-500 font-bold md:text-right mb-1 md:mb-0 pr-4" for="inline-password">
                                    {"Game ID"}
                                  </label>
                                  <p class="text-red-500 text-xs italic">{"If joining Game."}</p>
                                </div>
                                <div class="md:w-2/3">
                                  <input class="bg-gray-200 appearance-none border-2 border-gray-200 rounded w-full py-2 px-4 text-gray-700 leading-tight focus:outline-none focus:bg-white focus:border-purple-500" type="text"/>
                                </div>
                              </div>
                              <div class="md:flex md:items-center">
                                <div class="md:w-1/3"></div>
                                <div class="md:w-2/3">
                                  <button class="shadow bg-red-600 hover:bg-red-800 focus:shadow-outline focus:outline-none text-white font-bold py-2 px-4 rounded" type="button">
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
// async fn submit_form(client: Arc<Client>, name: String) -> reqwest::Result<Create_response> {
async fn submit_create_form(client: Arc<Client>, name: String) -> Result<create_response, &'static str> {
    let mut map = HashMap::new();
    map.insert("name",name);
    let response = client.post("http://localhost:9000/game").json(&map).send().await;
    if response.is_err() {
        return Err("Error");
    }
    let response = response.unwrap();
    match response.status() {
        StatusCode::CREATED => {
            match response.json::<create_response>().await {
                Ok(x) => return Ok(x),
                _ => return Err("Error")
            }
        },
        _ => return Err("Error")
    }
}
