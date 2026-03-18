use chrono::{DateTime, Utc};
use gloo::{storage::{LocalStorage, Storage}, utils::errors::JsError};
use rand::RngExt;
use serde::{Deserialize, Serialize};
use web_sys::window;
use yew::{virtual_dom::VNode, *};
use yew_router::prelude::Link;

use crate::{route::Route, models::Social};
use yew_icons::{Icon, IconData};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Record {
    pub created_on: DateTime<Utc>,
    pub duration: u32,
    pub has_won: bool,
    pub revealed_mines: u32,
    pub total_mines: u32
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

#[derive(Debug, Clone, PartialEq)]
struct Cell {
    row_id: usize,
    column_id: usize,
    is_mine: bool,
    is_revealed: bool,
    is_flagged: bool,
    neighbor_mines: u8,
}

struct Grid(Vec<Vec<Cell>>);

impl Cell {
    fn new(row_id: usize, column_id: usize, is_mine: bool) -> Self {
        Self {
            row_id,
            column_id,
            is_mine,
            is_revealed: false,
            is_flagged: false,
            neighbor_mines: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum GameState {
    Idle,
    Playing {
        cells: Vec<Cell>,
        rows: usize,
        cols: usize,
        mines_count: usize,
    },
    GameOver {
        has_won: bool,
    }
    // 
}

impl GameState {
    pub fn new() -> Self {
        Self::Idle
    }

    pub fn play(self) -> Self {

        Self::Playing {
            cells: vec![],
            rows: 10,
            cols: 10,
            mines_count: 1
        }
    }
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
    // let random_number = rand::rng().random_range(1..=100);

    let on_play: Callback<MouseEvent> = {
        let game_state = game_state.clone();

        Callback::from(move |_| {
            let new_state = (*game_state).clone();
            game_state.set(new_state.play());
        })
    };

    let on_reveal: Callback<MouseEvent> = {

        Callback::from(move |_| {
            
        })
    };

    let on_mark: Callback<MouseEvent> = {

        Callback::from(move |_| {
            

        })
    };


    html! {
        <section class={format!("{} flex w-full justify-center items-center", props.class.clone())}>
            {match (*game_state).clone() {
                GameState::Idle => {
                    html! {
                        <main class={"flex justify-center items-center w-300 h-200 bg-black/25"}>
                            <div class="">
                                <button type="button" onclick={on_play} class="flex gap-2 border-white border-2 p-4 mx-auto">
                                    <span class="dark:text-white">{"Play"}</span>
                                    <Icon class="dark:text-white" data={IconData::LUCIDE_PLAY} width={"20px".to_owned()}/>
                                </button>
                                <Records/>
                            </div>
                        </main>
                    }
                },
                GameState::Playing { .. } => {
                    html! {
                        <main class="flex">
                            
                        </main>
                    }
                },
                GameState::GameOver { has_won } => {
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