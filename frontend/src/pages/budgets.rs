use crate::tmp_fancy_yew::ListItem;
use fancy_yew::components::{ChartJs, ConfigBuilder};
use money_app_shared::history::AssociatedTypeValues;

use gloo::net::http::Request;
use stylist::Style;
use yew::prelude::*;

#[function_component]
pub fn Budgets() -> Html {
    let year = 0;
    let month = 3;
    let day = 0;
    let len = 1;
    let query = format!(
        "/api/history/budgets?year={}&month={}&day={}&len={}",
        year, month, day, len
    );
    let query_in = format!(
        "/api/history/inbudgets?year={}&month={}&day={}&len={}",
        year, month, day, len
    );

    let budget_history = use_state(|| vec![]);
    {
        let budget_history = budget_history.clone();
        use_effect_with((), move |_| {
            let budget_history = budget_history.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_budget_history: Vec<AssociatedTypeValues> = Request::get(&query)
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                budget_history.set(fetched_budget_history);
            });
            || ()
        });
    }

    let inbudget_history = use_state(|| vec![]);
    {
        let inbudget_history = inbudget_history.clone();
        use_effect_with((), move |_| {
            let inbudget_history = inbudget_history.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_inbudget_history: Vec<AssociatedTypeValues> = Request::get(&query_in)
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                inbudget_history.set(fetched_inbudget_history);
            });
            || ()
        });
    }

    let list_items = if budget_history.len() > 0 {
        Some(
            budget_history[budget_history.len() - 1]
                .data
                .iter()
                .map(|(key, value)| (key, value.value))
                .filter(|(_, value)| *value != 0)
                .map(|(key, value)| {
                    html! {<ListItem
                        title={key.clone()}
                        amount={value}
                        color_amount={-value}
                    />}
                })
                .collect::<Vec<Html>>(),
        )
    } else {
        None
    };

    let list_items_in = if inbudget_history.len() > 0 {
        Some(
            inbudget_history[inbudget_history.len() - 1]
                .data
                .iter()
                .map(|(key, value)| (key, value.value))
                .filter(|(_, value)| *value != 0)
                .map(|(key, value)| {
                    html! {<ListItem
                        title={key.clone()}
                        amount={value}
                    />}
                })
                .collect::<Vec<Html>>(),
        )
    } else {
        None
    };

    let chart_config = if budget_history.len() > 0 {
        let (labels, dataset): (Vec<String>, Vec<f64>) = budget_history[0]
            .data
            .iter()
            .map(|(key, value)| (key.to_string(), value.value as f64 / 100.))
            .unzip();
        Some(
            ConfigBuilder::doughnut()
                .labels(&labels)
                .dataset(&dataset)
                .build()
                .unwrap(),
        )
    } else {
        None
    };

    let chart_config_in = if inbudget_history.len() > 0 {
        let (labels, dataset): (Vec<String>, Vec<f64>) = inbudget_history[0]
            .data
            .iter()
            .map(|(key, value)| (key.to_string(), value.value as f64 / 100.))
            .unzip();
        Some(
            ConfigBuilder::doughnut()
                .labels(&labels)
                .dataset(&dataset)
                .build()
                .unwrap(),
        )
    } else {
        None
    };

    html! {
        <div class={Style::new(include_str!("budgets.css")).expect("Unwrapping CSS should work!")}>
            {
                if let Some(chart_config) = chart_config {
                    html! {
                        <div class="chart-wrapper">
                            <ChartJs config={chart_config}/>
                        </div>
                    }
                } else {
                    html! {}

                }

            }
            <ul>
                {list_items}
            </ul>
            {
                if let Some(chart_config) = chart_config_in {
                    html! {
                        <div class="chart-wrapper">
                            <ChartJs config={chart_config}/>
                        </div>
                    }
                } else {
                    html! {}

                }

            }
            <ul>
                {list_items_in}
            </ul>
        </div>
    }
}
