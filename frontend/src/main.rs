#![recursion_limit="500"]
use yew::prelude::*;
use yew_router::prelude::*;
mod pages;
mod components;
use crate::pages::home::Home;
use crate::pages::game::Game;

struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        return html! {
                <BrowserRouter>
                    <Switch<Route> render={Switch::render(switch)} />
                </BrowserRouter>
        };
    }
}

#[derive(Debug, Clone, PartialEq, Routable)]
pub enum Route  {
    #[at("/game/:id")]
    Lobby { id: String },
    #[not_found]
    #[at("/404")]
    PageNotFound,
    #[at("/")]
    HomePage,
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::HomePage => html! { <Home /> },
        Route::Lobby { id } => html! { <Game id = {id.clone()}/>},
        //Route::Lobby { id } => html! {<p>{format!("You are looking at game lobby {}", id)}</p>},
        Route::PageNotFound => html! { <h1>{ "404" }</h1> },
    }
}
fn main() {
    yew::start_app::<App>();
}
