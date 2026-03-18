use yew::*;
use yew_router::{HashRouter, Switch};

use crate::{api::get_social, components::Loader, models::{AppState, Social}, route::{switch, Route}};

async fn fetch_social(app_state: UseStateHandle<AppState>) {
    app_state.set(AppState::Loading);

    match get_social().await {
        Ok(social) => {
            app_state.set(AppState::Loaded(social));
        }
        Err(err) => {
            app_state.set(AppState::Error(err));
        }
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let app_state = use_state(|| AppState::default() );

     {
        let app_state = app_state.clone();
        use_effect_with(
            (),
            move |_| {
                wasm_bindgen_futures::spawn_local(fetch_social(app_state.clone()));
            },
        );
    }

    match (*app_state).clone() {
        AppState::Loading => {
            html! { <div></div> }
        },
        AppState::Error(fetch_error) => {
            html! { <div></div> }
        },
        AppState::Loaded(social) => {
            html! {
                <ContextProvider<Social> context={social}>
                    <HashRouter>
                        <Switch<Route> render={switch} />
                    </HashRouter>
                </ContextProvider<Social>>
            }
        },
    }
}
