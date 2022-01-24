use gloo_console::log;
use serde::{Deserialize, Serialize};
use std::fmt;
use yew::html::Scope;
use yew::prelude::*;
use yew::{function_component, html};

pub struct Card;
#[derive(Clone, PartialEq, Properties)]
pub struct CardProps {
    pub card_info: CardInfo,
    pub card_on_click: Callback<CardInfo>,
}
pub enum Msg {
    PlayCard,
    PlayWild(Color),
}
#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub enum Color {
    Red,
    Yellow,
    Green,
    Blue,
    Black,
}
impl Color {
    pub fn use_color(&self) -> String {
        match self {
            Color::Red => "red".to_string(),
            Color::Yellow => "yellow".to_string(),
            Color::Green => "green".to_string(),
            Color::Blue => "blue".to_string(),
            Color::Black => "black".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub enum CardType {
    Value,
    Skip,
    Reverse,
    Draw2,
    Draw4,
    Wild,
}
impl CardType {
    pub fn card_type_text(&self) -> String {
        match self {
            CardType::Skip => "Skip".to_string(),
            CardType::Reverse => "Reverse".to_string(),
            CardType::Draw2 => "+2".to_string(),
            CardType::Draw4 => "+4".to_string(),
            CardType::Wild => "Wild".to_string(),
            CardType::Value => "".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct CardInfo {
    pub color: Color,
    pub _type: CardType,
    pub value: Option<u32>,
}

impl Component for Card {
    type Message = Msg;
    type Properties = CardProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::PlayCard => {
                log! {"msg fired"};
                let props = ctx.props().clone();
                props.card_on_click.emit(props.card_info);
            }
            Msg::PlayWild(chosen_color) => {
                log! {"wild card clicked"};
                let mut props = ctx.props().clone();
                props.card_info.color = chosen_color;
                props.card_on_click.emit(props.card_info);
            }
        }
        true
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props().clone();
        if props.card_info._type.clone() == CardType::Wild {
            return html! {
                {print_wild_card(props.card_info.color.clone(),props.card_info._type.card_type_text().clone(),ctx.link().clone())}
            };
        }
        if props.card_info._type.clone() != CardType::Value {
            return html! {
                {print_card(props.card_info.color.clone(),props.card_info._type.card_type_text().clone(),ctx.link().clone())}
            };
        }
        return html! {
            {print_card(props.card_info.color.clone(),props.card_info.value.unwrap().to_string().clone(),ctx.link().clone())}
        };
    }
}
fn print_card(color: Color, value: String, link: Scope<Card>) -> Html {
    return html! {
        <div class="w-full h-full flex flex-col rounded-lg shadow-md"
        style={format!("background-color: {}", color.use_color().clone())}
        onclick={link.callback(|e: MouseEvent| { Msg::PlayCard })}
        >
                <div class="h-1/3">
                    <p class="text-6xl text-left text-White-500 font-bold">{format!("{}",value)}</p>
                </div>
                <div class="h-1/3 flex justify-center">
                    <p class="text-6xl text-center bg-gray-300 text-Black-500 font-bold">{format!("{}",value)}</p>
                </div>
                <div class="h-1/3">
                    <p class="text-6xl text-right text-White-500 font-bold">{format!{"{}",value}}</p>
                </div>
            </div>
    };
}
fn print_wild_card(color: Color, value: String, link: Scope<Card>) -> Html {
    let r = Color::Red.use_color();
    return html! {
        <div class="w-full h-full flex flex-col bg-black rounded-lg shadow-md">
            <div class="h-1/3 w-full flex flex-row rounded-lg">
                <div class="h-full w-1/2 rounded-lg" style="background-color: red" onclick={link.callback(|e: MouseEvent| { Msg::PlayWild(Color::Red) })}>
                </div>
                <div class="h-full w-1/2 rounded-lg" style="background-color: blue" onclick={link.callback(|e: MouseEvent| { Msg::PlayWild(Color::Blue) })}>
                </div>
            </div>
            <div class="h-1/3 flex justify-center">
                <p class="text-5xl text-center bg-gray-300 text-Black-500 font-bold">{format!("{}",value)}</p>
            </div>
            <div class="h-1/3 w-full flex flex-row rounded-lg">
                <div class="h-full w-1/2 rounded-lg" style="background-color: yellow" onclick={link.callback(|e: MouseEvent| { Msg::PlayWild(Color::Yellow) })}>
                </div>
                <div class="h-full w-1/2 rounded-lg" style="background-color: green" onclick={link.callback(|e: MouseEvent| { Msg::PlayWild(Color::Green) })}>
                </div>
            </div>
        </div>
    };
}
