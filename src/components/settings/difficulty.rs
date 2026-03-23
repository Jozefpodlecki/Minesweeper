use yew::prelude::*;

use crate::models::GameDifficulty;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub value: GameDifficulty,
    pub on_change: Callback<Event>
}

#[function_component(DifficultySelector)]
pub fn difficulty(props: &Props) -> Html {
    
let Props { value, on_change } = props;

    html! {
        <label class="flex flex-col text-sm">
            { "Difficulty" }
            <select
                class="mt-1 p-2 bg-black border"
                onchange={on_change}
                value={value.to_string()}
            >
                {
                    GameDifficulty::all().iter().map(|difficulty| {
                        let value_str = difficulty.to_string();

                        html! {
                            <option key={value_str.clone()} value={value_str} selected={value == difficulty}>
                                { difficulty.label() }
                            </option>
                        }
                    }).collect::<Html>()
                }
            </select>
        </label>
    }
}