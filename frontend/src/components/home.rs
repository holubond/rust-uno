use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;
use yew::prelude::*;
use yew::services::{ConsoleService, Task, TimeoutService};
use yewtil::future::LinkFuture;

pub enum Msg {
    NameChanged(InputData),
    GameIdChanged(InputData),
    Submit,
    SubmitSuccess,
    SubmitFailure,
    ResetSubmitResult,
}

pub struct Home {
    link: ComponentLink<Self>,
    client: Arc<Client>,
    name: String,
    game_id: String,
    timeout_job: Option<Box<dyn Task>>,
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            client: Arc::new(Client::new()),
            name: String::new(),
            game_id: String::new(),
            timeout_job: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::NameChanged(data) => {
                self.name = data.value;
            }
            Msg::GameIdChanged(data) => {
                self.game_id = data.value;
            }
            Msg::Submit => {
                ConsoleService::log("Start sending");
            }
            Msg::SubmitSuccess => {
                let handle = TimeoutService::spawn(
                    Duration::from_secs(3),
                    self.link.callback(|_| Msg::ResetSubmitResult),
                );
                self.timeout_job = Some(Box::new(handle));
            }
            Msg::SubmitFailure => {
                let handle = TimeoutService::spawn(
                    Duration::from_secs(3),
                    self.link.callback(|_| Msg::ResetSubmitResult),
                );
                self.timeout_job = Some(Box::new(handle));
            }
            Msg::ResetSubmitResult => {}
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        return html! {
            <main class="w-screen h-screen flex justify-center items-center bg-gray-300	">
                <div class="flex flex-col text-center p-12 rounded-lg bg-white shadow-md">
                    <div class="flex justify-center">
                        <img class="h-40 w-1/2" src="resources/Logo.png" alt="Uno"/>
                    </div>
                    <div class="flex flex-col text-center p-12 rounded-lg">
                        <h1 class="font-bold text-2xl border-b-2 border-blue-100 py-2 my-3">{ "Let's play some Uno!" }</h1>
                    </div>
                    <div class="flex">
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
                              <div class="md:flex md:items-center">
                                <div class="md:w-1/3"></div>
                                <div class="md:w-2/3">
                                  <button class="shadow bg-red-600 hover:bg-red-800 focus:shadow-outline focus:outline-none text-white font-bold py-2 px-4 rounded" type="button">
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
