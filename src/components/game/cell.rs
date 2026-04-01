use yew::prelude::*;
use yew_router::prelude::Link;

use crate::{game::{CellState, GameCell}, models::Social, route::Route};
use yew_icons::{Icon, IconData};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub cell: GameCell,
    pub on_reveal: Callback<MouseEvent>,
    pub on_toggle_flag: Callback<MouseEvent>,
    pub disabled: bool,
}

#[function_component(GameCellComponent)]
pub fn game_cell(props: &Props) -> Html {
    let Props {
        cell,
        disabled,
        on_reveal,
        on_toggle_flag
    } = props;

    let debug_mode = true;
    let base_class = "w-10 h-10 flex items-center justify-center font-bold transition-colors";

    let (state, classes, content, click_handler, context_handler) = match cell.state {
        CellState::Revealed => {
            let content = if cell.is_mine {
                html! { <Icon data={IconData::LUCIDE_BOMB} width={"20px"} /> }
            } else if cell.neighbor_mines > 0 {
                html! { cell.neighbor_mines.to_string() }
            } else {
                html! {}
            };

            let bg_class = if cell.is_mine {
                "bg-red-500/70"
            } else {
                match cell.neighbor_mines {
                    1 => "bg-blue-300/70",
                    2 => "bg-green-300/70",
                    3 => "bg-yellow-300/70",
                    4 => "bg-orange-300/70",
                    5 => "bg-red-300/70",
                    6 => "bg-purple-300/70",
                    7 => "bg-pink-300/70",
                    8 => "bg-gray-300/70",
                    _ => "bg-gray-500/70",
                }
            };

            let classes = format!("{} cursor-default {}", bg_class, base_class);
            ("revealed", classes, content, Callback::noop(), Callback::noop())
        }

        CellState::Hidden => {
            if debug_mode && cell.is_mine {
                let classes = format!("{} bg-gray-300/70 text-red-600", base_class);
                ("hidden", classes, html! { <Icon data={IconData::LUCIDE_BOMB} width={"20px"} /> }, on_reveal.clone(), on_toggle_flag.clone())
            } else {
                let classes = format!("{} bg-gray-300/70 text-black", base_class);
                ("hidden", classes, html! {}, on_reveal.clone(), on_toggle_flag.clone())
            }
        }

        CellState::Flagged => {
            let classes = format!("{} bg-gray-300/70 text-black", base_class);
            let content = html! {
                <Icon data={IconData::LUCIDE_FLAG} width={"20px"} />
            };
            ("flagged", classes, content, on_reveal.clone(), on_toggle_flag.clone())
        }
    };

    html! {
        <button
            data-state={state}
            data-id={cell.id.to_string()}
            data-row={cell.row.clone()}
            data-column={cell.column.clone()}
            key={&*cell.key}
            class={classes}
            onclick={click_handler}
            oncontextmenu={context_handler}
        >
            { content }
        </button>
    }
}