use crate::tmp_fancy_yew::ListItem;
use crate::Route;
use money_app_shared::goal::{Goal, GoalData};

use gloo::console::log;
use gloo::net::http::Request;
use stylist::Style;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component]
pub fn Goals() -> Html {
    let goals = use_state(|| vec![]);
    let load_goals = {
        let goals = goals.clone();
        move || {
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
        }
    };
    {
        let load_goals = load_goals.clone();
        use_effect_with((), move |_| {
            load_goals();
            || ()
        });
    }

    let add = {
        let load_goals = load_goals.clone();
        move |_: MouseEvent| {
            let goals = vec![Goal::default()];
            let load_goals = load_goals.clone();
            wasm_bindgen_futures::spawn_local(async move {
                Request::post("/api/goals")
                    .json(&goals)
                    .unwrap()
                    .send()
                    .await
                    .unwrap();
                load_goals();
            });
        }
    };

    let delete = {
        let load_goals = load_goals.clone();
        move |id: String| {
            let mut goal = Goal::default();
            goal.id = Some(id);
            let goals = vec![goal];
            let load_goals = load_goals.clone();
            wasm_bindgen_futures::spawn_local(async move {
                Request::delete("/api/goals")
                    .json(&goals)
                    .unwrap()
                    .send()
                    .await
                    .unwrap();
                load_goals();
            });
        }
    };

    let navigator = use_navigator().unwrap();
    let goal_items = goals
        .iter()
        .map(|goal| match goal.data {
            GoalData::RealWealth(wealth) => {
                let navigator = navigator.clone();
                let id = goal.id.clone().unwrap();
                let onedit = {
                    let id = id.clone();
                    Callback::from(move |_: MouseEvent| {
                        navigator.push(&Route::Goal { id: id.clone() })
                    })
                };
                let delete = delete.clone();
                let ondelete = Callback::from(move |_: MouseEvent| delete(id.clone()));
                html! {<ListItem
                    title={"Wealth"}
                    subtitle={goal.due.format("%d %b %Y").to_string()}
                    amount={wealth as i64}
                    onedit={onedit}
                    ondelete={ondelete}
                />}
            }
        })
        .collect::<Vec<Html>>();

    html! {
        <div class={Style::new(include_str!("goals.css")).expect("Unwrapping CSS should work!")}>
            <div class="header">
                <button
                    class="material-symbols-outlined icon"
                    onclick={add}
                >{"add"}</button>
            </div>
            <ul>
                {goal_items}
            </ul>
        </div>
    }
}
