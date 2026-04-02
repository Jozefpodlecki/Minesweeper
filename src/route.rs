use yew::*;
use yew_router::Routable;
use crate::pages::{ErrorPage, Game};

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Game,
    #[cfg(debug_assertions)]
    #[at("/error")]
    Error
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Game => html! { <Game/> },
        #[cfg(debug_assertions)]
        Route::Error => html! { <ErrorPage/> },
    }
}