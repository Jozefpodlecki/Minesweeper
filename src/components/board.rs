use yew::prelude::*;

use crate::{components::GameCellComponent, game::GameCell, models::GameEngine};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub engine: GameEngine,
    pub cells: Box<[GameCell]>,
    pub columns: usize,
    pub on_reveal: Callback<MouseEvent>,
    pub on_toggle_flag: Callback<MouseEvent>,
    pub disabled: bool,
}

#[function_component(GameBoard)]
pub fn board(props: &Props) -> Html {
    let Props {
        engine,
        cells,
        columns,
        on_reveal,
        on_toggle_flag,
        disabled,
    } = props.clone();

    match engine {
        GameEngine::Html => {
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
        },
        GameEngine::Canvas => {
            todo!("Canvas rendering");
        },
    }
}