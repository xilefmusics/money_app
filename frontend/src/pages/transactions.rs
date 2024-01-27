use crate::tmp_fancy_yew::ListItem;
use crate::Route;
use fancy_yew::components::ResourceHeader;
use fancy_yew::rest::Resource;
use money_app_shared::transaction::Transaction;

use stylist::Style;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component]
pub fn Transactions() -> Html {
    let transactions: UseStateHandle<Resource<Transaction>> = use_state(|| {
        Resource::new(
            "/api/transactions".into(),
            "transactions".into(),
            Vec::default(),
        )
    });
    {
        let transactions = transactions.clone();
        use_effect_with((), move |_| {
            Resource::reload(transactions);
            || ()
        });
    }

    let navigator = use_navigator().unwrap();
    let transaction_items = transactions
        .iter()
        .map(|transaction| {
            let navigator = navigator.clone();
            let id = transaction.id.clone().unwrap();
            let onedit = {
                let id = id.clone();
                Callback::from(move |_: MouseEvent| {
                    navigator.push(&Route::Transaction { id: id.clone() })
                })
            };
            html! {<ListItem
                title={transaction.title()}
                subtitle={transaction.date.format("%d %b %Y").to_string()}
                amount={transaction.amount as i64}
                color_amount={transaction.signed_amount()}
                onedit={onedit}
                ondelete={Resource::delete_callback(transaction.clone(),transactions.clone())}
            />}
        })
        .collect::<Vec<Html>>();

    html! {
        <div class={Style::new(include_str!("transactions.css")).expect("Unwrapping CSS should work!")}>
            <ResourceHeader<Transaction> handle={transactions.clone()} />
            <ul>
                {transaction_items}
            </ul>
        </div>
    }
}
