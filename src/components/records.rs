use chrono::{DateTime, Duration, Utc};
use gloo::{storage::{LocalStorage, Storage}, utils::errors::JsError};
use log::info;
use rand::{rng, seq::SliceRandom, RngExt};
use serde::{Deserialize, Serialize};
use web_sys::{window, HtmlElement};
use yew::{virtual_dom::VNode, *};
use yew_router::prelude::Link;
use wasm_bindgen::JsCast;

use crate::{game::Repository, models::Record}; 

#[function_component(Records)]
pub fn records() -> Html {
    let repository = unsafe { use_context::<Repository>().unwrap_unchecked() };
    let records = use_state(|| vec![] );

    {
        let records = records.clone();
        use_effect_with((), move |_| {
            let last_records = repository.get_last_records();
            records.set(last_records);
        });
    }

    html! {
        <div class="flex flex-col">
            {records.iter().map(|record|  html! { <div></div> }).collect::<Html>()}
        </div>
    }
}
