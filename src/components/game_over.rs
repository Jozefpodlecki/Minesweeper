use chrono::{DateTime, Duration, Utc};
use gloo::{storage::{LocalStorage, Storage}, utils::errors::JsError};
use log::info;
use rand::{rng, seq::SliceRandom, RngExt};
use serde::{Deserialize, Serialize};
use web_sys::{window, HtmlElement};
use yew::{virtual_dom::VNode, *};
use yew_router::prelude::Link;
use wasm_bindgen::JsCast; 

use crate::{components::{GameBoard, GameCellComponent, Records}, extensions::{DomStringMapExtensions, MouseEventExtensions}, game::*, models::Social, route::Route};
use yew_icons::{Icon, IconData};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub has_won: bool,
    pub on_play: Callback<MouseEvent>
}

#[function_component(GameOver)]
pub fn game_over(props: &Props) -> Html {

    let Props { has_won, on_play } = props.clone();

    let result = if has_won {
        html! { <span class="text-2xl text-green-500 font-bold">{"You won"}</span> }
    } else {
        html! { <span class="text-2xl text-red-500 font-bold">{"You lost"}</span> }
    };

    html! {
        <div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
            <div class="bg-black/80 p-10 rounded shadow-lg flex flex-col items-center gap-4 w-100">
                <span class="text-4xl">{"Game over!"}</span>
                {result}

                <button
                    type="button"
                    onclick={on_play}
                    class="flex gap-2 items-center border p-2"
                >
                    <span class="text-md">{"Play again"}</span>
                    <Icon data={IconData::LUCIDE_PLAY} width={"20px"}/>
                </button>

                <Records/>
            </div>
        </div>
    }
}