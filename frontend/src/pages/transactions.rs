use crate::tmp_fancy_yew::ListItem;
use money_app_shared::transaction::Transaction;

use gloo::net::http::Request;
use stylist::Style;
use yew::prelude::*;

#[function_component]
pub fn Transactions() -> Html {
    let transactions = use_state(|| vec![]);
    {
        let transactions = transactions.clone();
        use_effect_with((), move |_| {
            let transactions = transactions.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let mut fetched_transactions: Vec<Transaction> = Request::get("/api/transactions")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                fetched_transactions.reverse();
                transactions.set(fetched_transactions);
            });
            || ()
        });
    }

    let transaction_items = transactions
        .iter()
        .map(|transaction| {
            html! {<ListItem
                title={transaction.title()}
                subtitle={transaction.date.format("%d %b %Y").to_string()}
                amount={transaction.amount as i64}
                color_amount={transaction.signed_amount()}
            />}
        })
        .collect::<Vec<Html>>();

    html! {
        <div class={Style::new(include_str!("transactions.css")).expect("Unwrapping CSS should work!")}>
            <ul>
                {transaction_items}
            </ul>
        </div>
    }
}
