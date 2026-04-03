use gloo::events::EventListener;
use wasm_bindgen::JsCast;
use web_sys::{BeforeUnloadEvent, Window};
use log::*;
use yew::prelude::*;

#[function_component(UnsavedGuard)]
pub fn unsaved_guard() -> Html {
    let window = use_context::<Window>().unwrap(); 

    {
        let window = window.clone();

        use_effect(move || {

            let listener = EventListener::new(&window, "beforeunload", move |event| {
                let event = event.unchecked_ref::<BeforeUnloadEvent>();
                event.prevent_default();
                event.set_return_value("true");
            });

            move || drop(listener)
        });
    }

    html! {}
}