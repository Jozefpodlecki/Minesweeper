use std::{cell::RefCell, rc::Rc};

use js_sys::Function;
use log::info;
use rand::seq::IndexedRandom;
use wasm_bindgen::{prelude::{Closure, ScopedClosure}, JsCast};
use web_sys::window;
use yew::*;
use yew_icons::{Icon, IconData};
use yew_router::prelude::Link;

use crate::{components::{GameBoard, Timer}, game::{CellState, GameCell, GamePhase, GameSettings, GameState}, route::Route, services::{AiAction, AiAigent, SystemClock}};

#[function_component(AiPlaying)]
pub fn ai_playing() -> Html {
    let game_state = use_state(|| {
        let state = GameState::default().play(GameSettings::hard());
        state
    } );
    let runner = use_state(|| Rc::new(RefCell::new(AiAigent::new())));

    {
        let game_state = game_state.clone();
        let runner = runner.clone();

        use_effect(
            move || {

                let window = window().unwrap();
                let timeout_callback: Rc<RefCell<Option<ScopedClosure<'static, _>>>> = Rc::new(RefCell::new(None));

                *timeout_callback.borrow_mut() = Some(Closure::wrap(Box::new(move || {
 
                    let action = runner.borrow_mut().next(&game_state);

                    match action {
                        AiAction::None => return,
                        AiAction::Reveal { col, row } => {
                            let next_state = (&*game_state).clone();
                            let next_state = next_state.reveal(row, col);
                            game_state.set(next_state);
                            
                        },
                        AiAction::Flag { col, row } => {
                            let next_state = game_state.toggle_flag(row, col);
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
                let next_handle = window.set_timeout_with_callback_and_timeout_and_arguments_0(callback.as_ref().unchecked_ref(), 250).unwrap();
                callback.forget();
            });
    }
  

    html! {
        {match game_state.phase() {
            GamePhase::Initializing { cells, columns, mines_count, .. } => {

                let mines_left = mines_count - 0;

                html! {
                    <>
                        <div class="flex items-center gap-1 mb-2 px-2">
                            <Icon data={IconData::LUCIDE_BOMB} width={"20px"}/>
                            <span>{format!("Mines: {}", mines_left)}</span>
                        </div>
                        <GameBoard
                            cells={cells.clone()}
                            columns={*columns}
                            on_reveal={Callback::noop()}
                            on_toggle_flag={Callback::noop()}
                            disabled={true}
                        />
                    </>
                }
            },
            GamePhase::Playing { cells, columns, flags_count, mines_count, started_at, .. } => {

                let mines_left = mines_count - flags_count;

                html! {
                    <>
                        <div class="flex items-center gap-1 mb-2 px-2 text-white">
                            <Icon data={IconData::LUCIDE_BOMB} width={"20px"}/>
                            <span>{format!("Mines: {}", mines_left)}</span>
                            <Timer started_at={*started_at} />
                        </div>
                        <GameBoard
                            cells={cells.clone()}
                            columns={*columns}
                            on_reveal={Callback::noop()}
                            on_toggle_flag={Callback::noop()}
                            disabled={true}
                        />
                    </>
                }
            },
            GamePhase::GameOver { cells, columns, mines_count, flags_count, .. } => {

                let mines_left = mines_count - flags_count;

                html! {
                    <>
                        <div class="flex items-center gap-1 mb-2 px-2">
                            <Icon data={IconData::LUCIDE_BOMB} width={"20px"}/>
                            <span>{format!("Mines: {}", mines_left)}</span>
                        </div>
                        <GameBoard
                            cells={cells.clone()}
                            columns={*columns}
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