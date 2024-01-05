use chrono::{DateTime, Local, NaiveDateTime};
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub bind_handle: UseStateHandle<DateTime<Local>>,
}

#[function_component]
pub fn DateInput(props: &Props) -> Html {
    let oninput = {
        let bind_handle = props.bind_handle.clone();
        move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let string: String = format!("{}T0:0:0", input.value());
            let naive_date = NaiveDateTime::parse_from_str(&string, "%Y-%m-%dT%H:%M:%S").unwrap();
            let date = DateTime::<Local>::from_naive_utc_and_offset(
                naive_date,
                Local::now().offset().clone(),
            );
            bind_handle.set(date)
        }
    };

    html! {
        <input
            value={props.bind_handle.format("%Y-%m-%d").to_string()}
            oninput={oninput}
            type="date"
        />
    }
}
