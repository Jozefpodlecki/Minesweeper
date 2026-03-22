use std::hint::unreachable_unchecked;

use log::info;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use yew::*;
use yew_router::prelude::Link;

use crate::{extensions::*, route::Route};
use yew_icons::{Icon, IconData};

#[derive(Default)]
enum Action {
    Open,
    #[default]
    Close,
}

impl Action {
    fn from_str(element: HtmlElement) -> Action {
        let action = element.dataset().get_unchecked("action");

        match action.as_str() {
            "open" => Action::Open,
            "close" => Action::Close,
            _ => unsafe { unreachable_unchecked() },
        }
    }
}

fn resolve_action_target(event: &MouseEvent) -> Option<HtmlElement> {
    let target = unsafe { event.target().unwrap_unchecked() };
    let element: HtmlElement = target.unchecked_into();

    if element.tag_name() == "DIV" {
        return None;
    }

    unsafe {
        element.closest("button")
            .unwrap_unchecked()
            .map(|e| e.unchecked_into::<HtmlElement>())
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
}

#[function_component(Settings)]
pub fn settings(props: &Props) -> Html {
    let is_open = use_state(|| false);
   
    let on_action: Callback<MouseEvent> = {
        let is_open = is_open.clone();
        Callback::from(move |event: MouseEvent| {

            let action = resolve_action_target(&event)
                .map(Action::from_str)
                .unwrap_or_default();

            match action {
                Action::Open => is_open.set(true),
                Action::Close => is_open.set(false),
            }
        })
    };

    let stop_propagation = Callback::from(|event: MouseEvent| {
        event.stop_propagation();
    });

    html! {
        <>
            <section class="absolute top-0 left-0 text-white">
                <button
                    data-action="open"
                    type="button"
                    onclick={&on_action}
                    class="p-2 hover:bg-white/10 rounded transition"
                >
                    <Icon data={IconData::LUCIDE_SETTINGS} width={"20px"} />
                </button>
            </section>

            if *is_open {
                <div
                    data-action="close"
                    class="fixed inset-0 bg-black/50 flex items-center justify-center z-50"
                    onclick={&on_action}
                >
                    <div
                        class="bg-black/70 text-white p-6 rounded shadow-lg min-w-[300px]"
                        onclick={stop_propagation}
                    >
                        <h2 class="text-lg mb-4">{"Settings"}</h2>

                        <button
                            data-action="open"
                            type="button"
                            onclick={&on_action}
                            class="mt-2 px-4 py-2 border hover:bg-white/10 transition"
                        >
                            {"Close"}
                        </button>
                    </div>
                </div>
            }
        </>
    }

}