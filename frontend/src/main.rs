mod charts;
mod pages;
mod tmp_fancy_yew;

use pages::{Contracts, Dashboard, Debts, Goals, Pods, Transactions};

use fancy_yew::components::{DefaultLayout, NavItemBuilder, Navable};

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Index,
    #[at("/dashboard")]
    Dashboard,
    #[at("/transactions")]
    Transactions,
    #[at("/pods")]
    Pods,
    #[at("/debts")]
    Debts,
    #[at("/contracts")]
    Contracts,
    #[at("/goals")]
    Goals,
    #[not_found]
    #[at("/404")]
    NotFound,
}

impl Route {}

impl Navable for Route {
    fn route_items() -> Vec<Self> {
        vec![
            Route::Dashboard,
            Route::Transactions,
            Route::Pods,
            Route::Debts,
            Route::Contracts,
            Route::Goals,
        ]
    }

    fn to_nav_item(self) -> NavItemBuilder<'static> {
        match self {
            Route::Dashboard => NavItemBuilder::new()
                .path("/dashboard")
                .icon("dashboard")
                .text("Dashboard")
                .callback(Callback::from(|navigator: Navigator| {
                    navigator.push(&Route::Dashboard)
                }))
                .index(),
            Route::Transactions => NavItemBuilder::new()
                .path("/transactions")
                .icon("list_alt")
                .text("Transactions")
                .callback(Callback::from(|navigator: Navigator| {
                    navigator.push(&Route::Transactions)
                })),
            Route::Pods => NavItemBuilder::new()
                .path("/pods")
                .icon("wallet")
                .text("Pods")
                .callback(Callback::from(|navigator: Navigator| {
                    navigator.push(&Route::Pods)
                })),
            Route::Debts => NavItemBuilder::new()
                .path("/debts")
                .icon("credit_card_off")
                .text("Debts")
                .callback(Callback::from(|navigator: Navigator| {
                    navigator.push(&Route::Debts)
                })),
            Route::Contracts => NavItemBuilder::new()
                .path("/contracts")
                .icon("contract")
                .text("Contracts")
                .callback(Callback::from(|navigator: Navigator| {
                    navigator.push(&Route::Contracts)
                })),
            Route::Goals => NavItemBuilder::new()
                .path("/goals")
                .icon("flag")
                .text("Goals")
                .callback(Callback::from(|navigator: Navigator| {
                    navigator.push(&Route::Goals)
                })),
            _ => NavItemBuilder::new(),
        }
    }

    fn render(route: Route) -> Html {
        html! {
            <DefaultLayout<Route> nav_routes={Route::route_items()}>{
                match route {
                    Route::Index => html! { <h1>{ "Dashboard" }</h1> },
                    Route::Dashboard => html! { <Dashboard /> },
                    Route::Transactions => html! { <Transactions /> },
                    Route::Pods => html! { <Pods /> },
                    Route::Debts => html! { <Debts /> },
                    Route::Contracts => html! { <Contracts /> },
                    Route::Goals => html! { <Goals /> },
                    Route::NotFound => html! { <h1>{ "404 Not Found" }</h1> },
        }}
            </DefaultLayout<Route>>
        }
    }
}

#[function_component]
fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Route::render} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
