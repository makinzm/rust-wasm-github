use yew::prelude::*;

#[function_component(Counter)]
pub fn counter() -> Html {
    let counter = use_state(|| 0);

    let increment = {
        let counter = counter.clone();
        Callback::from(move |_| {
            let value = *counter + 1;
            counter.set(value);
        })
    };

    let decrement = {
        let counter = counter.clone();
        Callback::from(move |_| {
            let value = (*counter - 1).max(0);
            counter.set(value);
        })
    };

    html! {
        <div>
            <p>{ "Current count: " }{ *counter }</p>
            <button onclick={increment}>{ "Increment" }</button>
            <button onclick={decrement}>{ "Decrement" }</button>
        </div>
    }
}
