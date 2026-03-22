use yew::*;
use yew_router::prelude::Link;

use crate::{route::Route};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
}

#[function_component(Loader)]
pub fn loader(props: &Props) -> Html {
   
    html! {
        <div class="loader"/>
    }

}