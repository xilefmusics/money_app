use stylist::Style;
use yew::prelude::*;

use crate::charts::MonthlyExtrapolation;

#[function_component]
pub fn Dashboard() -> Html {
    html! {
        <div class={Style::new(include_str!("dashboard.css")).expect("Unwrapping CSS should work!")}>
            <h1>{"Monthly Extrapolation"}</h1>
            <MonthlyExtrapolation />
        </div>
    }
}
