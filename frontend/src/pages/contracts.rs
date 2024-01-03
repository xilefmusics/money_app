use money_app_shared::contract::Contract;
use crate::tmp_fancy_yew::{ListItem};

use stylist::Style;
use yew::prelude::*;
use gloo::net::http::Request;

#[function_component]
pub fn Contracts() -> Html {
    let contracts = use_state(|| vec![]);
    {
        let contracts = contracts.clone();
        use_effect_with((), move |_| {
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
                fetched_contracts.sort_by(|a, b| a.payment().date_of_next_payment().cmp(&b.payment().date_of_next_payment()));
                contracts.set(fetched_contracts);
            });
            || ()
        });
    }

    let contract_items = contracts.iter().map(|contract| html!{<ListItem 
        title={contract.title.clone()}
        subtitle={contract.payments[contract.payments.len() -1].date_of_next_payment().format("%d %b %Y").to_string()}
        amount={contract.payments[contract.payments.len() -1].signed_monthly_amount()}
    />}).collect::<Vec<Html>>();

    html! {
        <div class={Style::new(include_str!("contracts.css")).expect("Unwrapping CSS should work!")}>
            <ul>
                {contract_items} 
            </ul>
        </div>
    }
}
