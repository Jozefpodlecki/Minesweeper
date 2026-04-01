use web_sys::HtmlElement;
use yew::*;
use log::*;
use crate::{extensions::ExtensionsNodeRef, models::BackgroundSource, services::{DefaultBackground, SettingsManager}};


#[function_component(Background)]
pub fn background() -> Html {
    let background_manager = use_context::<DefaultBackground>().unwrap(); 
    let settings_manager = use_context::<SettingsManager>();
    let background = match settings_manager.as_ref().map(|pr| pr.get()) {
        Some(value) => value.background,
        None => BackgroundSource::Default,
    };
    let prev_background = use_state(|| match settings_manager.map(|pr| pr.get()) {
        Some(value) => value.background,
        None => BackgroundSource::Default,
    });
    let img_ref = use_node_ref();

    {
        use_effect_with((), move |_| {
            background_manager.set_opacity();
        });
    }

    {
        let background = background.clone();
        let prev_background = prev_background.clone();
        let img_ref = img_ref.clone();

        use_effect_with((prev_background, background), move |(prev_background_handle, background)| {
            let img: HtmlElement = img_ref.unchecked_cast();

            let prev_background = &*prev_background_handle.clone();

            if prev_background != background {
                img.style().set_property("opacity", "0").unwrap();
            }
        });
    }

    let on_transition_end: Callback<TransitionEvent> = {
        let background = background.clone();
        let prev_background = prev_background.clone();

        Callback::from(move |event: TransitionEvent| {
            
            if *prev_background == background {
                return;
            }

            prev_background.set(background.clone());
            let img: HtmlElement = event.target_unchecked_into();
            img.style().set_property("opacity", "1").unwrap();
        })
    };

    let (src, uploaded_on, file_name, data_url): (
        AttrValue,
        Option<AttrValue>,
        Option<AttrValue>,
        Option<AttrValue>,
    ) = match &*prev_background {
        BackgroundSource::Default => (
            "public/background.jpg".into(),
            None,
            None,
            None,
        ),
        BackgroundSource::FileSystem { uploaded_on, file_name, data_url } => (
            data_url.to_string().into(), 
            Some(uploaded_on.to_rfc3339().into()),
            Some(file_name.to_string().into()),
            None,
        ),
        BackgroundSource::Url { uploaded_on, url, data_url } => (
            data_url.to_string().into(),
            Some(uploaded_on.to_rfc3339().into()),
            None,
            Some(url.to_string().into()),
        ),
    };

    html! {
        <img
            ontransitionend={on_transition_end}
            ref={img_ref}
            data-background=""
            data-uploaded-on={uploaded_on}
            data-file-name={file_name}
            data-url={data_url}
            class="transition-all duration-100 fixed inset-0 w-full h-full object-cover -z-10 brightness-50"
            src={src.to_string()}
            alt="background"
        />
    }
}