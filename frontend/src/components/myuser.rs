use yew::prelude::*;
use yew::{function_component, html};
use crate::components::card::{Card, CardProps};

pub struct MyUser;
#[derive(Clone, PartialEq, Properties)]
pub struct MyUserProps {
    pub name: String,
    pub current: Option<String>,
    pub cards: Vec<CardProps>,
}
impl Component for MyUser {
    type Message = ();
    type Properties = MyUserProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let cur = ctx.props().current.clone().unwrap();
        if cur==ctx.props().name.clone() {
            return html!{
                <div class="flex flex-col w-80 h-96 rounded-lg bg-yellow-300 shadow-md justify-center">
                    <div class="h-80 w-72">
                        {
                            ctx.props().cards.iter().map(|card| {
                                html!{
                                    <Card color={card.color.clone()} _type={card._type.clone()} value={card.value.clone()} />
                                }
                            }).collect::<Html>()
                        }
                    </div>
                    <div>
                        <p class="text-2xl text-center text-Black-500 font-bold">{format!("{}",ctx.props().name)}</p>
                    </div>
                </div>
            };
        }
        return html!{
            <div class="flex flex-col w-2/3 h-96 rounded-lg bg-white shadow-md justify-center">
                <div class="h-80 w-full flex flex-row">
                        {
                            ctx.props().cards.iter().map(|card| {
                                html!{
                                    <Card color={card.color.clone()} _type={card._type.clone()} value={card.value.clone()} />
                                }
                            }).collect::<Html>()
                        }
                    </div>
                <div>
                    <p class="text-2xl text-center text-Black-500 font-bold">{format!("{}",ctx.props().name)}</p>
                </div>
            </div>
        };
    }
}
