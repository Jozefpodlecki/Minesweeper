use yew::*;
use yew_router::prelude::Link;

use crate::models::Social;
use crate::{route::Route};
use crate::components::*;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    
}

#[function_component(Game)]
pub fn game(props: &Props) -> Html {
    let social = unsafe { use_context::<Social>().unwrap_unchecked() };

    html! {
        <article class="flex flex-col w-full h-full">
           <h1 class="flex gap-2 justify-center items-center dark:text-white text-5xl py-2">
                <span class="font-[oswald]">{"Minesweeper"}</span>
                <img class="w-16" src="public/favicon-32x32.png" alt="logo"/>
            </h1>
           <Content class="flex flex-1" />
           <Footer social={social} />
        </article>
    }

}