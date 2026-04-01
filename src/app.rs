use std::rc::Rc;

use log::*;
use web_sys::{Document, HtmlElement, HtmlImageElement, Navigator, Storage, Window};
use yew::*;
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

    let clock = DefaultSystemClock;
    let default_background = DefaultBackground::new(default_background.clone());
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
    let state: UseReducerHandle<ToastState> = use_reducer(Default::default);
    let toast_manager = ToastManager::new(state.clone());
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
                <ContextProvider<DefaultBackground> context={default_background.clone()}>
                    <Layout>
                        <article data-loading="" class="flex w-full h-full justify-center items-center" ontransitionend={on_transition_end}>
                            <Loader/>
                        </article>
                    </Layout>
                </ContextProvider<DefaultBackground>>
            }
        },
        AppState::Error(error) => {
            html! {
                <ContextProvider<DefaultBackground> context={default_background.clone()}>
                    <Layout>
                        <Error error={error.clone()} on_retry={on_retry}/>
                    </Layout>
                </ContextProvider<DefaultBackground>>
            }
        },
        AppState::Loaded(social) => {
            html! {
                <ContextProvider<DefaultSystemClock> context={clock}>
                    <ContextProvider<ApiClient> context={api_client}>
                        <ContextProvider<DefaultBackground> context={default_background.clone()}>
                            <ContextProvider<ScreenshotService> context={screenshot_service}>
                                <ContextProvider<ToastManager> context={toast_manager}>
                                    <ContextProvider<SettingsManager> context={settings_manager}>
                                        <ContextProvider<Repository> context={repository}>
                                            <ContextProvider<Social> context={social.clone()}>
                                                <Layout>
                                                    <SettingsWidget/>
                                                    <ToastWidget/>
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
                        </ContextProvider<DefaultBackground>>
                    </ContextProvider<ApiClient>>
                </ContextProvider<DefaultSystemClock>>
            }
        },
    }
}
