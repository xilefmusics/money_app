use stylist::Style;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Html,
}

#[function_component]
pub fn Card(props: &Props) -> Html {
    html! {
        <div class={Style::new(include_str!("card.css")).expect("Unwrapping CSS should work!")}>
            {props.children.clone()}
        </div>
    }
}
