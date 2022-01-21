#![recursion_limit="500"]
use yew::prelude::*;

use yew_router::{prelude::*, Switch};

use yew_router::switch::{Permissive};
mod pages;
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
                <Router<AppRoute>
                    render = Router::render(|switch: AppRoute| {
                        match switch {
                            AppRoute::Test => {
                                html!{<Home />}
                            },
                            AppRoute::HomePage => html!{<Home />},
                            AppRoute::Lobby(u32) => html!{<Game />},
                            AppRoute::PageNotFound(Permissive(None)) => html!{"Page not found"},
                            AppRoute::PageNotFound(Permissive(Some(missed_route))) => html!{format!("Page '{}' not found", missed_route)}
                        }
                    })
                    redirect = Router::redirect(|route: Route| {
                        AppRoute::PageNotFound(Permissive(Some(route.route)))
                    })
                />
        };
    }
}

#[derive(Debug, Switch, Clone)]
pub enum AppRoute  {
    #[to = "/game/{id}"]
    Lobby(u32),
    #[to = "/test"]
    Test,
    #[to = "/page-not-found"]
    PageNotFound(Permissive<String>),
    #[to = "/!"]
    HomePage,
}

fn main() {
    yew::start_app::<App>();
}
