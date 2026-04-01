use log::*;
use web_sys::{File, HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;
use url::Url;
use yew_icons::{Icon, IconData};
use crate::{api::ApiClient, components::{BackgroundSourceSelector, DragAndDrop, Loader}, models::*, services::*};

#[derive(Debug, Default, Clone, PartialEq)]
pub enum UploadState {
    #[default]
    Idle,
    Loading,
    Error(AppError)
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub previous: BackgroundSource,
    pub value: BackgroundSource,
    pub on_change: Callback<BackgroundSource>,
}

#[function_component(BackgroundSelector)]
pub fn background_selector(props: &Props) -> Html {

    let Props { previous, value, on_change } = props;
    let clock = use_context::<DefaultSystemClock>().unwrap();
    let bg_type: Box<str> = value.name().into();
    let state = use_state(UploadState::default);
    let default_background = use_context::<DefaultBackground>().unwrap();
    let api_client = use_context::<ApiClient>().unwrap();

    let on_type_change = {
        let on_change = on_change.clone();
        let previous = previous.clone();

        Callback::from(move |event: Event| {
            let select: HtmlSelectElement = event.target_unchecked_into();
            let next_type = select.value();
            let next = BackgroundSource::from_str(&next_type);

            if previous.name() == next.name() {
                on_change.emit(previous.clone());
            }
            else {
                on_change.emit(next);
            }
        })
    };

    let on_url = {
        let selected = value.clone();
        let clock = clock.clone();
        let on_change = on_change.clone();
        let api_client = api_client.clone();
        let state = state.clone();
        
        Callback::from(move |event: InputEvent| {
            let input: HtmlInputElement = event.target_unchecked_into();
            let on_change = on_change.clone();
            let url = input.value();
            let clock = clock.clone();
            let api_client = api_client.clone();
            let state = state.clone();
            let mut new_value = selected.clone();

            new_value.set_url(&url);
            on_change.emit(new_value);

            match Url::parse(&url) {
                Ok(url) => {
                    state.set(UploadState::Loading);
                    wasm_bindgen_futures::spawn_local(async move {
                        // let test = api_client.get_image(url.as_str())
                        //     .await
                        //     .and_then(ApiClient::blob_to_data_url);

                        match api_client.get_image(url.as_str()).await {
                            Ok(blob) => {
                                match ApiClient::blob_to_data_url(&blob).await {
                                    Ok(data_url) => {
                                        
                                        let value = BackgroundSource::Url {
                                            uploaded_on: clock.utc_now(),
                                            url: url.as_str().into(),
                                            data_url: data_url.into()
                                        };

                                        state.set(UploadState::Idle);
                                        on_change.emit(value);
                                    },
                                    Err(err) => {
                                        state.set(UploadState::Error(err));
                                    },
                                }
                            },
                            Err(err) => {
                                state.set(UploadState::Error(err));
                            },
                        }
                    });
                },
                Err(err) => {},
            }
        })            
    };

    let on_file: Callback<Option<File>> = {
        let selected = value.clone();
        let clock = clock.clone();
        let on_change = on_change.clone();

        Callback::from(move |file: Option<File>| {
            let selected = selected.clone();
            let clock = clock.clone();
            let on_change = on_change.clone();

            let file = match file {
                Some(value) => value,
                None => {
                    let mut source: BackgroundSource = selected.clone();
                    source.clear();

                    on_change.emit(source);

                    return;
                },
            };

            let file_name = file.name();
            wasm_bindgen_futures::spawn_local(async move {
                match AsyncFileReader::read_as_data_url(file).await {
                    Ok(data_url) => {
                         let value = BackgroundSource::FileSystem {
                            uploaded_on: clock.utc_now(),
                            file_name: file_name.clone().into(),
                            data_url: data_url.into(),
                        };

                        // selected.set(value.clone());
                        on_change.emit(value);
                    },
                    Err(_) => todo!(),
                }
            })
        })          
    };

    let content = match (&value, &*state) {
        (BackgroundSource::FileSystem { data_url, .. }, _) => {

            html! {
                <div class="flex-1">
                    <DragAndDrop
                        data_url={(!data_url.is_empty()).then(|| data_url.to_owned())}
                        on_change={on_file} />
                </div>
            }
        },
        (BackgroundSource::Url { url, data_url, .. }, UploadState::Idle) => {
            html! {
                <div class="w-full h-48 flex flex-col gap-2">
                    <input
                        type="text"
                        class="w-full mt-1 p-2 bg-black border"
                        value={url.to_string()}
                        oninput={on_url}
                        placeholder={"Enter url or copy from clipboard..."}
                    />
                    { if data_url.is_empty() { html! {
                        <div data-idle="" class="flex-1 flex justify-center items-center border-2 border-dashed p-4 border-gray-500">
                            <Icon data={IconData::LUCIDE_CAMERA} width={"30px"} />
                        </div>
                    } } else { html! {
                        <div data-idle="" class="flex-1 min-h-0">
                            <img data-thumbnail="" src={data_url.to_string()} class="w-full h-full object-cover block" tag="preview-thumbnail" />
                        </div>
                    } } }
                </div>
            }
        },
        (BackgroundSource::Url { url, .. }, UploadState::Error(error)) => {
            html! {
                <div class="w-full h-48 flex flex-col gap-2">
                    <input
                        type="text"
                        class="w-full mt-1 p-2 bg-black border"
                        value={url.to_string()}
                        oninput={on_url}
                        placeholder={"Enter url or copy from clipboard..."}
                    />
                    <div data-idle="" class="flex-1 flex flex-col justify-center items-center gap-2 border-2 border-dashed p-4 border-gray-500">
                        <Icon data={IconData::LUCIDE_ALERT_TRIANGLE} width={"30px"} />
                        <span class="text-sm font-medium">{ error.to_string() }</span>
                    </div>
                </div>
            }
        }
        (_, UploadState::Loading) => {
            html! {
                <div data-loading="" class="flex-1 h-48 flex justify-center items-center border-2 border-dashed p-4 border-gray-500">
                    <Loader/>
                </div>
            }
        },
        (BackgroundSource::Default, _) => {
            html! {
                <img
                    data-thumbnail=""
                    src={default_background.src()}
                    class="w-full h-48 object-cover block brightness-50" tag="preview-thumbnail" />
            }
        },
    };

    html! {
        <label class="flex flex-col text-sm">
            {"Background"}
            <BackgroundSourceSelector on_change={on_type_change} value={bg_type} />

            <div data-source={value.name()} class="mt-4 flex">
                {content}
            </div>
        </label>
    }
}