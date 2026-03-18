use yew::*;
use yew_router::prelude::Link;

use crate::{route::Route};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub src: String
}

#[function_component(Background)]
pub fn background(props: &Props) -> Html {
   
    html! {
        <img class="fixed inset-0 w-full h-full object-cover -z-10 brightness-50" src={props.src.clone()}/>
    }

}