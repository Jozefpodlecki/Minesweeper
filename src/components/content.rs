use chrono::{DateTime, Duration, Utc};
use gloo::{storage::{LocalStorage, Storage}, utils::errors::JsError};
use rand::{rng, seq::SliceRandom, RngExt};
use serde::{Deserialize, Serialize};
use web_sys::{window, HtmlElement};
use yew::{virtual_dom::VNode, *};
use yew_router::prelude::Link;
use wasm_bindgen::JsCast; 

use crate::{components::GameCellComponent, game::{GameCell, GameState, SavedGameState}, models::Social, route::Route};
use yew_icons::{Icon, IconData};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Record {
    pub created_on: DateTime<Utc>,
    pub duration: u32,
    pub has_won: bool,
    pub revealed_mines: u32,
    pub total_mines: u32
}

pub fn get_last_state() -> Option<SavedGameState> {
    let records = LocalStorage::get::<SavedGameState>("state");

    records.ok()
}

pub fn save_state(value: SavedGameState) -> Result<(), JsError> {
    LocalStorage::set("state", value)
        .map_err(|err| {
            let js_error = js_sys::Error::new(&err.to_string());
            JsError::from(js_error)
        })?;

    Ok(())
}

pub fn get_last_records() -> Vec<Record> {
    let records = LocalStorage::get::<Vec<Record>>("records");

    records.unwrap_or_default()
}

pub fn set_last_record(value: Record) -> Result<(), JsError> {
    let records = LocalStorage::get::<Vec<Record>>("records");
    let mut records = records.unwrap_or_default();

    records.push(value);

    LocalStorage::set("records", records)
        .map_err(|err| {
            let js_error = js_sys::Error::new(&err.to_string());
            JsError::from(js_error)
        })?;

    Ok(())
}


#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub class: String
}

#[function_component(Records)]
pub fn records() -> Html {
    let records = use_state(|| get_last_records() );

    html! {
        <div class="flex flex-col">
            {(*records).clone().into_iter().map(|record: Record|  html! { <div></div> }).collect::<Html>()}
        </div>
    }
}

#[function_component(Content)]
pub fn content(props: &Props) -> Html {
    
    let game_state = use_state(|| GameState::new() );

    let on_play: Callback<MouseEvent> = {
        let game_state = game_state.clone();

        Callback::from(move |_| {
            let new_state = (*game_state).clone();
            game_state.set(new_state.play(Default::default()));
        })
    };

    let on_reveal: Callback<MouseEvent> = {
        let game_state = game_state.clone();

        Callback::from(move |event: MouseEvent| {
            // let new_state = match current_state {
            //     GameState::Playing { cells, .. } => {
            //         let has_mines = cells.iter().any(|c| c.is_mine);
                    
            //         if !has_mines {
            //             current_state.initialize_with_first_click(row, col)
            //         } else {
            //             current_state.reveal_cell(row, col)
            //         }
            //     },
            //     _ => current_state,
            // };
        })
    };

    let on_mark: Callback<MouseEvent> = {

        Callback::from(move |event: MouseEvent| {
            let current_target = event.current_target().unwrap();
            let html_element = current_target.unchecked_into::<HtmlElement>();
            let dataset = html_element.dataset();
        })
    };


    html! {
        <section class={format!("{} flex w-full justify-center items-center", props.class.clone())}>
            {match (*game_state).clone() {
                GameState::Idle => {
                    html! {
                        <main class={"flex justify-center items-center w-300 h-200 bg-black/25"}>
                            <div class="">
                               <button 
                                    type="button" 
                                    onclick={on_play} 
                                    class="flex gap-2 border-white border-2 p-4 mx-auto hover:bg-black/30 hover:scale-105 transition-all duration-200"
                                >
                                    <span class="dark:text-white">{"Play"}</span>
                                    <Icon class="dark:text-white" data={IconData::LUCIDE_PLAY} width={"20px".to_owned()}/>
                                </button>
                                <Records/>
                            </div>
                        </main>
                    }
                },
                GameState::Initializing { cells, cols, .. } | GameState::Playing { cells, cols, .. } => {
                    let grid_style = format!(
                        "display: grid; grid-template-columns: repeat({}, 42px); gap: 2px;", 
                        cols
                    );

                    let rendered_cells = cells.clone().into_iter().map(|cell: GameCell| {
                        html! {
                            <GameCellComponent
                                cell={cell.clone()}
                                on_reveal={on_reveal.clone()}
                                on_mark={on_mark.clone()}
                                disabled={false}
                            />
                        }
                    }).collect::<Html>();

                    html! {
                        <main class="flex justify-center items-center w-300 h-200 bg-black/25">
                            <div style={grid_style}>
                                { rendered_cells }
                            </div>
                            <button type="button" onclick={on_play} class="">
                                <span>{"Reset"}</span>
                            </button>
                        </main>
                    }
                },
                GameState::GameOver { has_won, .. } => {
                    html! {
                        <main class="flex">
                            <span>{"Game over!"}</span>
                            // { if has_won {
                            //     <span>{"You won"}</span>
                            // } else {
                            //     <span>{"You lost"}</span>
                            // } }
                            <button type="button" onclick={on_play} class="">{"Play"}</button>
                            <Records/>
                        </main>
                    }
                },
            }}
        </section>
    }
}