use yew::*;
use yew_router::prelude::Link;

use crate::{models::BackgroundSource, route::Route, services::SettingsManager};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub src: String
}

#[function_component(Background)]
pub fn background(props: &Props) -> Html {
    let settings_manager = use_context::<SettingsManager>(); 
    let background = match settings_manager.map(|pr| pr.get()) {
        Some(value) => value.background,
        None => BackgroundSource::Default,
    };

    match background {
        BackgroundSource::Default => {
            html! {
                <img
                    class="fixed inset-0 w-full h-full object-cover -z-10 brightness-50"
                    src={props.src.clone()}
                    alt="background" />
            }
        },
        BackgroundSource::FileSystem { uploaded_on, file_name, data_url } => {
            html! {
                <img
                    data-file-name={file_name.to_string()}
                    data-uploaded-on={uploaded_on.to_rfc3339()}
                    class="fixed inset-0 w-full h-full object-cover -z-10 brightness-50"
                    src={data_url.to_string()}
                    alt="background" />
            }
        },
        BackgroundSource::Url { uploaded_on, url, data_url } => {
            html! {
                <img
                    data-url={url.to_string()}
                    data-uploaded-on={uploaded_on.to_rfc3339()}
                    class="fixed inset-0 w-full h-full object-cover -z-10 brightness-50"
                    src={data_url.to_string()}
                    alt="background" />
            }
        },
    }
}