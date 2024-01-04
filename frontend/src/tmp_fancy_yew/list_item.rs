use stylist::Style;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: String,
    pub subtitle: String,
    pub amount: i64,
    #[prop_or_default]
    pub color_amount: Option<i64>,
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
                    {props.title.chars().next().unwrap()}
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
            </div>
            <div class="right">
                <span class={"amount ".to_string() + color}>
                    {format!("{:.2} €", (props.amount as f64) / 100.)}
                </span>
            </div>
        </li>
    }
}
