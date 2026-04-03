use yew::*;
use yew_icons::{Icon, IconData};

use crate::models::AppError;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub error: AppError,
    pub on_retry: Callback<MouseEvent>
}

#[function_component(Error)]
pub fn error(props: &Props) -> Html {
   
    html! {
        <article data-error="" class="flex w-full h-full justify-center items-center text-white px-4">
            <section class="
                flex flex-col gap-6
                max-w-md w-full
                p-6
                rounded-2xl
                bg-black/60 backdrop-blur-md
                shadow-xl
                border border-white/10
            ">
                <div class="flex items-center gap-3 text-red-400">
                    <Icon data={IconData::LUCIDE_ALERT_TRIANGLE} width={"28px"} />
                    <h2 class="text-xl font-semibold">
                        {"Could not load the game"}
                    </h2>
                </div>

                <p class="text-sm text-white/80 break-words">
                    {props.error.to_string()}
                </p>

                <div class="flex justify-end">
                    <button
                        type="button"
                        onclick={&props.on_retry}
                        class="
                            flex items-center gap-2
                            px-4 py-2
                            rounded-lg
                            bg-white/10 hover:bg-white/20
                            transition
                            border border-white/10
                            text-sm
                        "
                    >
                        <Icon data={IconData::LUCIDE_TIMER_RESET} width={"18px"} />
                        <span>{"Retry"}</span>
                    </button>
                </div>
            </section>
        </article>
    }
}