use crate::tmp_fancy_yew::ListItem;
use money_app_shared::contract::Contract;

use gloo::net::http::Request;
use stylist::Style;
use yew::prelude::*;

#[function_component]
pub fn Contracts() -> Html {
    let contracts: UseStateHandle<Vec<Contract>> = use_state(|| vec![]);
    let load_contracts = {
        let contracts = contracts.clone();
        move || {
            let contracts = contracts.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let mut fetched_contracts: Vec<Contract> = Request::get("/api/contracts")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                // TODO: Move this to the backend
                fetched_contracts.sort_by(|a, b| {
                    a.payment
                        .date_of_next_payment()
                        .cmp(&b.payment.date_of_next_payment())
                });
                contracts.set(fetched_contracts);
            });
        }
    };
    {
        let load_contracts = load_contracts.clone();
        use_effect_with((), move |_| {
            load_contracts();
            || ()
        });
    }

    let add = {
        let load_contracts = load_contracts.clone();
        move |_: MouseEvent| {
            let contracts = vec![Contract::default()];
            let load_contracts = load_contracts.clone();
            wasm_bindgen_futures::spawn_local(async move {
                Request::post("/api/contracts")
                    .json(&contracts)
                    .unwrap()
                    .send()
                    .await
                    .unwrap();
                load_contracts();
            });
        }
    };

    let contract_items = contracts
        .iter()
        .map(|contract| {
            html! {<ListItem
                title={contract.title.clone()}
                subtitle={contract.payment.date_of_next_payment().format("%d %b %Y").to_string()}
                amount={contract.payment.signed_monthly_amount()}
            />}
        })
        .collect::<Vec<Html>>();

    html! {
        <div class={Style::new(include_str!("contracts.css")).expect("Unwrapping CSS should work!")}>
            <div class="header">
                <button
                    class="material-symbols-outlined icon"
                    onclick={add}
                >{"add"}</button>
            </div>
            <ul>
                {contract_items}
            </ul>
        </div>
    }
}
