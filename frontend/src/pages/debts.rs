use crate::tmp_fancy_yew::ListItem;
use fancy_yew::components::{ChartJs, ConfigBuilder};
use money_app_shared::history::AssociatedTypeDiffValues;

use gloo::net::http::Request;
use stylist::Style;
use yew::prelude::*;

#[function_component]
pub fn Debts() -> Html {
    let year = 0;
    let month = 3;
    let day = 0;
    let len = 13;
    let query = format!(
        "/api/history/debts?year={}&month={}&day={}&len={}",
        year, month, day, len
    );

    let debt_history = use_state(|| vec![]);
    {
        let debt_history = debt_history.clone();
        use_effect_with((), move |_| {
            let debt_history = debt_history.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_debt_history: Vec<AssociatedTypeDiffValues> = Request::get(&query)
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                debt_history.set(fetched_debt_history);
            });
            || ()
        });
    }

    let list_items = if debt_history.len() > 0 {
        Some(
            debt_history[debt_history.len() - 1]
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

    let chart_config = if debt_history.len() > 0 {
        let labels = debt_history
            .iter()
            .map(|item| item.date.format("%b-%Y").to_string())
            .collect::<Vec<String>>();
        let mut config_builder = ConfigBuilder::line().labels(&labels);

        let datasets = debt_history[0]
            .data
            .keys()
            .map(|debt| {
                (
                    debt.to_string(),
                    debt_history
                        .iter()
                        .map(|item| item.data.get(debt).unwrap().value as f64 / 100.)
                        .collect::<Vec<f64>>(),
                )
            })
            .collect::<Vec<(String, Vec<f64>)>>();

        for dataset in &datasets {
            config_builder = config_builder
                .dataset(&dataset.1)
                .dataset_label(&dataset.0)
                .dataset_border_width(1);
        }

        Some(config_builder.build().unwrap())
    } else {
        None
    };

    html! {
        <div class={Style::new(include_str!("debts.css")).expect("Unwrapping CSS should work!")}>
            {
                if let Some(chart_config) = chart_config {
                    html! {
                        <ChartJs config={chart_config}/>
                    }
                } else {
                    html! {}

                }

            }
            <ul>
                {list_items}
            </ul>
        </div>
    }
}
