use yew::prelude::*;
use yew_router::prelude::Link;

use crate::{route::Route, models::Social};
use yew_icons::{Icon, IconData};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub social: Social
}

#[function_component(Footer)]
pub fn footer(props: &Props) -> Html {
    html! {
        <footer class="h-10 bg-black flex items-center px-10">
           <a class="font-[roboto] text-sm md:text-base text-white flex items-center gap-1 hover:underline" 
               href={props.social.portfolio.to_string()}>
                {"© Jozef Podlecki 2026"}
                <Icon data={IconData::LUCIDE_EXTERNAL_LINK} width={"14px"}/>
            </a>
           <div class="flex gap-2 ml-auto">
                <a class="transition-all dark:text-white hover:bg-black/80 hover:opacity-50" href={props.social.github.to_string()}>
                    <Icon data={IconData::SIMPLE_ICONS_GITHUB} class="w-4 h-4 sm:w-5 sm:h-5" />
                </a>
                <a class="transition-all dark:text-white hover:bg-black/80 hover:opacity-50" href={props.social.linkedin.to_string()}>
                    <Icon data={IconData::SIMPLE_ICONS_LINKEDIN} class="w-4 h-4 sm:w-5 sm:h-5" />
                </a>
           </div>
        </footer>
    }
}