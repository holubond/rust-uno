use yew::prelude::*;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use gloo_console::log;

pub enum Msg {
    NameChanged(String),
    Submit,
    SubmitSuccess,
    SubmitFailure,
    ResetSubmitResult,
}

pub struct Game {
    client: Arc<Client>,
    name: String,
    game_id: String,
}

impl Component for Game {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            client: Arc::new(Client::new()),
            name: String::new(),
            game_id: String::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::NameChanged(data) => {
                self.name = data;
            }
            Msg::Submit => {
                log!("Start sending");
            }
            Msg::SubmitSuccess => {

            }
            Msg::SubmitFailure => {

            }
            Msg::ResetSubmitResult => {
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        return html! {
            <main class="w-screen h-screen flex justify-center items-center bg-gray-300	">
                <div class="flex flex-col text-center p-12 rounded-lg bg-white shadow-md">
                    <h1>{"HEJ"}</h1>
                </div>
            </main>
        };
    }
}
