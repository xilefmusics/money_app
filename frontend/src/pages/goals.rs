use crate::tmp_fancy_yew::ListItem;
use crate::Route;
use money_app_shared::goal::{Goal, GoalData};

use gloo::net::http::Request;
use stylist::Style;
use yew::prelude::*;
use yew_router::prelude::*;

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

    let navigator = use_navigator().unwrap();
    let goal_items = goals
        .iter()
        .map(|goal| match goal.data {
            GoalData::RealWealth(wealth) => {
                let navigator = navigator.clone();
                let id = goal.id.clone().unwrap();
                let onclick = Callback::from(move |_: MouseEvent| {
                    navigator.push(&Route::Goal { id: id.clone() })
                });
                html! {<ListItem
                    title={"Wealth"}
                    subtitle={goal.due.format("%d %b %Y").to_string()}
                    amount={wealth as i64}
                    onclick={onclick}
                />}
            }
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
