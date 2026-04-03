use std::hint::unreachable_unchecked;

use log::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, HtmlInputElement, HtmlSelectElement};
use yew::*;

use crate::{components::{settings::unsaved_guard::UnsavedGuard, BackgroundSelector, DifficultySelector}, extensions::*, models::BackgroundSource, services::{SettingsManager, ToastManager}};
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
            .map(|element| element.unchecked_into::<HtmlElement>())
    }
}

#[function_component(SettingsWidget)]
pub fn settings() -> Html {
    let is_open = use_state(|| false);
    let toast_manager = unsafe { use_context::<ToastManager>().unwrap_unchecked() };
    let settings_manager = unsafe { use_context::<SettingsManager>().unwrap_unchecked() };
    let prev_settings = use_state(|| settings_manager.get() );
    let settings = use_state(|| settings_manager.get() );
    let has_changes = &prev_settings != &settings;
    let can_save = {

        let is_valid_background = match &settings.background {
            BackgroundSource::Default => true,
            BackgroundSource::FileSystem { data_url, .. } => !data_url.is_empty(),
            BackgroundSource::Url { data_url, .. } => !data_url.is_empty(),
        };

        let result = has_changes && is_valid_background;

        result
    };
   
    let on_action: Callback<MouseEvent> = {
        let is_open = is_open.clone();
        let prev_settings = prev_settings.clone();
        let settings = settings.clone();
        let toast_manager = toast_manager.clone();

        Callback::from(move |event: MouseEvent| {

            let action = resolve_action_target(&event)
                .map(Action::from_str)
                .unwrap_or_default();

            match action {
                Action::Open => is_open.set(true),
                Action::Save => {

                    let settings = (&*settings).clone();
                    settings_manager.save(settings.clone());
                    prev_settings.set(settings);
                    toast_manager.success("Saved settings");
                    is_open.set(false);
                },
                Action::Close => {
                    let prev_settings = (&*prev_settings).clone();
                    settings.set(prev_settings);
                    
                    is_open.set(false);
                },
            }
        })
    };

    let on_difficulty_change = {
        let settings = settings.clone();

        Callback::from(move |event: Event| {
            let mut next = (*settings).clone();

            let input: HtmlSelectElement = event.target_unchecked_into();
            
            let difficulty = input.value().parse().unwrap();
            next.difficulty = difficulty;

            settings.set(next);
        })
    };

    let on_background_change: Callback<BackgroundSource> = {
        let settings = settings.clone();

        Callback::from(move |value: BackgroundSource| {
            let mut next = (*settings).clone();
            next.background = value;
            settings.set(next);
        })
    };

    let on_persist_change = {
        let settings = settings.clone();

        Callback::from(move |event: Event| {
            let input: HtmlInputElement = event.target_unchecked_into();
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
                    data-settings=""
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
                    data-overlay=""
                    data-action="close"
                    class="fixed inset-0 bg-black/50 flex items-center justify-center z-50"
                    onclick={&on_action}
                >
                    if has_changes {
                        <UnsavedGuard/>
                    }
                    <div
                        data-modal=""
                        class="flex flex-col bg-black/70 text-white p-6 rounded shadow-lg min-w-[400px] h-150"
                        onclick={stop_propagation}
                    >
                        <div class="flex justify-center items-center gap-2 mb-4">
                            <h1 class="text-3xl">{"Settings"}</h1>
                            <Icon data={IconData::LUCIDE_SETTINGS} width={"30px"} />
                        </div>
                        <div class="flex-1 flex flex-col gap-3 mb-4">
                            <DifficultySelector value={settings.difficulty} on_change={on_difficulty_change} />
                            <BackgroundSelector previous={prev_settings.background.clone()} value={settings.background.clone()} on_change={on_background_change} />

                            <div class="flex items-center gap-3 p-2 rounded hover:bg-white/5 transition cursor-pointer">
                                <input
                                    id="persist-game"
                                    type="checkbox"
                                    checked={settings.persist_game}
                                    onchange={on_persist_change}
                                    class="w-4 h-4 accent-white cursor-pointer"
                                />
                                <label for="persist-game" class="text-sm text-gray-200 cursor-pointer select-none">
                                    {"Save game progress"}
                                </label>
                            </div>

                        </div>
                        <footer class="flex gap-2 p-2">
                            <button
                                disabled={!can_save}
                                data-action="save"
                                type="button"
                                onclick={&on_action}
                                class="w-30 flex justify-center items-center gap-1 mt-2 px-4 py-2 border enabled:hover:bg-white/10 disabled:opacity-50 transition"
                            >
                                {"Save"}
                                <Icon data={IconData::LUCIDE_HARD_DRIVE} width={"15px"}/>
                            </button>
                            <button
                                data-action="close"
                                type="button"
                                onclick={&on_action}
                                class="w-30 flex justify-center items-center gap-1 mt-2 px-4 py-2 border hover:bg-white/10 transition"
                            >
                                {"Close"}
                                <Icon data={IconData::LUCIDE_X} width={"15px"}/>
                            </button>
                        </footer>
                    </div>
                </div>
            }
        </>
    }

}