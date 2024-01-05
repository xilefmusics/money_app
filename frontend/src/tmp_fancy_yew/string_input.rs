use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub bind_handle: UseStateHandle<String>,
    #[prop_or_default]
    pub options: Vec<String>,
}

#[function_component]
pub fn StringInput(props: &Props) -> Html {
    let oninput = {
        let bind_handle = props.bind_handle.clone();
        move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            bind_handle.set(input.value())
        }
    };
    let value = (*props.bind_handle).clone();
    let options = props
        .options
        .iter()
        .map(|option| html! {<option value={option.clone()} />})
        .collect::<Vec<Html>>();
    let list_id = "todo random";

    if options.len() > 0 {
        html! {
            <>
                <input
                    list={list_id}
                    value={value}
                    oninput={oninput}
                    type="string"
                />
                <datalist id={list_id}>
                    {options}
                </datalist>
            </>
        }
    } else {
        html! {
            <input
                value={value}
                oninput={oninput}
                type="string"
            />
        }
    }
}
