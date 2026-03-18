use yew::*;
use yew_router::Routable;
use crate::pages::Game;

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Game
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Game => html! { <Game/> },
    }
}