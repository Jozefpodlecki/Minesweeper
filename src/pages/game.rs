use yew::*;

use crate::models::Social;
use crate::components::*;

#[function_component(Game)]
pub fn game() -> Html {
    let social = unsafe { use_context::<Social>().unwrap_unchecked() };

    html! {
        <article class="flex flex-col w-full h-full">
           <h1 class="flex gap-2 justify-center items-center dark:text-white text-5xl py-2">
                <span class="font-[oswald]">{"Minesweeper"}</span>
                <img class="w-16 drop-shadow-[0_0_8px_rgba(220,38,38,0.6)] animate-pulse" src="public/favicon-32x32.png" alt="logo"/>
            </h1>
           <Content class="flex flex-1" />
           <Footer social={social} />
        </article>
    }

}