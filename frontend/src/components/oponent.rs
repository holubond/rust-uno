use yew::prelude::*;
use yew::{function_component, html};
use crate::pages::game::Player;

pub struct Oponent;
#[derive(Clone, PartialEq, Properties)]
pub struct OponentProps {
    pub name: String,
    pub num: u32,
    pub current: bool,
}
impl Component for Oponent {
    type Message = ();
    type Properties = OponentProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        if ctx.props().current {
            return html!{
                <div class="w-1/5 h-full flex flex-col rounded-lg bg-yellow-300 shadow-md">
                    <div>
                        <p class="text-2xl text-center text-Black-500 font-bold">{format!("{}",ctx.props().name)}</p>
                    </div>
                    <div>
                        <img class="h-full w-2/3" src="../resources/card_face_down.png" alt="card"/>
                    </div>
                    <div>
                        <p class="text-xl text-center text-Black-500 font-bold">{format!{"number of cards: {}",ctx.props().num}}</p>
                    </div>
                </div>
            };
        }
        return html!{
            <div class="w-1/5 h-full flex flex-col rounded-lg bg-red-100 shadow-md">
                <div>
                    <p class="text-2xl text-center text-Black-500 font-bold">{format!("{}",ctx.props().name)}</p>
                </div>
                <div>
                    <img class="h-full w-2/3" src="../resources/card_face_down.png" alt="card"/>
                </div>
                <div>
                    <p class="text-xl text-center text-Black-500 font-bold">{format!{"number of cards: {}",ctx.props().num}}</p>
                </div>
            </div>
        };
    }
}
pub struct Oponents;
#[derive(Clone, PartialEq, Properties)]
pub struct OponentsProps {
    pub players: Vec<Player>,
    pub current: Option<String>,
}
impl Component for Oponents {
    type Message = ();
    type Properties = OponentsProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        if ctx.props().current.is_none() {
            return html! {
                ctx.props().players.iter().map(|player| {
                    html!{
                        <Oponent name ={player.name.clone()} num ={player.cards} current={false} />
                    }
                }).collect::<Html>()
            }
        }
        return html!{
            ctx.props().players.iter().map(|player| {
                if player.name == ctx.props().current.clone().unwrap() {
                    html!{
                        <Oponent name ={player.name.clone()} num ={player.cards} current={true} />
                    }
                } else{
                    html!{
                        <Oponent name ={player.name.clone()} num ={player.cards} current={false} />
                    }
                }
            }).collect::<Html>()
        };
    }
}
