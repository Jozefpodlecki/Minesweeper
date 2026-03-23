use web_sys::{File, HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;

use crate::{components::DragAndDrop, models::*, services::{DefaultSystemClock, SystemClock}};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub value: BackgroundSource,
    pub on_change: Callback<BackgroundSource>,
}

#[function_component(BackgroundSelector)]
pub fn background_selector(props: &Props) -> Html {

    let Props { value, on_change } = props;
    let clock = DefaultSystemClock;
    let bg_type = value.name();
    let selected = use_state(|| value.clone() );

    let on_type_change = {
        let clock = clock.clone();
        let on_change = on_change.clone();
        let selected = selected.clone();
        
        Callback::from(move |event: Event| {
            let select: HtmlSelectElement = event.target_unchecked_into();
            let next_type = select.value();

            let next_bg = match next_type.as_str() {
                "default" => BackgroundSource::Default,
                "file" => BackgroundSource::FileSystem {
                    uploaded_on: Default::default(),
                    file_name: "".into(),
                    data_url: "".into(),
                },
                "url" => BackgroundSource::Url {
                    uploaded_on: Default::default(),
                    url: "".into(),
                    data_url: "".into(),
                },
                _ => BackgroundSource::Default,
            };

            selected.set(next_bg);
            // on_change.emit(next_bg);
        })
    };

    let on_url = {
        let clock = clock.clone();
        let on_change = on_change.clone();

        Callback::from(move |event: InputEvent| {
            let input: HtmlInputElement = event.target_unchecked_into();
            let url = input.value();



            let value = BackgroundSource::Url {
                uploaded_on: clock.utc_now(),
                url: "".into(),
                data_url: "".into()
            };

            on_change.emit(value);
        })            
    };

    let on_file: Callback<File> = {
        let clock = clock.clone();
        let on_change = on_change.clone();

        Callback::from(move |file: File| {
            // let input: HtmlInputElement = event.target_unchecked_into();
            // let url = input.value();

            let value = BackgroundSource::FileSystem {
                uploaded_on: clock.utc_now(),
                file_name: "".into(),
                data_url: "".into()
            };

            on_change.emit(value);
        })          
    };

    html! {
        <label class="flex flex-col text-sm">
            {"Background"}
            <select class="mt-1 p-2 bg-black border" value={bg_type} onchange={on_type_change}>
                <option value="default" selected={"default" == bg_type}>{"Default"}</option>
                <option value="file" selected={"file" == bg_type}>{"Upload file"}</option>
                <option value="url" selected={"url" == bg_type}>{"URL"}</option>
            </select>

            {
                match &*selected {
                    BackgroundSource::FileSystem { data_url, file_name, .. } => html! {
                        <>
                            <DragAndDrop on_change={on_file} />
                            <img data-file-name={file_name.to_string()} src={data_url.to_string()} class="" tag="preview-thumbnail" />
                        </>
                    },
                    BackgroundSource::Url { url, data_url, .. } => html! {
                        <>
                            <input
                                type="text"
                                class="mt-1 p-2 bg-black border"
                                value={url.to_string()}
                                oninput={on_url}
                                placeholder={"Enter url"}
                            />
                            <img src={data_url.to_string()} class="" tag="preview-thumbnail" />
                        </>
                    },
                    _ => html! {},
                }
            }
        </label>
    }
}