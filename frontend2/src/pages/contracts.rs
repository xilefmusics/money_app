use stylist::Style;
use yew::prelude::*;

#[function_component]
pub fn Contracts() -> Html {
    html! {
        <div class={Style::new(include_str!("contracts.css")).expect("Unwrapping CSS should work!")}>
            <h1>{"Contracts"}</h1>
        </div>
    }
}
