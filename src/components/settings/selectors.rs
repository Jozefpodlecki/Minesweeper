use yew::prelude::*;

use crate::models::GameDifficulty;

#[derive(Properties, PartialEq)]
pub struct BackgroundSourceSelectorProps {
    pub value: Box<str>,
    pub on_change: Callback<Event>
}

#[function_component(BackgroundSourceSelector)]
pub fn background_source(props: &BackgroundSourceSelectorProps) -> Html {
    
    let BackgroundSourceSelectorProps { value, on_change } = props;

    html! {
        <select class="mt-1 p-2 bg-black border" value={value.to_string()} onchange={on_change}>
            <option value="default" selected={"default" == value.as_ref()}>{"Default"}</option>
            <option value="file" selected={"file" == value.as_ref()}>{"Upload file"}</option>
            <option value="url" selected={"url" == value.as_ref()}>{"URL"}</option>
        </select>
    }
}

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