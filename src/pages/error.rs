use gloo::timers::callback::Timeout;
use yew::prelude::*;

#[function_component(ErrorPage)]
pub fn error() -> Html {
    
    {
        use_effect_with((), move |_| {
            
            let timeout = Timeout::new(1000, move || {
                panic!("Wasm error");
            });

            timeout.forget();
        });
    }

    html! {}

}