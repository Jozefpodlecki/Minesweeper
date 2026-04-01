use log::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use yew::prelude::*;
use crate::{components::{AiPlaying, GameBoard, GameOver, Records}, extensions::{DomStringMapExtensions, MouseEventExtensions}, game::*, models::GameDifficulty, services::SettingsManager};
use yew_icons::{Icon, IconData};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub class: String
}

#[function_component(Content)]
pub fn content(props: &Props) -> Html {
    
    let settings_manager = unsafe { use_context::<SettingsManager>().unwrap_unchecked() };
    let repository = unsafe { use_context::<Repository>().unwrap_unchecked() };
    // let game_state = use_state(|| GameState::default() );
    let game_state = use_state(|| GameState::default().play(GameSettings::from_difficulty(15, 15, GameDifficulty::Hard)) );
    // let game_state = use_state(|| GameState::game_over(true) );
    // let game_state = use_state(|| GameState::game_over(false) );
    let settings = use_state(|| settings_manager.get() );

    let on_play: Callback<MouseEvent> = {
        let game_state = game_state.clone();
        let settings_manager = settings_manager.clone();

        Callback::from(move |_| {
            let settings = settings_manager.get();
            let new_state = game_state.play(GameSettings::from_difficulty(15, 15, settings.difficulty));
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
            log::info!("{}", element.tag_name());
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
        let game_state = game_state.clone();
        use_effect_with(
            game_state,
            move |game_state| {
                match game_state.phase() {
                    GamePhase::Playing { .. } => {
                        // info!("playing");
                        let value = game_state.to_state();
                        repository.save_game_session(value);
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
                GamePhase::Initializing { cells, columns, .. } => {
                    html! {
                        <main class="flex flex-col text-white h-200">
                            <header data-top-panel="" class="flex items-center gap-1 mb-2 px-2">
                                <Icon data={IconData::LUCIDE_BOMB} width={"20px"}/>
                                <span>{format!("Mines: {}", 0)}</span>
                                // <span>{format!("Time: {}s", seconds)}</span>
                            </header>
                            <GameBoard
                                engine={settings.engine}
                                cells={cells.clone()}
                                columns={*columns}
                                on_reveal={&on_reveal}
                                on_toggle_flag={&on_toggle_flag}
                                disabled={false}
                            />
                            <footer class="flex mt-4">
                                <button data-action="reset" disabled={true} type="button" onclick={&on_play} class="flex p-2 border gap-2">
                                    <span>{"Reset"}</span>
                                    <Icon data={IconData::LUCIDE_TIMER_RESET} width={"20px"}/>
                                </button>
                            </footer>
                        </main>
                    }
                },
                GamePhase::Playing { cells, columns, flags_count, mines_count, .. } => {
 
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
                                cells={cells.clone()}
                                columns={*columns}
                                on_reveal={&on_reveal}
                                on_toggle_flag={&on_toggle_flag}
                                disabled={false}
                            />
                            <footer class="flex mt-4">
                                <button
                                    data-action="reset"
                                    disabled={false}
                                    type="button"
                                    onclick={&on_play}
                                    class="flex p-2 border gap-2 hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors duration-200">
                                    <span>{"Reset"}</span>
                                    <Icon data={IconData::LUCIDE_TIMER_RESET} width={"20px"}/>
                                </button>
                            </footer>
                        </main>
                    }
                },
                GamePhase::GameOver { has_won, cells, columns, mines_count, flags_count, .. } => {

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
                                cells={cells.clone()}
                                columns={*columns}
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
                            <GameOver has_won={has_won} on_play={&on_play} />
                        </main>
                    }
                },
            }}
        </section>
    }
}