use crate::tmp_fancy_yew::ListItem;
use money_app_shared::goal::{Goal, GoalData};

use gloo::net::http::Request;
use stylist::Style;
use yew::prelude::*;

#[function_component]
pub fn Goals() -> Html {
    let goals = use_state(|| vec![]);
    {
        let goals = goals.clone();
        use_effect_with((), move |_| {
            let goals = goals.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_goals: Vec<Goal> = Request::get("/api/goals")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                goals.set(fetched_goals);
            });
            || ()
        });
    }

    let goal_items = goals
        .iter()
        .map(|goal| match goal.data {
            GoalData::RealWealth(wealth) => html! {<ListItem
                title={"Wealth"}
                subtitle={goal.due.format("%d %b %Y").to_string()}
                amount={wealth as i64}
            />},
        })
        .collect::<Vec<Html>>();

    html! {
        <div class={Style::new(include_str!("goals.css")).expect("Unwrapping CSS should work!")}>
            <ul>
                {goal_items}
            </ul>
        </div>
    }
}
