use chrono::{DateTime, Utc};
use log::*;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::window;
use yew::*;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub started_at: DateTime<Utc>
}

#[function_component(Timer)]
pub fn timer(props: &Props) -> Html {
    let now = use_state(Utc::now);

    {
        let now = now.clone();
        use_effect(move || {
            let window = window().unwrap();

            let closure = Closure::wrap(Box::new(move || {
                now.set(Utc::now());
            }) as Box<dyn FnMut()>);

            let handle = window
                .set_interval_with_callback_and_timeout_and_arguments_0(
                    closure.as_ref().unchecked_ref(),
                    1000,
                )
                .unwrap();

            closure.forget();

            move || {
                window.clear_interval_with_handle(handle);
            }
        });
    }

    let duration = *now - props.started_at;

    let seconds = duration.num_seconds() % 60;
    let minutes = duration.num_minutes() % 60;
    let hours = duration.num_hours();
    
    html! {
        <span>
            {format!("{:02}:{:02}:{:02}", hours, minutes, seconds)}
        </span>
    }

}