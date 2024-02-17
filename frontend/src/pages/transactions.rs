use crate::tmp_fancy_yew::ListItem;
use crate::Route;
use fancy_yew::components::ResourceHeader;
use fancy_yew::rest::Resource;
use money_app_shared::transaction::Transaction;

use chrono::{Datelike, Local};
use serde::Deserialize;
use stylist::Style;
use url::Url;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Deserialize, Debug, Clone, Default)]
pub struct Query {
    pub start: Option<String>,
    pub end: Option<String>,
    pub pod: Option<String>,
    pub debt: Option<String>,
    pub budget: Option<String>,
    pub inbudget: Option<String>,
    pub ttype: Option<String>,
}

impl Query {
    pub fn api_url(&self) -> String {
        let base = Url::parse("https://example.net").unwrap();
        let mut url = Url::parse("https://example.net/api/transactions").unwrap();
        {
            let mut query_pairs = url.query_pairs_mut();
            query_pairs.append_pair(
                "start",
                &self
                    .start
                    .clone()
                    .unwrap_or(Local::now().year().to_string()),
            );
            if let Some(end) = &self.end {
                query_pairs.append_pair("end", end);
            }
            if let Some(pod) = &self.pod {
                query_pairs.append_pair("pod", pod);
            }
            if let Some(debt) = &self.debt {
                query_pairs.append_pair("debt", debt);
            }
            if let Some(budget) = &self.budget {
                query_pairs.append_pair("budget", budget);
            }
            if let Some(inbudget) = &self.inbudget {
                query_pairs.append_pair("inbudget", inbudget);
            }
            if let Some(ttype) = &self.ttype {
                query_pairs.append_pair("ttype", ttype);
            }
        }
        base.make_relative(&url).unwrap().to_string()
    }
}

#[function_component]
pub fn Transactions() -> Html {
    let query = use_location()
        .unwrap()
        .query::<Query>()
        .unwrap_or(Query::default());

    let transactions: UseStateHandle<Resource<Transaction>> =
        use_state(|| Resource::new(query.api_url(), "transactions".into(), Vec::default()));
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
                highlight={!transaction.validate()}
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
