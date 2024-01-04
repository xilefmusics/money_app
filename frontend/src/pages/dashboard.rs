use fancy_yew::components::{ChartJs, ConfigBuilder};
use money_app_shared::exploration_item::ExtrapolationItem;

use gloo::net::http::Request;
use stylist::Style;
use yew::prelude::*;

#[function_component]
pub fn Dashboard() -> Html {
    let extrapolation = use_state(|| vec![]);
    {
        let extrapolation = extrapolation.clone();
        use_effect_with((), move |_| {
            let extrapolation = extrapolation.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_extrapolation: Vec<ExtrapolationItem> =
                    Request::get("/api/extrapolation")
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();
                extrapolation.set(fetched_extrapolation);
            });
            || ()
        });
    }

    let chart_config = ConfigBuilder::bar()
        .labels(
            &extrapolation
                .iter()
                .map(|item| item.date.format("%b").to_string())
                .collect::<Vec<String>>(),
        )
        .dataset(
            &extrapolation
                .iter()
                .map(|item| (item.contract_expenses as f64) / 100.)
                .collect::<Vec<f64>>(),
        )
        .dataset_stack(0)
        .dataset_label("Expenses")
        .dataset_border_color_rgba(150, 50, 0, 0.5)
        .dataset_border_width(1)
        .dataset_background_color_rgba(150, 50, 0, 0.2)
        .dataset(
            &extrapolation
                .iter()
                .map(|item| (item.planned_savings as f64) / 100.)
                .collect::<Vec<f64>>(),
        )
        .dataset_stack(0)
        .dataset_label("Savings")
        .dataset_border_color_rgba(50, 50, 230, 0.5)
        .dataset_border_width(1)
        .dataset_background_color_rgba(50, 50, 230, 0.2)
        .dataset(
            &extrapolation
                .iter()
                .map(|item| (item.freely_available as f64) / 100.)
                .collect::<Vec<f64>>(),
        )
        .dataset_stack(0)
        .dataset_label("Free")
        .dataset_border_color_rgba(0, 80, 0, 0.5)
        .dataset_border_width(1)
        .dataset_background_color_rgba(0, 80, 0, 0.2)
        .build()
        .unwrap();

    if extrapolation.len() == 0 {
        html! {}
    } else {
        html! {
            <div class={Style::new(include_str!("dashboard.css")).expect("Unwrapping CSS should work!")}>
                <h1>{"Monthly Extrapolation"}</h1>
                <ChartJs config={chart_config}/>
            </div>
        }
    }
}
