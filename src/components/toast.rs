use std::{cell::RefCell, rc::Rc};

use gloo::timers::callback::Timeout;
use js_sys::Date;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use yew::*;
use yew_icons::{Icon, IconData};

use crate::{extensions::DomStringMapExtensions, services::ToastManager};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub id: u64,
    pub message: Box<str>,
    pub on_click: Callback<MouseEvent>
}

#[function_component(ToastMessage)]
pub fn toast_message(props: &Props) -> Html {
    let opacity = use_state(|| 1.0);
    
   {
        let opacity = opacity.clone();
        use_effect_with((), move |_| {
            
            let timeout_rc = Rc::new(RefCell::new(None));
            let opacity = opacity.clone();
            let start: f64 = Date::now();

            fn tick(start: f64, opacity: UseStateHandle<f64>, timeout: Rc<RefCell<Option<Timeout>>>) {
                
                let timeout_clone = timeout.clone();
                *timeout.borrow_mut() = Some(Timeout::new(16, move || {

                    let elapsed = Date::now() - start;
                    let new_opacity = 1.0 - (elapsed / 2500.0);
                    if new_opacity <= 0.0 {
                        opacity.set(0.0);
                    } else {
                        opacity.set(new_opacity);
                        tick(start, opacity.clone(), timeout_clone);
                    }
                }));
            }

            tick(start, opacity.clone(), timeout_rc.clone());

            move || {
                if let Some(timeout) = timeout_rc.borrow_mut().take() {
                    timeout.cancel();
                }
            }
        });
    }
    
    html! {
        <div 
            class="bg-gray-800 text-white p-3 rounded-lg shadow-md flex justify-between items-center max-w-sm space-x-4 transition-opacity duration-75"
            style={format!("opacity: {}", *opacity)}
        >
            <span class="flex-1 break-words">{ &*props.message }</span>
            <button
                data-action="dismiss"
                data-id={props.id.to_string()} 
                type="button" 
                onclick={&props.on_click} 
                class="flex items-center gap-2 bg-gray-700 hover:bg-gray-600 text-white px-3 py-1 rounded transition-colors duration-150"
            >
                <Icon data={IconData::LUCIDE_X} width="16px" height="16px"/>
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
            let mut element: HtmlElement = event.target_unchecked_into();
            
            if element.tag_name() == "svg" {
                element = unsafe {
                    element.closest("button")
                        .unwrap_unchecked()
                        .map(|element| element.unchecked_into::<HtmlElement>())
                        .unwrap_unchecked()
                };
            }

            let dataset = element.dataset();
            let id: u64 = dataset.parse_unchecked("id");
            manager.remove(id)
        })
    };

    {
        let manager = manager.clone();

        use_effect(move || {
            let manager = manager.clone();
            let timeout_rc = Rc::new(RefCell::new(None));
            
            fn tick(manager: ToastManager, timeout: Rc<RefCell<Option<Timeout>>>) {
                
                manager.remove_expired();

                let manager_clone = manager.clone();
                let timeout_clone = timeout.clone();
                *timeout.borrow_mut() = Some(Timeout::new(500, move || {
                    tick(manager_clone, timeout_clone);
                }));
            }

            tick(manager, timeout_rc.clone());

            move || {
                if let Some(timeout) = timeout_rc.borrow_mut().take() {
                    timeout.cancel();
                }
            }
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