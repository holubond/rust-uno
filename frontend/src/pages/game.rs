use yew::prelude::*;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;
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

pub struct Game {
    link: ComponentLink<Self>,
    client: Arc<Client>,
    name: String,
    game_id: String,
    timeout_job: Option<Box<dyn Task>>,
}

impl Component for Game {
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
            Msg::ResetSubmitResult => {
            }
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
                    <h1>{"HEJ"}</h1>
                </div>
            </main>
        };
    }
}
