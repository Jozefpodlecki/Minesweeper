use std::rc::Rc;

use log::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use yew::prelude::*;
use crate::{components::{AiPlaying, GameBoard, GameOver, Records}, extensions::{DomStringMapExtensions, MouseEventExtensions}, game::*, models::GameDifficulty, services::{AiAction, AiAigent, SettingsManager}};
use yew_icons::{Icon, IconData};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub class: String
}

#[function_component(Content)]
pub fn content(props: &Props) -> Html {
    
    let settings_manager = unsafe { use_context::<SettingsManager>().unwrap_unchecked() };
    let repository = unsafe { use_context::<Repository>().unwrap_unchecked() };
    let game_manager = unsafe { use_context::<DefaultGameManager>().unwrap_unchecked() };
    let settings = use_state(|| settings_manager.get() );
    let game_state = use_state(|| game_manager.create() );
    let is_reviewing = use_state(|| false );
    let ai_agent = use_state(|| Rc::new(AiAigent::new()));
    
    let on_action: Callback<MouseEvent> = {
        let game_state = game_state.clone();
        let settings_manager = settings_manager.clone();

        Callback::from(move |event: MouseEvent| {
            
            let mut element: HtmlElement = event.target_unchecked_into();
            
            if element.tag_name() != "button" {
                element = unsafe {
                    element.closest("button")
                        .unwrap_unchecked()
                        .map(|element| element.unchecked_into::<HtmlElement>())
                        .unwrap_unchecked()
                };
            }

            let dataset = element.dataset();
            let action: String = dataset.parse_unchecked("action");

            match action.as_str() {
               "test" => {},
               _ => {}
            }
        })
    };

    let on_review: Callback<MouseEvent> = {
        let is_reviewing = is_reviewing.clone();

        Callback::from(move |_| {
            is_reviewing.set(true);
        })
    };

    let on_ai_move: Callback<MouseEvent> = {
        let game_state = game_state.clone();
        let ai_agent = ai_agent.clone();

        Callback::from(move |_| {
            let state = &*game_state;
            let action = ai_agent.next(state);

            match action {
                AiAction::None => return,
                AiAction::Reveal { column, row, .. } => {
                    let next_state = (&*game_state).clone();
                    let next_state = next_state.reveal(row, column);
                    game_state.set(next_state);
                    
                },
                AiAction::Flag { column, row, .. } => {
                    let next_state = game_state.toggle_flag(row, column);
                    game_state.set(next_state);
                },
                AiAction::Restart => {
                    let next_state = game_state.restart();
                    game_state.set(next_state);
                }
            }
        })
    };

    let on_play: Callback<MouseEvent> = {
        let game_state = game_state.clone();
        let settings_manager = settings_manager.clone();
        let repository = repository.clone();

        Callback::from(move |_| {
            repository.clear_game_session();
            let new_state = game_state.play();
            game_state.set(new_state);
        })
    };

    let on_reveal: Callback<MouseEvent> = {
        let game_state = game_state.clone();

        Callback::from(move |event: MouseEvent| {
            let mut element: HtmlElement = event.target_unchecked_into();
            
            if element.tag_name() != "button" {
                element = unsafe {
                    element.closest("button")
                        .unwrap_unchecked()
                        .map(|element| element.unchecked_into::<HtmlElement>())
                        .unwrap_unchecked()
                };
            }

            let dataset = element.dataset();

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
            event.prevent_default();
            let mut element: HtmlElement = event.target_unchecked_into();

            if element.tag_name() != "button" {
                element = unsafe {
                    element.closest("button")
                        .unwrap_unchecked()
                        .map(|element| element.unchecked_into::<HtmlElement>())
                        .unwrap_unchecked()
                };
            }

            let dataset = element.dataset();
            let row = dataset.parse_unchecked("row");
            let column = dataset.parse_unchecked("column");
            let new_state = game_state.toggle_flag(row, column);

            game_state.set(new_state);
        })
    };

    {
        let settings_manager = settings_manager.clone();
        let settings_state = settings.clone();
        let game_state = game_state.clone();
        let repository = repository.clone();

        use_effect_with(
            settings_manager,
            move |settings_manager| {
                let new_settings = settings_manager.get();
                let old_settings = &*settings_state;
                let difficulty_change = new_settings.difficulty != old_settings.difficulty;

                if difficulty_change {
                    settings_state.set(new_settings);
                }

                match game_state.phase() {
                    GamePhase::Playing { .. } => {
                        if difficulty_change {
                            repository.clear_game_session();
                            let state = game_state.restart();
                            game_state.set(state);
                        }
                    },
                    _ => {}
                } 
            },
        );
    }

    {
        let game_state = game_state.clone();
        let repository = repository.clone();

        use_effect_with(
            game_state,
            move |game_state| {
                match game_state.phase() {
                    GamePhase::Playing { .. } => {
                        let value = game_state.to_state();
                        repository.save_game_session(value);
                    },
                    GamePhase::GameOver { .. } => {
                        repository.clear_game_session();
                    },
                    _ => {}
                } 
            },
        );
    }

    if *is_reviewing {
        let content =  match game_state.phase() {
            GamePhase::GameOver {
                has_won, grid, last_cell, duration,
                started_at, mines_count, revealed_count, flags_count } => {
                html! {

                }
            },
            _ => unreachable!("Should not reach this state")
        };

        return html! {
            <section class={format!("{} flex w-full justify-center items-center", props.class)}>
                {content}
            </section>
        }
    }

    let content = match game_state.phase() {
        GamePhase::Idle => {
            html! {
                <main class={"flex flex-col justify-center items-center h-200"}>
                    <AiPlaying/>
                    <div data-overlay="" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
                        <div class="flex flex-col">
                            <Records/>
                            <button
                                data-action="play"
                                type="button" 
                                onclick={&on_play} 
                                class="flex justify-center gap-2 w-30 border-white border-2 p-2 mx-auto hover:bg-black/30 hover:scale-105 transition-all duration-200 dark:text-white mt-2"
                            >
                                <span class="">{"Play"}</span>
                                <Icon class="" data={IconData::LUCIDE_PLAY} width={"20px"}/>
                            </button>
                        </div>
                        
                    </div>
                </main>
            }
        },
        GamePhase::Initializing { grid, .. } => {
            html! {
                <main class="flex flex-col text-white h-200">
                    <header data-top-panel="" class="flex items-center gap-1 mb-2 px-2">
                        <Icon data={IconData::LUCIDE_BOMB} width={"20px"}/>
                        <span>{format!("Mines: {}", 0)}</span>
                        // <span>{format!("Time: {}s", seconds)}</span>
                    </header>
                    <GameBoard
                        engine={settings.engine}
                        cells={grid.cells().to_vec().into_boxed_slice()}
                        columns={grid.columns}
                        on_reveal={&on_reveal}
                        on_toggle_flag={&on_toggle_flag}
                        disabled={false}
                    />
                    <footer class="flex mt-4">
                        <button data-action="reset"
                            disabled={true}
                            type="button"
                            onclick={&on_play}
                            class="flex p-2 border gap-2 disabled:opacity-50 disabled:cursor-not-allowed hover:bg-gray-100">
                            <span>{"Reset"}</span>
                            <Icon data={IconData::LUCIDE_TIMER_RESET} width={"20px"}/>
                        </button>
                    </footer>
                </main>
            }
        },
        GamePhase::Playing { grid, flags_count, mines_count, .. } => {

            let mines_left = mines_count - flags_count;

            html! {
                <main class="flex flex-col text-white h-200">
                    <header data-top-panel="" class="flex items-center gap-1 mb-2 px-2">
                        <Icon data={IconData::LUCIDE_BOMB} width={"20px"}/>
                        <span>{format!("Mines: {}", mines_left)}</span>
                        // <span>{format!("Time: {}s", seconds)}</span>
                    </header>
                    <GameBoard
                        engine={settings.engine}
                        cells={grid.cells().to_vec().into_boxed_slice()}
                        columns={grid.columns}
                        on_reveal={&on_reveal}
                        on_toggle_flag={&on_toggle_flag}
                        disabled={false}
                    />
                    <footer class="flex gap-4 mt-4">
                        <button
                            data-action="reset"
                            disabled={false}
                            type="button"
                            onclick={&on_play}
                            class="flex p-2 border gap-2 hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors duration-200">
                            <span>{"Reset"}</span>
                            <Icon data={IconData::LUCIDE_TIMER_RESET} width={"20px"}/>
                        </button><button
                            data-action="ai"
                            disabled={false}
                            type="button"
                            onclick={&on_ai_move}
                            class="flex p-2 border gap-2 hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors duration-200">
                            <span>{"Ai Move"}</span>
                            <Icon data={IconData::LUCIDE_BOT} width={"20px"}/>
                        </button>
                    </footer>
                </main>
            }
        },
        GamePhase::GameOver { has_won, grid, mines_count, flags_count, .. } => {

            let mines_left = mines_count - flags_count;

            html! {
                <main class="flex flex-col text-white h-200">
                    <header data-top-panel="" class="flex items-center gap-1 mb-2 px-2">
                        <Icon data={IconData::LUCIDE_BOMB} width={"20px"}/>
                        <span>{format!("Mines: {}", mines_left)}</span>
                        // <span>{format!("Time: {}s", seconds)}</span>
                    </header>
                    <GameBoard
                        engine={settings.engine}
                        cells={grid.cells().to_vec().into_boxed_slice()}
                        columns={grid.columns}
                        on_reveal={&on_reveal}
                        on_toggle_flag={&on_toggle_flag}
                        disabled={true}
                    />
                    <footer data-footer="" class="flex mt-4">
                        <button disabled={true} type="button" onclick={&on_play} class="flex text-white p-2 border gap-2">
                            <span>{"Reset"}</span>
                            <Icon data={IconData::LUCIDE_TIMER_RESET} width={"20px"}/>
                        </button>
                    </footer>
                    <GameOver has_won={has_won} on_play={&on_play} on_review={on_review} />
                </main>
            }
        },
    };

    html! {
        <section class={format!("{} flex w-full justify-center items-center", props.class)}>
            {content}
        </section>
    }
}