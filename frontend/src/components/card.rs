use std::fmt;
use yew::prelude::*;
use yew::{function_component, html};

pub struct Card;
#[derive(Clone, PartialEq, Properties)]
pub struct CardProps {
    pub color: Color,
    pub _type: CardType,
    pub value: Option<u32>,
}
#[derive(PartialEq, Clone)]
 pub enum Color {
    Red,
    Yellow,
    Green,
    Blue,
    Black,
}
impl fmt::Debug for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Color::Red => write!(f, "red"),
            Color::Yellow => write!(f, "yellow"),
            Color::Green => write!(f, "green"),
            Color::Blue => write!(f, "blue"),
            Color::Black => write!(f, "black"),
        }
    }
}
#[derive(PartialEq, Clone)]
pub enum CardType {
    Value,
    Skip,
    Reverse,
    Draw2,
    Draw4,
    Wild,
}
impl fmt::Debug for CardType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CardType::Skip => write!(f, "Skip"),
            CardType::Reverse => write!(f, "Reverse"),
            CardType::Draw2 => write!(f, "+2"),
            CardType::Draw4 => write!(f, "+4"),
            CardType::Wild => write!(f, "Wild"),
            _ => write!(f,"value"),
        }
    }
}
impl Component for Card {
    type Message = ();
    type Properties = CardProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        if ctx.props()._type.clone() != CardType::Value{
            return html!{
                <div class="w-full h-full flex flex-col rounded-lg bg-red-100 shadow-md" style={format!("background-color: {:?}", ctx.props().color.clone())}>
                    <div class="h-1/3">
                        <p class="text-5xl text-left text-White-500 font-bold">{format!("{:?}",ctx.props()._type.clone())}</p>
                    </div>
                    <div class="h-1/3 flex justify-center">
                        <p class="text-5xl text-center align-bottom bg-gray-300 text-Black-500 font-bold">{format!("{:?}",ctx.props()._type.clone())}</p>
                    </div>
                    <div class="h-1/3">
                        <p class="text-5xl text-right text-White-500 font-bold">{format!("{:?}",ctx.props()._type.clone())}</p>
                    </div>
                </div>
            }
        }
        return html!{
            <div class="w-full h-full flex flex-col rounded-lg bg-red-100 shadow-md" style={format!("background-color: {:?}", ctx.props().color.clone())}>
                <div class="h-1/3">
                    <p class="text-8xl text-left text-White-500 font-bold">{format!("{}",ctx.props().value.unwrap())}</p>
                </div>
                <div class="h-1/3 flex justify-center">
                    <p class="text-8xl text-center bg-gray-300 text-Black-500 font-bold">{format!("{}",ctx.props().value.unwrap())}</p>
                </div>
                <div class="h-1/3">
                    <p class="text-8xl text-right text-White-500 font-bold">{format!{"{}",ctx.props().value.unwrap()}}</p>
                </div>
            </div>
        };
    }
}
