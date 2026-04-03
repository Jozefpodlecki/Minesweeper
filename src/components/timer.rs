use chrono::{DateTime, Utc};
use log::*;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::window;
use yew::*;
use yew_icons::{Icon, IconData};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub started_at: DateTime<Utc>,
    pub is_running: bool,
}

#[function_component(Timer)]
pub fn timer(props: &Props) -> Html {
    let now = use_state(Utc::now);

    {
        let now = now.clone();

        use_effect_with(
            props.is_running,
            move |is_running| {
                let window = window().unwrap();

                if !*is_running {
                    return Box::new(|| {}) as Box<dyn FnOnce()>;
                }

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

                Box::new(move || {
                    window.clear_interval_with_handle(handle);
                })
            },
        );
    }

    let duration = *now - props.started_at;

    let seconds = duration.num_seconds() % 60;
    let minutes = duration.num_minutes() % 60;
    let hours = duration.num_hours();
    
    html! {
        <div data-timer="" class="flex items-center gap-2">
            <Icon data={IconData::LUCIDE_CLOCK} width={"20px"}/>
            {format!("{:02}:{:02}:{:02}", hours, minutes, seconds)}
        </div>
    }

}