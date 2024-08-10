use rust_wasm_github::distributions::continuous::bivariate_normal::BivariateNormalDistribution;
use rust_wasm_github::distributions::discrete::geometric::GeometricDistribution;
use rust_wasm_github::distributions::discrete::negative_binomial::NegativeBinomialDistribution;
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
                    <li> <GeometricDistribution /> </li>
                    <li> <NegativeBinomialDistribution /> </li>
                </ul>
                <h2>{"😂 Continuous Distributions" }</h2>
                <ul>
                    <li> <BivariateNormalDistribution /> </li>
                </ul>
            </div>
        </>
    }
}
