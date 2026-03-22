use chrono::{DateTime, Duration, Utc};
use gloo::{storage::{LocalStorage, Storage}, utils::errors::JsError};
use log::info;
use rand::{rng, seq::SliceRandom, RngExt};
use serde::{Deserialize, Serialize};
use web_sys::{window, HtmlElement};
use yew::{virtual_dom::VNode, *};
use yew_router::prelude::Link;
use wasm_bindgen::JsCast; 

use crate::{components::{AiPlaying, GameBoard, GameCellComponent, GameOver, Records}, extensions::{DomStringMapExtensions, MouseEventExtensions}, game::*, models::Social, route::Route};
use yew_icons::{Icon, IconData};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub class: String
}

#[function_component(Content)]
pub fn content(props: &Props) -> Html {
    
    let game_state = use_state(|| GameState::default() );
    // let game_state = use_state(|| GameState::default().play(Default::default()) );
    // let game_state = use_state(|| GameState::game_over(true) );

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
            let next_state = {
                (*game_state).clone().reveal(row, column)
            };

            match &next_state.phase() {
                GamePhase::GameOver { .. } => {
                    info!("over1");
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

    {
        let game_state = game_state.clone();
        use_effect_with(
            game_state,
            move |game_state| {
                match game_state.phase() {
                    GamePhase::Playing { .. } => {
                        info!("playing");
                    },
                    GamePhase::GameOver { .. } => {
                        info!("over");
                    },
                    _ => {}
                } 
            },
        );
    }

    html! {
        <section class={format!("{} flex w-full justify-center items-center", props.class)}>
            {match game_state.phase() {
                GamePhase::Idle => {
                    html! {
                        <main class={"flex flex-col justify-center items-center h-200"}>
                            <AiPlaying/>
                            <div class="fixed inset-0 bg-black/80 flex items-center justify-center z-50">
                               <button 
                                    type="button" 
                                    onclick={&on_play} 
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
                GamePhase::Initializing { cells, columns, .. } => {
                    html! {
                        <main class="flex flex-col text-white h-200">
                            <div class="flex items-center gap-1 mb-2 px-2">
                                <Icon data={IconData::LUCIDE_BOMB} width={"20px"}/>
                                <span>{format!("Mines: {}", 0)}</span>
                                // <span>{format!("Time: {}s", seconds)}</span>
                            </div>
                            <GameBoard
                                cells={cells.clone()}
                                columns={*columns}
                                on_reveal={&on_reveal}
                                on_toggle_flag={&on_toggle_flag}
                                disabled={false}
                            />
                            <div class="flex mt-4">
                                <button disabled={true} type="button" onclick={&on_play} class="flex p-2 border gap-2">
                                    <span>{"Reset"}</span>
                                    <Icon data={IconData::LUCIDE_TIMER_RESET} width={"20px"}/>
                                </button>
                            </div>
                        </main>
                    }
                },
                GamePhase::Playing { cells, columns, flags_count, mines_count, .. } => {
 
                    let mines_left = mines_count - flags_count;

                    html! {
                        <main class="flex flex-col text-white h-200">
                            <div class="flex items-center gap-1 mb-2 px-2">
                                <Icon data={IconData::LUCIDE_BOMB} width={"20px"}/>
                                <span>{format!("Mines: {}", mines_left)}</span>
                                // <span>{format!("Time: {}s", seconds)}</span>
                            </div>
                            <GameBoard
                                cells={cells.clone()}
                                columns={*columns}
                                on_reveal={&on_reveal}
                                on_toggle_flag={&on_toggle_flag}
                                disabled={false}
                            />
                            <div class="flex mt-4">
                                <button disabled={true} type="button" onclick={&on_play} class="flex p-2 border gap-2">
                                    <span>{"Reset"}</span>
                                    <Icon data={IconData::LUCIDE_TIMER_RESET} width={"20px"}/>
                                </button>
                            </div>
                        </main>
                    }
                },
                GamePhase::GameOver { has_won, cells, columns, mines_count, flags_count, .. } => {

                    let mines_left = mines_count - flags_count;

                    html! {
                        <main class="flex flex-col text-white h-200">
                            <div class="flex items-center gap-1 mb-2 px-2">
                                <Icon data={IconData::LUCIDE_BOMB} width={"20px"}/>
                                <span>{format!("Mines: {}", mines_left)}</span>
                                // <span>{format!("Time: {}s", seconds)}</span>
                            </div>
                            <GameBoard
                                cells={cells.clone()}
                                columns={*columns}
                                on_reveal={&on_reveal}
                                on_toggle_flag={&on_toggle_flag}
                                disabled={true}
                            />
                            <div class="flex mt-4">
                                <button disabled={true} type="button" onclick={&on_play} class="flex text-white p-2 border gap-2">
                                    <span>{"Reset"}</span>
                                    <Icon data={IconData::LUCIDE_TIMER_RESET} width={"20px"}/>
                                </button>
                            </div>
                            <GameOver has_won={has_won} on_play={&on_play} />
                        </main>
                    }
                },
            }}
        </section>
    }
}