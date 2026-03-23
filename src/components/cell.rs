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

    let base_class = "w-10 h-10 flex items-center justify-center font-bold transition-colors";

    let (state, classes, content) = match cell.state {
        CellState::Revealed => {
            let content = if cell.is_mine {
                html! { <Icon data={IconData::LUCIDE_BOMB} width={"20px"} /> }
            } else if cell.neighbor_mines > 0 {
                html! { cell.neighbor_mines.to_string() }
            } else {
                html! {}
            };

            let state_class = if cell.is_mine {
                "bg-red-500/70"
            }
            else {
                "bg-gray-100/70"
            };

            let classes = format!("{} cursor-default {} text-black", state_class, base_class);

            ("revealed", classes, content)
        }

        CellState::Hidden => {
            let classes = format!("{} bg-gray-300/70 text-black", base_class);
            ("hidden", classes, html! {})
        }

        CellState::Flagged => {
            let classes = format!("{} bg-gray-300/70 text-black pointer-events-none", base_class);
            let content = html! {
                <Icon data={IconData::LUCIDE_FLAG} width={"20px"} />
            };
            ("flagged", classes, content)
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
            onclick={&props.on_reveal}
            oncontextmenu={&props.on_toggle_flag}
        >
            { content }
        </button>
    }
}