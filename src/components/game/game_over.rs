use log::*;
use yew::prelude::*;
use yew_icons::{Icon, IconData};

use crate::components::Records;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub has_won: bool,
    pub on_play: Callback<MouseEvent>,
    pub on_review: Callback<MouseEvent>,
}

#[function_component(GameOver)]
pub fn game_over(props: &Props) -> Html {
    let Props { has_won, on_play, on_review } = props;

    let result = if *has_won {
        html! { <span class="text-2xl text-green-500 font-bold">{"You won"}</span> }
    } else {
        html! { <span class="text-2xl text-red-500 font-bold">{"You lost"}</span> }
    };

    html! {
        html! {
            <div class="fixed inset-0 bg-black/50 flex items-center justify-center z-40">
                <div class="bg-black/80 p-10 rounded shadow-lg flex flex-col items-center gap-4 w-200 relative">
                    <button
                        data-action="review"
                        type="button"
                        onclick={on_review}
                        class="absolute top-4 right-4 p-2 hover:bg-gray-700 rounded transition-colors"
                    >
                        <Icon data={IconData::LUCIDE_EYE_OFF} width={"20px"}/>
                    </button>
                    <span class="text-4xl">{"Game over!"}</span>
                    {result}

                    <button
                        data-action="play"
                        type="button"
                        onclick={on_play}
                        class="flex gap-2 items-center border p-2"
                    >
                        <span class="text-md">{"Play again"}</span>
                        <Icon data={IconData::LUCIDE_PLAY} width={"20px"}/>
                    </button>

                    <Records/>
                </div>
            </div>
        }
    }
}