use rust_wasm_github::distributions::continuous::bivariate_normal::BivariateNormalDistribution;
use rust_wasm_github::distributions::continuous::exponential::ExponentialDistribution;
use rust_wasm_github::distributions::continuous::WeibullDistribution;
use rust_wasm_github::distributions::discrete::geometric::GeometricDistribution;
use rust_wasm_github::distributions::discrete::hypergeometric::HypergeometricDistribution;
use rust_wasm_github::distributions::discrete::negative_binomial::NegativeBinomialDistribution;
use rust_wasm_github::distributions::discrete::poisson::PoissonDistribution;

use yew::prelude::*;

#[derive(Properties, PartialEq)]
struct DistributionProps {
    name: &'static str,
    children: Children,
}

#[function_component(DistributionItem)]
fn distribution_item(props: &DistributionProps) -> Html {
    let visible = use_state(|| false);
    let toggle_visibility = {
        let visible = visible.clone();
        Callback::from(move |_| visible.set(!*visible))
    };

    html! {
        <li>
            <div onclick={toggle_visibility} style="cursor: pointer;">
                { if *visible { format!("{} ▼", props.name) } else { format!("{} ►", props.name) } }
            </div>
            if *visible {
                <div style="margin-left: 20px;">
                    { for props.children.iter() }
                </div>
            }
        </li>
    }
}

#[function_component(Distribution)]
pub fn distribution() -> Html {
    html! {
        <>
            <h1>{ "Distribution" }</h1>
            <div>
                <h2>{"🤯 Discrete Distributions" }</h2>
                <ul>
                    <DistributionItem name="Poisson Distribution">
                        <PoissonDistribution />
                    </DistributionItem>
                    <DistributionItem name="Geometric Distribution">
                        <GeometricDistribution />
                    </DistributionItem>
                    <DistributionItem name="Negative Binomial Distribution">
                        <NegativeBinomialDistribution />
                    </DistributionItem>
                    <DistributionItem name="Hypergeometric Distribution">
                        <HypergeometricDistribution />
                    </DistributionItem>
                </ul>
                <h2>{"😂 Continuous Distributions" }</h2>
                <ul>
                    <DistributionItem name="Exponential Distribution">
                        <ExponentialDistribution />
                    </DistributionItem>
                    <DistributionItem name="Weibull Distribution">
                        <WeibullDistribution />
                    </DistributionItem>
                    <DistributionItem name="Bivariate Normal Distribution">
                        <BivariateNormalDistribution />
                    </DistributionItem>
                </ul>
            </div>
        </>
    }
}
