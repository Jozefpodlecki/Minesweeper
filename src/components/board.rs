use chrono::{DateTime, Duration, Utc};
use gloo::{storage::{LocalStorage, Storage}, utils::errors::JsError};
use log::info;
use rand::{rng, seq::SliceRandom, RngExt};
use serde::{Deserialize, Serialize};
use web_sys::{window, HtmlElement};
use yew::{virtual_dom::VNode, *};
use yew_router::prelude::Link;
use wasm_bindgen::JsCast; 

use crate::{components::{GameCellComponent, Records}, extensions::{DomStringMapExtensions, MouseEventExtensions}, game::{GameCell, GameState, Repository, SavedGameState}, models::Social, route::Route};
use yew_icons::{Icon, IconData};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub cells: Box<[GameCell]>,
    pub columns: usize,
    pub on_reveal: Callback<MouseEvent>,
    pub on_toggle_flag: Callback<MouseEvent>,
    pub disabled: bool,
}

#[function_component(GameBoard)]
pub fn board(props: &Props) -> Html {
    let Props {
        cells,
        columns,
        on_reveal,
        on_toggle_flag,
        disabled,
    } = props.clone();

    let grid_style = format!(
        "grid-template-columns: repeat({}, 42px); gap: 2px;",
        columns
    );

    let rendered_cells = cells.into_iter().map(|cell| {
        html! {
            <GameCellComponent
                cell={cell}
                on_reveal={on_reveal.clone()}
                on_toggle_flag={on_toggle_flag.clone()}
                disabled={disabled}
            />
        }
    }).collect::<Html>();

    html! {
        <main data-board="" style={grid_style} class="grid">
            { rendered_cells }
        </main>
    }
}