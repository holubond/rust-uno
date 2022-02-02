use crate::module::module::PlayCardRequest;
use gloo_console::log;
use serde::{Deserialize, Serialize};
use yew::html;
use yew::html::Scope;
use yew::prelude::*;

pub struct Card;

#[derive(Clone, PartialEq, Properties)]
pub struct CardProps {
    pub card_info: CardInfo,
    pub card_on_click: Callback<PlayCardRequest>,
}

pub enum Msg {
    PlayCard,
    PlayWild(Color),
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all(serialize = "UPPERCASE", deserialize = "UPPERCASE"))]
pub enum Color {
    Red,
    Yellow,
    Green,
    Blue,
    Black,
}

impl Color {
    pub fn to_str(&self) -> &str {
        match self {
            Color::Red => "red",
            Color::Yellow => "yellow",
            Color::Green => "green",
            Color::Blue => "blue",
            Color::Black => "black",
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all(serialize = "UPPERCASE", deserialize = "UPPERCASE"))]
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
            CardType::Reverse => "Rev".to_string(),
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
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub _type: CardType,
    pub value: Option<u32>,
}

impl CardInfo {
    pub fn value_to_string(&self) -> String {
        if self._type != CardType::Value {
            return self._type.card_type_text();
        }

        match self.value {
            None => panic!("Attempting to access value of a card, that does not have any"),
            Some(v) => v.to_string(),
        }
    }
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
                props.card_on_click.emit(PlayCardRequest {
                    card: props.card_info,
                    new_color: None,
                    said_uno: false,
                });
            }
            Msg::PlayWild(chosen_color) => {
                log! {"colored card clicked"};
                let props = ctx.props().clone();
                props.card_on_click.emit(PlayCardRequest {
                    card: props.card_info,
                    new_color: Some(chosen_color.to_str().to_string()),
                    said_uno: false,
                });
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props().clone();

        if props.card_info._type == CardType::Wild || props.card_info._type == CardType::Draw4 {
            return print_colorful_card(props.card_info.value_to_string(), ctx.link().clone());
        }

        print_card(
            &props.card_info.color,
            props.card_info.value_to_string(),
            ctx.link().clone(),
        )
    }
}

fn print_card(color: &Color, value: String, link: Scope<Card>) -> Html {
    return html! {
        <div class="w-40 h-full cursor-pointer flex flex-col rounded-lg shadow-md border-black border-4"
            style={format!("background-color: {}", color.to_str())}
            onclick={link.callback(|_: MouseEvent| { Msg::PlayCard })}
        >
            <div class="h-1/3 w-full">
                <p class="text-6xl text-left text-White-500 font-bold">{value.to_string()}</p>
            </div>

            <div class="h-1/3 w-full flex justify-center">
                <p class="text-6xl text-center bg-gray-300 text-Black-500 font-bold">{value.to_string()}</p>
            </div>

            <div class="h-1/3 w-full">
                <p class="text-6xl text-right text-White-500 font-bold">{value.to_string()}</p>
            </div>
        </div>
    };
}

fn print_colorful_card(value: String, link: Scope<Card>) -> Html {
    return html! {
        <div class="w-40 h-full cursor-pointer flex flex-col bg-black rounded-lg shadow-md border-black border-4">
            <div class="h-1/3 w-full flex flex-row rounded-lg">
                <div
                    class="h-full w-1/2 rounded-lg" style="background-color: red"
                    onclick={link.callback(|_: MouseEvent| { Msg::PlayWild(Color::Red) })}
                >
                </div>

                <div
                    class="h-full w-1/2 rounded-lg" style="background-color: blue"
                    onclick={link.callback(|_: MouseEvent| { Msg::PlayWild(Color::Blue) })}
                >
                </div>
            </div>

            <div class="h-1/3 w-full flex justify-center">
                <p class="text-5xl text-center bg-gray-300 text-Black-500 font-bold">
                    {value.to_string()}
                </p>
            </div>

            <div class="h-1/3 w-full flex flex-row rounded-lg">
                <div
                    class="h-full w-1/2 rounded-lg" style="background-color: yellow"
                    onclick={link.callback(|_: MouseEvent| { Msg::PlayWild(Color::Yellow) })}
                >
                </div>

                <div
                    class="h-full w-1/2 rounded-lg" style="background-color: green"
                    onclick={link.callback(|_: MouseEvent| { Msg::PlayWild(Color::Green) })}
                >
                </div>
            </div>
        </div>
    };
}
