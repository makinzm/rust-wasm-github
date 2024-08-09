use rust_wasm_github::distributions::discrete::poisson::PoissonDistribution;
use yew::prelude::*;

#[function_component(Distribution)]
pub fn distribution() -> Html {
    html! {
        <>
            <h1>{ "Distribution" }</h1>
            <div>
                <h2>{"🤯 Discrete Distributions" }</h2>
                <ul>
                    <li> <PoissonDistribution /> </li>
                </ul>
            </div>
        </>
    }
}
