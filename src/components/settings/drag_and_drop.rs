use log::*;
use web_sys::{File, HtmlInputElement};
use yew::prelude::*;
use yew_icons::{Icon, IconData};

use crate::extensions::ExtensionsNodeRef;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub data_url: Option<Box<str>>,
    pub on_change: Callback<Option<File>>,
}

fn validate(file: &File) -> Result<(), &'static str> {
    if !file.type_().starts_with("image/") {
        return Err("Invalid file type (images only)");
    }
    
    if file.size() > 1024_f64 * 1024_f64 {
        return Err("File too large (max 1 MB)");
    }

    Ok(())
}

#[function_component(DragAndDrop)]
pub fn drag_and_drop(props: &Props) -> Html {
    let input_ref = use_node_ref();
    let hover = use_state(|| false);
    let error = use_state(|| Option::<String>::None);
    info!("props.data_url={:?}", props.data_url);
    let on_drag_over: Callback<DragEvent> = {
        let hover = hover.clone();
        Callback::from(move |event: DragEvent| {
            event.prevent_default();
            hover.set(true);
        })
    };

    let on_drag_leave: Callback<DragEvent> = {
        let hover = hover.clone();
        Callback::from(move |_| {
            hover.set(false);
        })
    };

    let on_drop: Callback<DragEvent> = {
        let hover = hover.clone();
        let on_change = props.on_change.clone();
        let error = error.clone();

        Callback::from(move |event: DragEvent| {
            event.prevent_default();
            hover.set(false);

            if let Some(data_transfer) = event.data_transfer() {
                if let Some(file) = data_transfer.files().and_then(|files| files.get(0)) {
                    handle_file(file, &on_change, &error);
                }
            }
        })
    };

    let on_click: Callback<MouseEvent> = {
        let input_ref = input_ref.clone();
        Callback::from(move |_| {
            let input: HtmlInputElement = input_ref.unchecked_cast();
            input.click();
        })
    };

    let on_input_change: Callback<Event> = {
        let on_change = props.on_change.clone();
        let input_ref = input_ref.clone();
        let error = error.clone();

        Callback::from(move |_| {
            let input: HtmlInputElement = input_ref.unchecked_cast();

            if let Some(file) = input.files().and_then(|files| files.get(0)) {
                handle_file(file, &on_change, &error);
            }
        })
    };

    let on_clear: Callback<MouseEvent> = {
        let on_change = props.on_change.clone();

        Callback::from(move |event: MouseEvent| {
            event.stop_propagation(); // prevent triggering file picker
            on_change.emit(None);
        })
    };

    fn handle_file(
        file: File,
        on_change: &Callback<Option<File>>,
        error: &UseStateHandle<Option<String>>,
    ) {
        match validate(&file) {
            Ok(_) => {
                error.set(None);
                on_change.emit(Some(file));
            }
            Err(msg) => {
                error.set(Some(msg.to_string()));
            }
        }
    }

    let base_drop_classes =
        "relative flex justify-center items-center h-48 w-full border-2 border-dashed p-4 text-center cursor-pointer overflow-hidden";
    let drop_classes = if *hover {
        format!("{base_drop_classes} border-white bg-white/10")
    } else {
        format!("{base_drop_classes} border-gray-500")
    };

    let remove_button = if props.data_url.is_some() {
        html! {
            <div class="absolute top-2 right-2 z-30">
                <button
                    type="button"
                    onclick={on_clear}
                    class="bg-black/60 hover:bg-black/80 hover:opacity-50 text-white p-4 rounded transition"
                >
                    <Icon class="" data={IconData::LUCIDE_TRASH_2} width={"30px"} />
                </button>
            </div>
        }
    } else {
        html! {}
    };

    let image = if let Some(data_url) = &props.data_url {
        html! {
            <img
                src={data_url.to_string()}
                class="absolute inset-0 w-full h-full object-cover opacity-60 pointer-events-none"
            />
        }
    } else {
        html! {}
    };

    let overlay = if props.data_url.is_some() {
        html! {
            <div class="
                absolute inset-0
                flex items-center justify-center
                bg-black/30
                text-white
                text-center
                z-20
                pointer-events-none
            ">
                <span class="text-sm">
                    {"Drag & drop a new image or click to replace"}
                </span>
            </div>
        }
    } else {
        html! {}
    };

    let content = if props.data_url.is_none() && error.is_none() {
        html! {
            <span class="z-10">
                {"Drag & drop an image here, or click to upload"}
            </span>
        }
    } else {
        html! {}
    };

    let error_overlay = if let Some(msg) = (*error).clone() {
        html! {
            <div class="absolute inset-0 flex flex-col items-center justify-center bg-black/60 text-red-400 z-20 p-2 text-center">
                <div class="flex items-center gap-2">
                    <Icon data={IconData::LUCIDE_ALERT_TRIANGLE} width={"16px"} />
                    <span class="text-sm font-medium">{ msg }</span>
                </div>
                <span class="text-xs mt-1 text-gray-300">
                    {"Please try again"}
                </span>
            </div>
        }
    } else {
        html! {}
    };

    html! {
        <div
            class={drop_classes}
            ondragover={on_drag_over}
            ondragleave={on_drag_leave}
            ondrop={on_drop}
            onclick={on_click}
        >
            {image}
            {overlay}
            {remove_button}
            {content}
            {error_overlay}

            <input
                type="file"
                accept="image/*"
                ref={input_ref}
                class="hidden"
                onchange={on_input_change}
            />
        </div>
    }
}