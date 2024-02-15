use stylist::Style;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: String,
    #[prop_or_default]
    pub subtitle: String,
    pub amount: i64,
    #[prop_or_default]
    pub color_amount: Option<i64>,
    #[prop_or_default]
    pub onedit: Option<Callback<MouseEvent>>,
    #[prop_or_default]
    pub ondelete: Option<Callback<MouseEvent>>,
    #[prop_or_default]
    pub onfilter: Option<Callback<MouseEvent>>,
}

#[function_component]
pub fn ListItem(props: &Props) -> Html {
    let color_amount = props.color_amount.unwrap_or(props.amount);
    let color = if color_amount > 0 {
        "green"
    } else if color_amount < 0 {
        "red"
    } else {
        "gray"
    };
    html! {
        <li class={Style::new(include_str!("list_item.css")).expect("Unwrapping CSS should work!")}>
            <div class="left">
                <span class="character-icon">
                    {props.title.chars().next().unwrap_or('?')}
                </span>
            </div>
            <div class="middle-left">
                <span class="title">
                    {&props.title}
                </span>
                <span class="subtitle">
                    {&props.subtitle}
                </span>
            </div>
            <div class="middle-right">
            {
                if let Some(onfilter) = props.onfilter.clone() {
                    html!{
                        <button
                            class="material-symbols-outlined icon"
                            onclick={onfilter}
                        >{"list_alt"}</button>
                    }
                } else {
                    html! {}
                }
            }
            {
                if let Some(onedit) = props.onedit.clone() {
                    html!{
                        <button
                            class="material-symbols-outlined icon"
                            onclick={onedit}
                        >{"edit"}</button>
                    }
                } else {
                    html! {}
                }
            }
            {
                if let Some(ondelete) = props.ondelete.clone() {
                    html!{
                        <button
                            class="material-symbols-outlined icon"
                            onclick={ondelete}
                        >{"delete"}</button>
                    }
                } else {
                    html! {}
                }
            }
            </div>
            <div class="right">
                <span class={"amount ".to_string() + color}>
                    {format!("{:.2} €", (props.amount as f64) / 100.)}
                </span>
            </div>
        </li>
    }
}
