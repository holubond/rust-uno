use crate::components::card::{Card, CardInfo};
use crate::module::module::PlayCardRequest;
use yew::html;
use yew::prelude::*;

pub struct MyUser;

#[derive(Clone, PartialEq, Properties)]
pub struct MyUserProps {
    pub username: String,
    pub current_username: Option<String>,
    pub cards: Vec<CardInfo>,
    pub card_on_click: Callback<PlayCardRequest>,
    pub done: Vec<String>,
}

impl Component for MyUser {
    type Message = ();
    type Properties = MyUserProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props().clone();
        let done = props.done.contains(&props.username);
        let current_username = match props.current_username {
            None => panic!("No value in MyUserProps.current_username"),
            Some(x) => x,
        };

        if done {
            return html! {
                <div class="flex flex-col w-full h-96 rounded-lg bg-gray-500 shadow-md justify-center">
                    {player_board(props.username, props.cards, props.card_on_click)}
                </div>
            };
        }
        if current_username == props.username {
            return html! {
                <div class="flex flex-col w-full h-96 rounded-lg bg-yellow-300 shadow-md justify-center">
                    {player_board(props.username, props.cards, props.card_on_click)}
                </div>
            };
        }
        return html! {
            <div class="flex flex-col w-full h-96 rounded-lg bg-white shadow-md justify-center">
                {player_board(props.username, props.cards, props.card_on_click)}
            </div>
        };
    }
}

fn player_board(
    username: String,
    cards: Vec<CardInfo>,
    card_on_click: Callback<PlayCardRequest>,
) -> Html {
    return html! {
        <>
            <div class="h-80 flex flex-row justify-around overflow-auto">
                {
                    cards.iter().map(|card| {
                        html!{
                            <Card card_info={card.clone()} card_on_click={card_on_click.clone()} />
                        }
                    }).collect::<Html>()
                }
            </div>
            <div>
                <p class="text-2xl text-center text-Black-500 font-bold">{username}</p>
            </div>
        </>
    };
}
