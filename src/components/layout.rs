use yew::*;

use crate::{components::Background, route::Route};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Html,
}

pub struct Layout;

impl Component for Layout {
    type Message = ();
    type Properties = Props;

    fn create(_context: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, context: &Context<Self>) -> Html {
        html! {
            <>
                <Background/>
                {context.props().children.clone()}
            </>
        }
    }
}