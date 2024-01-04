use stylist::Style;
use yew::prelude::*;

use crate::charts::{MonthlyExtrapolation, WealthHistory};

#[function_component]
pub fn Dashboard() -> Html {
    html! {
        <div class={Style::new(include_str!("dashboard.css")).expect("Unwrapping CSS should work!")}>
            <h1>{"Wealth History"}</h1>
            <WealthHistory
                len=26
                year=0
                month=1
                day=1
            />
            <h1>{"Monthly Extrapolation"}</h1>
            <MonthlyExtrapolation />
        </div>
    }
}
