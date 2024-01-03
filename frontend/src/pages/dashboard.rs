use stylist::Style;
use yew::prelude::*;

#[function_component]
pub fn Dashboard() -> Html {
    html! {
        <div class={Style::new(include_str!("dashboard.css")).expect("Unwrapping CSS should work!")}>
            <h1>{"Dashboard"}</h1>
        </div>
    }
}
