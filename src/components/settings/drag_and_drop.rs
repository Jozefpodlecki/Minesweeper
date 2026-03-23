use web_sys::{File, HtmlInputElement};
use yew::prelude::*;

use crate::extensions::ExtensionsNodeRef;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_change: Callback<File>,
}

#[function_component(DragAndDrop)]
pub fn drag_and_drop(props: &Props) -> Html {
    let input_ref = use_node_ref();
    let hover = use_state(|| false);

    let on_drag_over: Callback<DragEvent> = {
        let hover = hover.clone();
        Callback::from(move |event: DragEvent| {
            event.prevent_default();
            hover.set(true);
        })
    };

    let on_drag_leave: Callback<_> = {
        let hover = hover.clone();
        Callback::from(move |_| {
            hover.set(false);
        })
    };

    let on_drop: Callback<DragEvent> = {
        let hover = hover.clone();
        let on_change = props.on_change.clone();
        Callback::from(move |event: DragEvent| {
            event.prevent_default();
            hover.set(false);

            if let Some(data_transfer) = event.data_transfer() {
                if let Some(file) = data_transfer.files().and_then(|files| files.get(0)) {
                    // optional: check type

                    let file_size = file.size();

                    if file.type_().starts_with("image/") {
                        on_change.emit(file);
                    }
                }
            }
        })
    };

    let on_click: Callback<MouseEvent> = {
        let input_ref = input_ref.clone();
        Callback::from(move |_| {
            if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                input.click();
            }
        })
    };

    let on_input_change: Callback<Event> = {
        let on_change = props.on_change.clone();
        let input_ref = input_ref.clone();

        Callback::from(move |_: Event| {
            let input = input_ref.unchecked_cast::<HtmlInputElement>();

            
            if let Some(file) = input.files().and_then(|files| files.get(0)) {
                if file.type_().starts_with("image/") {
                    on_change.emit(file);
                }
            }
        
        })
    };

    let drop_classes = if *hover {
        "border-2 border-dashed border-white p-4 text-center cursor-pointer bg-white/10"
    } else {
        "border-2 border-dashed border-gray-500 p-4 text-center cursor-pointer"
    };

    html! {
        <div
            class={drop_classes}
            ondragover={on_drag_over}
            ondragleave={on_drag_leave}
            ondrop={on_drop}
            onclick={on_click}
        >
            {"Drag & drop an image here, or click to upload"}
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