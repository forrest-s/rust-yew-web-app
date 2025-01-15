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
        <>
            <header>
                <form id="newBountyForm" name="newBounty">
                    <label>{ "First Name" }</label>
                    <input type="text" value={input.clone().to_string()} name="firstName" placeholder="First Name" {oninput}/>
                    <label>{ "Last Name" }</label>
                    <input type="text" name="lastName" placeholder="Last Name" />
                    <label>{ "Living" }</label>
                    <select type="text" name="living" placeholder="First Name" >
                        <option>{ "is bounty alive" }</option>
                        <option value="true">{ "True" }</option>
                        <option value="false">{ "False" }</option>
                    </select>
                    <label>{ "Bounty Amount" }</label>
                    <input type="text" name="bountyAmount" placeholder="Bounty Amount" />
                    <label>{ "Type" }</label>
                    <select>
                        <option>{ "Jedi or Sith" }</option>
                        <option value="jedi">{ "Jedi" }</option>
                        <option value="sith">{ "Sith" }</option>
                    </select>   
                    <button{onclick}>{ "Enter Bounty" }</button>
                </form>
            </header>
            <main>

                <ul>
                    { for todos.iter().map(|todo| html! { <li>{ todo }</li> }) }
                </ul>
            </main>
        </>
    }
}
