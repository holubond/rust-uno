#![recursion_limit = "500"]
use yew::prelude::*;
use yew_router::prelude::*;
mod components;
mod pages;
mod util;
mod url;
use crate::pages::game::Game;
use crate::pages::home::Home;

struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                <Switch<Route> render={Switch::render(switch)} />
            </BrowserRouter>
        }
    }
}

#[derive(Debug, Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    HomePage,

    #[at("/game/:id")]
    Lobby { id: String },

    #[not_found]
    #[at("/404")]
    PageNotFound,
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::HomePage => html! { <Home /> },
        Route::Lobby { id } => html! { <Game id = {id.clone()}/>},
        Route::PageNotFound => html! { <h1>{ "404" }</h1> },
    }
}
fn main() {
    yew::start_app::<App>();
}
