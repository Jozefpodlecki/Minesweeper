use std::rc::Rc;

use chrono::{Duration, Utc};
use log::info;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{window, Document, HtmlElement, HtmlImageElement, Navigator, Storage, Window};
use yew::*;
use yew_icons::{Icon, IconData};
use yew_router::{HashRouter, Switch};

use crate::{api::ApiClient, components::*, game::Repository, models::*, route::{switch, Route}, services::*, testing::set_sample_records, utils::set_document_version};

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
    pub app_name: Rc<str>,
    pub version: Rc<str>,
    pub window: Window,
    pub document: Document,
    pub body: HtmlElement,
    pub default_background: HtmlImageElement,
    pub local_storage: Storage,
    pub navigator: Navigator
}

#[function_component(App)]
pub fn app(props: &AppProps) -> Html {

    let AppProps {
        app_name,
        version,
        window,
        document,
        body,
        default_background,
        local_storage,
        navigator
    } = props; 

    let app_state = use_state(AppState::default);
    let mut repository = Repository::new(local_storage.clone());

    {
        let repository = repository.clone();

        use_effect_with((), move |_| {
            repository.clear_records();
            set_sample_records(&repository);
        });

    }

    let http_client = HttpClient::new(window.clone(), version.clone(), app_name.clone());
    let settings_accessor = StorageAccessor::new("settings", local_storage.clone());
    let settings_state = use_state(Settings::default);
    let settings_manager = SettingsManager::new(settings_state, settings_accessor);
    settings_manager.init();

    let api_client = ApiClient::new(http_client.clone());
    let toast_manager = ToastManager::new(window.clone());
    let screenshot_service = ScreenshotService::new(document.clone(), body.clone(), navigator.clone());

    use_effect_with((version.clone(), document.clone()), set_document_version);

    {
        let app_state = app_state.clone();
        let api_client = api_client.clone();

        use_effect_with(
            (),
            move |_| wasm_bindgen_futures::spawn_local(fetch_social(api_client, app_state)),
        );
    }

    let on_transition_end: Callback<TransitionEvent> = {
        

        Callback::from(move |event: TransitionEvent| {
            info!("TransitionEvent")
        })
    };

    let on_retry: Callback<MouseEvent> = {
        let app_state = app_state.clone();
        let api_client = api_client.clone();

        Callback::from(move |event: MouseEvent| {
            let app_state = app_state.clone();
            let api_client = api_client.clone();

            wasm_bindgen_futures::spawn_local(fetch_social(api_client, app_state))
        })
    };

    match &*app_state {
        AppState::Loading => {
            html! {
                <ContextProvider<HtmlImageElement> context={default_background.clone()}>
                    <Layout>
                        <article data-loading="" class="flex w-full h-full justify-center items-center" ontransitionend={on_transition_end}>
                            <Loader/>
                        </article>
                    </Layout>
                </ContextProvider<HtmlImageElement>>
            }
        },
        AppState::Error(error) => {
            html! {
                <ContextProvider<HtmlImageElement> context={default_background.clone()}>
                    <Layout>
                        <Error error={error.clone()} on_retry={on_retry}/>
                    </Layout>
                </ContextProvider<HtmlImageElement>>
            }
        },
        AppState::Loaded(social) => {
            html! {
                <ContextProvider<ApiClient> context={api_client}>
                    <ContextProvider<HtmlImageElement> context={default_background.clone()}>
                        <ContextProvider<ScreenshotService> context={screenshot_service}>
                            <ContextProvider<ToastManager> context={toast_manager}>
                                <ContextProvider<SettingsManager> context={settings_manager}>
                                    <ContextProvider<Repository> context={repository}>
                                        <ContextProvider<Social> context={social.clone()}>
                                            <Layout>
                                                <SettingsWidget/>
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
                    </ContextProvider<HtmlImageElement>>
                </ContextProvider<ApiClient>>
            }
        },
    }
}
