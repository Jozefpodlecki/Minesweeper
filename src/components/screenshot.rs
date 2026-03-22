use yew::*;
use yew_router::prelude::Link;

use crate::{route::Route};
use yew_icons::{Icon, IconData};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
}

#[function_component(Screenshot)]
pub fn screenshot(props: &Props) -> Html {
 
    let on_screenshot: Callback<MouseEvent> = {
       
        Callback::from(move |event| {
       

        })
    };

    html! {
        <>
            <section class="absolute top-0 right-0 text-white">
                <button
                    type="button"
                    onclick={on_screenshot}
                    class="p-2 hover:bg-white/10 rounded transition"
                >
                    <Icon data={IconData::LUCIDE_CAMERA} width={"20px"} />
                </button>
            </section>
        </>
    }

}