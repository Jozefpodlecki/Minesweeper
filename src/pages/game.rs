use yew::*;

use crate::models::{GameDifficulty, Social};
use crate::components::*;
use crate::services::SettingsManager;

#[function_component(Game)]
pub fn game() -> Html {
    let social = unsafe { use_context::<Social>().unwrap_unchecked() };
    let settings = unsafe { use_context::<SettingsManager>().unwrap_unchecked() };

    let difficulty_class = match settings.get().difficulty {
        GameDifficulty::Easy => "bg-green-600 dark:bg-green-700 text-green-100 dark:text-green-200",
        GameDifficulty::Medium => "bg-orange-600 dark:bg-orange-700 text-orange-100 dark:text-orange-200",
        GameDifficulty::Hard => "bg-red-600 dark:bg-red-700 text-red-100 dark:text-red-200",
    };

    html! {
        <article data-header="" class="flex flex-col w-full h-full">
            <div class="select-none flex flex-col justify-center items-center dark:text-white">
                <h1 class="flex gap-2 justify-center items-center text-5xl py-2">
                    <span class="font-[oswald]">{"Minesweeper"}</span>
                    <img class="w-16 drop-shadow-[0_0_8px_rgba(220,38,38,0.6)] animate-pulse" src="public/favicon-32x32.png" alt="logo"/>
                </h1>
                <span class={format!(
                        "{} {}",
                        difficulty_class,
                        "pointer-events-none mt-2 inline-flex items-center px-3 py-1 rounded-full text-sm font-semibold shadow-md transition-colors duration-200 hover:brightness-110"
                    )}>
                    {"Difficulty: "}
                    <span class="ml-1">{settings.get().difficulty.label()}</span>
                </span>
            </div>
           <Content class="flex flex-1" />
           <Footer social={social} />
        </article>
    }

}