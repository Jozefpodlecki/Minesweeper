use std::time::Duration;

use gloo::timers::future::sleep;
use log::info;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{window, Document, HtmlElement, Window};
use yew::*;
use yew_icons::{Icon, IconData};
use yew_router::{HashRouter, Switch};

use crate::{api::ApiClient, components::{Background, Error, Layout, Loader, Screenshot, Settings}, game::Repository, models::{AppError, AppState, Social}, route::{switch, Route}, services::SettingsManager, utils::set_document_version};

async fn fetch_social(app_state: UseStateHandle<AppState>) {
    app_state.set(AppState::Loading);
    // app_state.set(AppState::Error(AppError::failed_to_build_request("test".into())));
    // sleep(Duration::from_secs(2000)).await;

    let client = ApiClient::new();

    match client.get_social().await {
        Ok(social) => {
            app_state.set(AppState::Loaded(social));
        }
        Err(err) => {
            app_state.set(AppState::Error(err));
        }
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct AppProps {
    pub window: Window,
    pub document: Document
}

#[function_component(App)]
pub fn app(props: &AppProps) -> Html {
    let app_state = use_state(AppState::default);
    let repository = Repository::new();
    let settings_manager = SettingsManager::new();
    use_effect_with((), set_document_version);

    {
        let app_state = app_state.clone();
        use_effect_with(
            (),
            move |_| wasm_bindgen_futures::spawn_local(fetch_social(app_state)),
        );
    }

    let on_transition_end: Callback<TransitionEvent> = {
        

        Callback::from(move |event: TransitionEvent| {
            info!("TransitionEvent")
        })
    };

    let on_retry: Callback<MouseEvent> = {
        let app_state = app_state.clone();

        Callback::from(move |event: MouseEvent| {
            let app_state = app_state.clone();
            wasm_bindgen_futures::spawn_local(fetch_social(app_state))
        })
    };

    match &*app_state {
        AppState::Loading => {
            html! {
                <Layout>
                    <article data-loading="" class="flex w-full h-full justify-center items-center" ontransitionend={on_transition_end}>
                        <Loader/>
                    </article>
                </Layout>
            }
        },
        AppState::Error(error) => {
            html! {
                <Layout>
                    <Error error={error.clone()} on_retry={on_retry}/>
                </Layout>
            }
        },
        AppState::Loaded(social) => {
            html! {
                <ContextProvider<SettingsManager> context={settings_manager}>
                    <ContextProvider<Repository> context={repository}>
                        <ContextProvider<Social> context={social.clone()}>
                            <Layout>
                                <Settings/>
                                <Screenshot/>
                                <HashRouter>
                                    <Switch<Route> render={switch} />
                                </HashRouter>
                            </Layout>
                        </ContextProvider<Social>>
                    </ContextProvider<Repository>>
                </ContextProvider<SettingsManager>>
            }
        },
    }
}
