use crate::components::card::{Card, CardInfo};
use yew::html;
use yew::prelude::*;

pub struct MyUser;

#[derive(Clone, PartialEq, Properties)]
pub struct MyUserProps {
    pub username: String,
    pub current_username: Option<String>,
    pub cards: Vec<CardInfo>,
    pub card_on_click: Callback<CardInfo>,
}

impl Component for MyUser {
    type Message = ();
    type Properties = MyUserProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props().clone();

        let current_username = match props.current_username.clone() {
            None => panic!("No value in MyUserProps.current_username"),
            Some(x) => x,
        };

        if current_username == props.username.clone() {
            return html! {
                <div class="flex flex-col w-80 h-96 rounded-lg bg-yellow-300 shadow-md justify-center">
                    {player_board(props.username.clone(), props.cards.clone(), props.card_on_click.clone())}
                </div>
            };
        }
        return html! {
            <div class="flex flex-col w-2/3 h-96 rounded-lg bg-white shadow-md justify-center">
                {player_board(props.username.clone(), props.cards.clone(), props.card_on_click.clone())}
            </div>
        };
    }
}

fn player_board(username: String, cards: Vec<CardInfo>, card_on_click: Callback<CardInfo>) -> Html {
    /*
    let card_on_click = move |card:CardInfo|{
        card_on_click.emit(card.clone());
    };*/
    return html! {
        <>
            <div class="h-80 flex flex-row overflow-auto">
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
