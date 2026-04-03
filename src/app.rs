use std::rc::Rc;

use log::*;
use web_sys::{Document, HtmlElement, HtmlImageElement, Navigator, Storage, Window};
use yew::*;
use yew_router::{HashRouter, Switch};

use crate::{api::ApiClient, components::*, game::{DefaultGameManager, Repository}, models::*, route::{switch, Route}, services::*, testing::set_sample_records};

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

#[derive(Clone, Properties)]
pub struct AppProps {
    pub app_name: Rc<str>,
    pub version: Rc<str>,
    pub window: Window,
    pub document: Document,
    pub body: HtmlElement,
    pub default_background: HtmlImageElement,
    pub local_storage: Storage,
    pub navigator: Navigator,
    pub http_client: HttpClient,
    pub api_client: ApiClient
}

impl PartialEq for AppProps {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

#[function_component(App)]
pub fn app(props: &AppProps) -> Html {

    let AppProps {
        version,
        window,
        document,
        body,
        default_background,
        local_storage,
        navigator,
        api_client,
        ..
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

    let settings_accessor: StorageAccessor<Settings> = StorageAccessor::new("settings", local_storage.clone());
    let settings_state = use_state(|| settings_accessor.get().unwrap_or_default());
    let settings_manager = SettingsManager::new(settings_state.clone(), settings_accessor);
    settings_manager.init();
    let state_accessor = StorageAccessor::new("state", local_storage.clone());
    let game_manager = DefaultGameManager::new(
        clock.clone(),
        state_accessor,
        settings_state.clone());
    
    let state: UseReducerHandle<ToastState> = use_reducer(Default::default);
    let toast_manager = ToastManager::new(state.clone());
    let screenshot_service = ScreenshotService::new(document.clone(), body.clone(), navigator.clone());

    {
        let app_state = app_state.clone();
        let api_client = api_client.clone();

        use_effect_with(
            (),
            move |_| wasm_bindgen_futures::spawn_local(fetch_social(api_client, app_state)),
        );
    }

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
                        <article data-loading="" class="flex w-full h-full justify-center items-center">
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
                <ContextProvider<Window> context={window.clone()}>
                    <ContextProvider<DefaultSystemClock> context={clock}>
                        <ContextProvider<ApiClient> context={api_client.clone()}>
                            <ContextProvider<DefaultBackground> context={default_background.clone()}>
                                <ContextProvider<ScreenshotService> context={screenshot_service}>
                                    <ContextProvider<ToastManager> context={toast_manager}>
                                        <ContextProvider<SettingsManager> context={settings_manager}>
                                            <ContextProvider<DefaultGameManager> context={game_manager}>
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
                                            </ContextProvider<DefaultGameManager>>
                                        </ContextProvider<SettingsManager>>
                                    </ContextProvider<ToastManager>>
                                </ContextProvider<ScreenshotService>>
                            </ContextProvider<DefaultBackground>>
                        </ContextProvider<ApiClient>>
                    </ContextProvider<DefaultSystemClock>>
                </ContextProvider<Window>>
            }
        },
    }
}
