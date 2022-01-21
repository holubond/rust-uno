#![recursion_limit="500"]
use yew::prelude::*;
use yew_router::{route::Route, switch::Permissive};

mod pages;
mod route;
use route::{AppAnchor, AppRoute, AppRouter, PublicUrlSwitch};
use crate::pages::home::Home;
use crate::pages::game::Game;

struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        return html! {
            <main>
                <AppRouter
                    render=AppRouter::render(Self::switch)
                    redirect=AppRouter::redirect(|route: Route| {
                        AppRoute::PageNotFound(Permissive(Some(route.route))).into_public()
                    })
                />
            </main>
        };
    }
}

impl App {
    fn switch(switch: PublicUrlSwitch) -> Html {
        match switch.route() {
            AppRoute::Lobby(id) => {
                return html! {<Game />};
            }
            AppRoute::Test => {
                return html! {
                    <main>
                        <h1>{"lul"}</h1>
                    </main>};
            }
            AppRoute::HomePage => {
                return html! {<Home />};
            }
            AppRoute::PageNotFound(Permissive(route)) => {
                return html! {
                    <main>
                        <h1>{"My custom pageNotFound"}</h1>
                    </main>};
            }
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
