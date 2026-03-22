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
    pub class: String
}

#[function_component(Content)]
pub fn content(props: &Props) -> Html {
    
    let game_state = use_state(|| GameState::game_over(true) );

    let on_play: Callback<MouseEvent> = {
        let game_state = game_state.clone();

        Callback::from(move |_| {
            let new_state = game_state.play(Default::default());
            game_state.set(new_state);
        })
    };

    let on_reveal: Callback<MouseEvent> = {
        let game_state = game_state.clone();

        Callback::from(move |event: MouseEvent| {
            let dataset = event.target_dataset_unchecked();
            let row = dataset.parse_unchecked("row");
            let column = dataset.parse_unchecked("column");
            let new_state = (*game_state).clone();

            let next_state = match &new_state {
                GameState::Initializing { .. } => {
                    let initialized = new_state.initialize_with_first_click(row, column);
                    initialized.reveal(row, column)
                }
                _ => new_state.reveal(row, column),
            };

            match &next_state {
                GameState::GameOver { has_won, cells, rows, cols, duration, started_at, mines_count, revealed_count } => {
                    info!("over");
                },
                _ => {}
            }

            game_state.set(next_state);
        })
    };

    let on_toggle_flag: Callback<MouseEvent> = {
        let game_state = game_state.clone();

        Callback::from(move |event: MouseEvent| {
            let dataset = event.target_dataset_unchecked();
                let row = dataset.parse_unchecked("row");
                let column = dataset.parse_unchecked("column");
                let new_state = game_state.toggle_flag(row, column);

                game_state.set(new_state);
        })
    };


    html! {
        <section class={format!("{} flex w-full justify-center items-center", props.class)}>
            {match &*game_state {
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
                                    <Icon class="dark:text-white" data={IconData::LUCIDE_PLAY} width={"20px"}/>
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
                                cell={cell}
                                on_reveal={&on_reveal}
                                on_toggle_flag={&on_toggle_flag}
                                disabled={false}
                            />
                        }
                    }).collect::<Html>();

                    html! {
                        <main class="flex justify-center items-center w-300 h-200 bg-black/25">
                            <div style={grid_style}>
                                { rendered_cells }
                            </div>
                            <button  type="button" onclick={on_play} class="flex">
                                <span>{"Reset"}</span>
                                <Icon data={IconData::LUCIDE_TIMER_RESET} width={"20px"}/>
                            </button>
                        </main>
                    }
                },
                GameState::GameOver { has_won, cells, cols, .. } => {

                    let grid_style = format!(
                        "display: grid; grid-template-columns: repeat({}, 42px); gap: 2px;", 
                        cols
                    );

                    let rendered_cells = cells.clone().into_iter().map(|cell: GameCell| {
                        html! {
                            <GameCellComponent
                                cell={cell}
                                on_reveal={&on_reveal}
                                on_toggle_flag={&on_toggle_flag}
                                disabled={true}
                            />
                        }
                    }).collect::<Html>();

                    let result = if *has_won {
                        html! { <span>{"You won"}</span> }
                    } else {
                        html! { <span>{"You lost"}</span> }
                    };

                    html! {
                        <main class="flex flex-col text-white">
                            <div style={grid_style}>
                                { rendered_cells }
                            </div>
                            <div class="absolute bg-black/50 p-10">
                                <span class="">{"Game over!"}</span>
                                {result}
                                <button type="button" onclick={on_play} class="flex gap-2 items-center border p-2">
                                    <span>{"Play again"}</span>
                                    <Icon data={IconData::LUCIDE_PLAY} width={"20px"}/>
                                </button>
                                <Records/>
                            </div>
                        </main>
                    }
                },
            }}
        </section>
    }
}