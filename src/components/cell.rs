use yew::prelude::*;
use yew_router::prelude::Link;

use crate::{game::GameCell, models::Social, route::Route};
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
    let cell = &props.cell;
    let row = cell.row_id;
    let col = cell.column_id;
    
    let base_class = "w-10 h-10 flex items-center justify-center font-bold transition-colors";
    let state_class = if props.disabled {
        if cell.is_mine {
            "bg-red-500"
        } else {
            "bg-gray-300"
        }
    } else if cell.is_revealed {
        if cell.is_mine {
            "bg-red-500"
        } else {
            "bg-gray-300"
        }
    } else if cell.is_flagged {
        "bg-blue-300 hover:bg-blue-400"
    } else {
        "bg-gray-500 hover:bg-gray-400 cursor-pointer"
    };
    
    let text_color = if cell.is_revealed && !cell.is_mine {
        match cell.neighbor_mines {
            1 => "text-blue-700",
            2 => "text-green-700",
            3 => "text-red-700",
            4 => "text-purple-700",
            _ => "text-gray-900",
        }
    } else {
        ""
    };
    
    let content = if cell.is_revealed && !cell.is_mine && cell.neighbor_mines > 0 {
        cell.neighbor_mines.to_string()
    } else if cell.is_revealed && cell.is_mine {
        "💣".to_string()
    } else if cell.is_flagged && !props.disabled {
        "🚩".to_string()
    } else {
        "".to_string()
    };
    
    let classes = format!("{} {} {}", base_class, state_class, text_color);
    
    html! {
        <button
            data-row={row.to_string()}
            data-column={col.to_string()}
            key={format!("{}-{}", row, col)}
            class={classes}
            onclick={props.on_reveal.clone()}
            oncontextmenu={props.on_toggle_flag.clone()}
            disabled={props.disabled}
        >
            { content }
        </button>
    }
}