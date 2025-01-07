use std::{borrow::Borrow, vec};

use web_sys:: HtmlInputElement;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let todos = use_state(|| vec![]);
    let input = use_state(|| "".to_string());

    let onclick = {
        let input = input.clone();
        let todos = todos.clone();
        Callback::from(move |_| {
            todos.set(
                todos
                    .borrow()
                    .to_vec()
                    .into_iter()
                    .chain(vec![input.borrow().to_string()].into_iter())
                    .collect(),
            );
            input.set("".to_string());
        })
    };

    let oninput = {
        let input = input.clone();
        move |e: InputEvent| {
            let elm: HtmlInputElement = e.target_unchecked_into();
            input.set(elm.value());
        }
    };

    html! {
        <main>
            <input type="text" value={input.clone().to_string()} {oninput} />
            <button{onclick}>{ "add todo" }</button>
            <ul>
                { for todos.iter().map(|todo| html! { <li>{ todo }</li> }) }
            </ul>
        </main>
    }
}
