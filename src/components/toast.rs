use std::{cell::RefCell, rc::Rc};

use gloo::timers::callback::Timeout;
use wasm_bindgen::prelude::Closure;
use yew::*;
use yew_icons::{Icon, IconData};

use crate::{extensions::{DomStringMapExtensions, MouseEventExtensions}, services::{ToastManager, ToastState}};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub id: u64,
    pub message: Box<str>,
    pub on_click: Callback<MouseEvent>
}

#[function_component(ToastMessage)]
pub fn toast_message(props: &Props) -> Html {
    html! {
        <div class="bg-gray-800 text-white p-3 rounded shadow flex justify-between items-center">
            <span class="">{ &*props.message }</span>
            <button data-id={props.id.to_string()} type="button" onclick={&props.on_click} class="">
                <span>{"Reset"}</span>
                <Icon data={IconData::LUCIDE_X} width={"20px"}/>
            </button>
        </div>
    }
}

#[function_component(ToastWidget)]
pub fn toast_widget() -> Html {
    let manager = use_context::<ToastManager>().unwrap();
    
    let on_click: Callback<MouseEvent> = {
        let manager = manager.clone();
        Callback::from(move |event: MouseEvent| {
            let dataset = event.target_dataset_unchecked();
            let id: u64 = dataset.parse_unchecked("id");
            manager.remove(id)
        })
    };

    {
        let manager = manager.clone();
        use_effect_with((), move |_| {
            fn tick(manager: ToastManager) {
                let now = js_sys::Date::now() as u64;
                let stale_toasts: Vec<u64> = manager
                    .get_toasts()
                    .iter()
                    .filter(|t| now - t.id > 4000)
                    .map(|t| t.id)
                    .collect();

                for id in stale_toasts {
                    log::info!("rem {id}");
                    manager.remove(id);
                }

                let manager_clone = manager.clone();
                Timeout::new(500, move || tick(manager_clone)).forget();
            }

            tick(manager.clone());

            move || {}
        });
    }

    html! {
        <div class="fixed bottom-4 right-4 flex flex-col gap-2 z-50">
            {
                manager.get_toasts().iter().map(|toast| {
                    html! { <ToastMessage id={toast.id} message={toast.message.clone()} on_click={&on_click} /> }
                }).collect::<Html>()
            }
        </div>
    }
}