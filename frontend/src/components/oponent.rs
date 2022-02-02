use crate::pages::game::Player;
use yew::html;
use yew::prelude::*;

pub struct Oponent;

#[derive(Clone, PartialEq, Properties)]
pub struct OponentProps {
    pub name: String,
    pub num: u32,
    pub current: bool,
    pub done: bool,
}

impl Component for Oponent {
    type Message = ();
    type Properties = OponentProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        if ctx.props().done {
            return html! {
                <div class="w-1/5 h-full flex flex-col rounded-lg bg-gray-500 shadow-md">
                    { render_opponent(&ctx.props().name, ctx.props().num) }
                </div>
            };
        }

        if ctx.props().current {
            return html! {
                <div class="w-1/5 h-full flex flex-col rounded-lg bg-yellow-300 shadow-md">
                    { render_opponent(&ctx.props().name, ctx.props().num) }
                </div>
            };
        }

        return html! {
            <div class="w-1/5 h-full flex flex-col rounded-lg bg-red-100 shadow-md">
                { render_opponent(&ctx.props().name, ctx.props().num) }
            </div>
        };
    }
}

fn render_opponent(name: &String, number_of_cards: u32) -> Html {
    html! {
        <>
            <div class="h-1/6">
                <p class="text-2xl text-center text-Black-500 font-bold">
                    { name }
                </p>
            </div>

            <div class="h-4/6">
                <img class="h-full w-full object-scale-down" src="../resources/card_face_down.png" alt="card"/>
            </div>

            <div class="h-1/6">
                <p class="text-xl text-center text-Black-500 font-bold">
                    { format!{"number of cards: {}", number_of_cards} }
                </p>
            </div>
        </>
    }
}

pub struct Oponents;

#[derive(Clone, PartialEq, Properties)]
pub struct OponentsProps {
    pub players: Vec<Player>,
    pub you: String,
    pub current: Option<String>,
    pub done: Vec<String>,
}

impl Component for Oponents {
    type Message = ();
    type Properties = OponentsProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();

        let current_player = match &ctx.props().current {
            None => panic!("OponentsProps.current is None"),
            Some(x) => x,
        };
        return html! {
            props.players.iter()
                .filter(|p| p.name != props.you)
                .map( |player| {
                    let mut done = false;
                    if props.done.contains(&player.name) {
                        done = true;
                    }
                    if &player.name == current_player {
                        html!{
                            <Oponent name={player.name.clone()} num={player.cards} current={true} done={done}/>
                        }
                    } else{
                        html!{
                            <Oponent name={player.name.clone()} num={player.cards} current={false} done={done}/>
                        }
                    }
                }).collect::<Html>()
        };
    }
}
