use chrono::{Duration, Utc};
use log::info;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{window, Document, HtmlElement, Navigator, Storage, Window};
use yew::*;
use yew_icons::{Icon, IconData};
use yew_router::{HashRouter, Switch};

use crate::{api::ApiClient, components::{Background, Error, Layout, Loader, Screenshot, Settings}, game::Repository, models::{AppError, AppState, GameResult, Social}, route::{switch, Route}, services::{ScreenshotService, SettingsManager, ToastManager}, utils::set_document_version};

async fn fetch_social(client: ApiClient, app_state: UseStateHandle<AppState>) {
    app_state.set(AppState::Loading);

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
    pub document: Document,
    pub body: HtmlElement,
    pub local_storage: Storage,
    pub navigator: Navigator
}

#[function_component(App)]
pub fn app(props: &AppProps) -> Html {

    let AppProps {
        window,
        document,
        body,
        local_storage,
        navigator
    } = props; 

    let app_state = use_state(AppState::default);
    let mut repository = Repository::new(local_storage.clone());

    repository.clear_records();

    let now = Utc::now();

    repository.set_last_record(GameResult {
        has_won: true,
        started_at: now - chrono::Duration::minutes(25),
        duration: chrono::Duration::minutes(5),
        rows: 10,
        columns: 10,
        revealed_count: 20,
        mines_count: 50,
        flags_count: 20
    });

    repository.set_last_record(GameResult {
        has_won: false,
        started_at: now - chrono::Duration::minutes(15),
        duration: chrono::Duration::minutes(5),
        rows: 10,
        columns: 10,
        revealed_count: 20,
        mines_count: 50,
        flags_count: 20
    });

    repository.set_last_record(GameResult {
        has_won: true,
        started_at: now,
        duration: chrono::Duration::minutes(5),
        rows: 10,
        columns: 10,
        revealed_count: 20,
        mines_count: 50,
        flags_count: 20
    });

    let settings_manager = SettingsManager::new(local_storage.clone());
    settings_manager.init();
    let client = ApiClient::new(window.clone());
    let toast_manager = ToastManager::new(window.clone());
    let screenshot_service = ScreenshotService::new(document.clone(), body.clone(), navigator.clone());

    use_effect_with(document.clone(), set_document_version);

    {
        let app_state = app_state.clone();
        let client = client.clone();

        use_effect_with(
            (),
            move |_| wasm_bindgen_futures::spawn_local(fetch_social(client, app_state)),
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
            let client = client.clone();

            wasm_bindgen_futures::spawn_local(fetch_social(client, app_state))
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
                <ContextProvider<ScreenshotService> context={screenshot_service}>
                    <ContextProvider<ToastManager> context={toast_manager}>
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
                    </ContextProvider<ToastManager>>
                </ContextProvider<ScreenshotService>>
            }
        },
    }
}
