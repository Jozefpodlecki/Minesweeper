use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{HtmlElement, HtmlImageElement};
use yew::*;
use log::*;
use crate::{extensions::ExtensionsNodeRef, models::BackgroundSource, route::Route, services::SettingsManager};


#[function_component(Background)]
pub fn background() -> Html {
    let background_manager = use_context::<HtmlImageElement>().unwrap(); 
    let settings_manager = use_context::<SettingsManager>(); 
    let background = match settings_manager.map(|pr| pr.get()) {
        Some(value) => value.background,
        None => BackgroundSource::Default,
    };
    let img_ref = use_node_ref();

    {
        let img_ref = img_ref.clone();
        
        use_effect_with((), move |_| {
            let styles = background_manager.style();
            let img: HtmlElement = img_ref.unchecked_cast();

            let closure = Closure::wrap(Box::new(move |_e: web_sys::Event| {
                if let Some(img) = img_ref.cast::<HtmlElement>() {
                    img.style().set_property("opacity", "1").unwrap();
                }
            }) as Box<dyn FnMut(_)>);

            img.add_event_listener_with_callback("transitionend", closure.as_ref().unchecked_ref()).unwrap();
            closure.forget();
            styles.set_property("opacity", "0").unwrap();
        });
    }

    {
        use_effect_with((), move |_| {
            
        });
    }

    let on_transition_end: Callback<TransitionEvent> = {
        

        Callback::from(move |event: TransitionEvent| {
            info!("TransitionEvent")
        })
    };

    match background {
        BackgroundSource::Default => {
            html! {
                <img
                    ontransitionend={on_transition_end}
                    ref={img_ref.clone()}
                    data-background=""
                    class="transition-all fixed inset-0 w-full h-full object-cover -z-10 brightness-50"
                    src={"public/background.jpg"}
                    alt="background" />
            }
        },
        BackgroundSource::FileSystem { uploaded_on, file_name, data_url } => {
            html! {
                <img
                    ontransitionend={on_transition_end}
                    ref={img_ref.clone()}
                    data-background=""
                    data-file-name={file_name.to_string()}
                    data-uploaded-on={uploaded_on.to_rfc3339()}
                    class="transition-all fixed inset-0 w-full h-full object-cover -z-10 brightness-50"
                    src={data_url.to_string()}
                    alt="background" />
            }
        },
        BackgroundSource::Url { uploaded_on, url, data_url } => {
            html! {
                <img
                    ontransitionend={on_transition_end}
                    ref={img_ref.clone()}
                    data-background=""
                    data-url={url.to_string()}
                    data-uploaded-on={uploaded_on.to_rfc3339()}
                    class="transition-all fixed inset-0 w-full h-full object-cover -z-10 brightness-50"
                    src={data_url.to_string()}
                    alt="background" />
            }
        },
    }
}