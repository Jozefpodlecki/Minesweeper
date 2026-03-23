use std::hint::unreachable_unchecked;

use log::info;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, HtmlInputElement, HtmlSelectElement};
use yew::*;
use yew_router::prelude::Link;

use crate::{extensions::*, route::Route, services::SettingsManager};
use yew_icons::{Icon, IconData};

#[derive(Default)]
enum Action {
    Open,
    #[default]
    Save,
    Close,
}

impl Action {
    fn from_str(element: HtmlElement) -> Action {
        let action = element.dataset().get_unchecked("action");

        match action.as_str() {
            "open" => Action::Open,
            "save" => Action::Save,
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
    let settings_manager = unsafe { use_context::<SettingsManager>().unwrap_unchecked() };
    let settings = use_state(|| settings_manager.get());
   
    let on_action: Callback<MouseEvent> = {
        let is_open = is_open.clone();
        let settings = settings.clone();

        Callback::from(move |event: MouseEvent| {

            let action = resolve_action_target(&event)
                .map(Action::from_str)
                .unwrap_or_default();

            match action {
                Action::Open => is_open.set(true),
                Action::Save => {

                    let settings = (&*settings).clone();
                    settings_manager.save(settings);

                    is_open.set(false)  
                },
                Action::Close => is_open.set(false),
            }
        })
    };

    let on_difficulty_change = {
        let settings = settings.clone();

        Callback::from(move |event: Event| {
            let input: HtmlSelectElement = event.target_unchecked_into();
            let mut next = (*settings).clone();
            next.difficulty = input.value().parse().unwrap();
            settings.set(next);
        })
    };

    let on_background_change = {
        let settings = settings.clone();

        Callback::from(move |event: InputEvent| {
            let input: HtmlInputElement = event.target_unchecked_into();
            let mut next = (*settings).clone();
            next.background_url = Some(input.value());
            settings.set(next);
        })
    };

    let on_persist_change = {
        let settings = settings.clone();

        Callback::from(move |e: Event| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            let mut next = (*settings).clone();
            next.persist_game = input.checked();
            settings.set(next);
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
                        <div class="flex flex-col gap-3 mb-4">
                            <label class="flex flex-col text-sm">
                                {"Difficulty"}
                                <select
                                    class="mt-1 p-2 bg-black border"
                                    onchange={on_difficulty_change}
                                    value={settings.difficulty.to_string()}
                                >
                                    <option value="easy">{"Easy"}</option>
                                    <option value="medium">{"Medium"}</option>
                                    <option value="hard">{"Hard"}</option>
                                </select>
                            </label>

                            <label class="flex flex-col text-sm">
                                {"Background Image URL"}
                                <img src="" alt=""/>
                                <input
                                    type="text"
                                    class="mt-1 p-2 bg-black border"
                                    value={settings.background_url.clone()}
                                    oninput={on_background_change}
                                    placeholder={"Enter background image url"}
                                />
                            </label>

                            <div class="flex items-center gap-2">
                                <input
                                    type="checkbox"
                                    checked={settings.persist_game}
                                    onchange={on_persist_change}
                                />
                                <label class="text-sm">
                                    {"Save game progress"}
                                </label>
                            </div>

                        </div>
                        <button
                            data-action="save"
                            type="button"
                            onclick={&on_action}
                            class="flex gap-1 mt-2 px-4 py-2 border hover:bg-white/10 transition"
                        >
                            {"Save"}
                            <Icon data={IconData::LUCIDE_DISC} width={"20px"}/>
                        </button>
                        <button
                            data-action="close"
                            type="button"
                            onclick={&on_action}
                            class="flex gap-1 mt-2 px-4 py-2 border hover:bg-white/10 transition"
                        >
                            {"Close"}
                            <Icon data={IconData::LUCIDE_CROSS} width={"20px"}/>
                        </button>
                    </div>
                </div>
            }
        </>
    }

}