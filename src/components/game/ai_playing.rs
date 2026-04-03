use std::{cell::RefCell, rc::Rc};

use log::*;
use wasm_bindgen::{prelude::{Closure, ScopedClosure}, JsCast};
use web_sys::window;
use yew::*;
use yew_icons::{Icon, IconData};

use crate::{components::{GameBoard, Timer}, game::*, models::*, services::{AiAction, AiAigent}};

#[function_component(AiPlaying)]
pub fn ai_playing() -> Html {
    let game_manager = unsafe { use_context::<DefaultGameManager>().unwrap_unchecked() };
    let game_state = use_state(|| game_manager.create_with_difficulty(GameDifficulty::Hard) );
    let ai_agent = use_state(|| Rc::new(AiAigent::new()));

    {
        let game_state = game_state.clone();
        let ai_agent = ai_agent.clone();

        use_effect(
            move || {

                let window = window().unwrap();
                let timeout_callback: Rc<RefCell<Option<ScopedClosure<'static, _>>>> = Rc::new(RefCell::new(None));
                let timeout = match &game_state.phase() {
                    GamePhase::Idle => 250,
                    GamePhase::Initializing { .. } => 500,
                    GamePhase::Playing { .. } => 250,
                    GamePhase::GameOver { .. } => 1500,
                };

                *timeout_callback.borrow_mut() = Some(Closure::wrap(Box::new(move || {
 
                    let action = ai_agent.next(&game_state);
                    debug!("{action}");

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
                }) as Box<dyn FnMut()>));

                let mut timeout_callback_borrow = timeout_callback.borrow_mut();
                let callback = timeout_callback_borrow.take().unwrap();
                let js_function = callback.as_ref().unchecked_ref();

                let handle = window.set_timeout_with_callback_and_timeout_and_arguments_0(js_function, timeout).unwrap();
                callback.forget();

                move || {
                    window.clear_timeout_with_handle(handle);
                }
            });
    }
  

    html! {
        {match game_state.phase() {
            GamePhase::Initializing { grid, mines_count, .. } => {

                let mines_left = mines_count - 0;

                html! {
                    <>
                        <div data-top-panel="" class="flex items-center gap-1 mb-2 px-2">
                            <Icon data={IconData::LUCIDE_BOMB} width={"20px"}/>
                            <span>{format!("Mines: {}", mines_left)}</span>
                        </div>
                        <GameBoard
                            engine={GameEngine::Html}
                            cells={grid.cells().to_vec().into_boxed_slice()}
                            columns={grid.columns}
                            on_reveal={Callback::noop()}
                            on_toggle_flag={Callback::noop()}
                            disabled={true}
                        />
                    </>
                }
            },
            GamePhase::Playing { grid, flags_count, mines_count, started_at, .. } => {

                let mines_left = mines_count - flags_count;

                html! {
                    <>
                        <header data-top-panel="" class="flex items-center gap-1 mb-2 px-2 text-white">
                            <Icon data={IconData::LUCIDE_BOMB} width={"20px"}/>
                            <span>{format!("Mines: {}", mines_left)}</span>
                            <Timer started_at={*started_at} />
                        </header>
                        <GameBoard
                            engine={GameEngine::Html}
                            cells={grid.cells().to_vec().into_boxed_slice()}
                            columns={grid.columns}
                            on_reveal={Callback::noop()}
                            on_toggle_flag={Callback::noop()}
                            disabled={true}
                        />
                    </>
                }
            },
            GamePhase::GameOver { grid, mines_count, flags_count, .. } => {

                let mines_left = mines_count - flags_count;

                html! {
                    <>
                        <header data-top-panel="" class="flex items-center gap-1 mb-2 px-2">

                            <Icon data={IconData::LUCIDE_BOMB} width={"20px"}/>
                            <span>{format!("Mines: {}", mines_left)}</span>
                        </header>
                        <GameBoard
                            engine={GameEngine::Html}
                            cells={grid.cells().to_vec().into_boxed_slice()}
                            columns={grid.columns}
                            on_reveal={Callback::noop()}
                            on_toggle_flag={Callback::noop()}
                            disabled={true}
                        />
                    </>
                }
            }
           _ => html! {}
        }}
    }

}