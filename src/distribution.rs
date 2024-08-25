use rust_wasm_github::distributions::continuous::beta::BetaDistribution;
use rust_wasm_github::distributions::continuous::bivariate_normal::BivariateNormalDistribution;
use rust_wasm_github::distributions::continuous::chi_squared::ChiSquaredDistribution;
use rust_wasm_github::distributions::continuous::exponential::ExponentialDistribution;
use rust_wasm_github::distributions::continuous::f::FDistribution;
use rust_wasm_github::distributions::continuous::gamma::GammaDistribution;
use rust_wasm_github::distributions::continuous::log_normal::LogNormalDistribution;
use rust_wasm_github::distributions::continuous::student::TDistribution;
use rust_wasm_github::distributions::continuous::WeibullDistribution;
use rust_wasm_github::distributions::discrete::binomial::BinomialDistribution;
use rust_wasm_github::distributions::discrete::geometric::GeometricDistribution;
use rust_wasm_github::distributions::discrete::hypergeometric::HypergeometricDistribution;
use rust_wasm_github::distributions::discrete::negative_binomial::NegativeBinomialDistribution;
use rust_wasm_github::distributions::discrete::poisson::PoissonDistribution;

use yew::prelude::*;

#[derive(Properties, PartialEq)]
struct DistributionProps {
    name: &'static str,
    children: Children,
    global_visibility: bool,
}

#[function_component(DistributionItem)]
fn distribution_item(props: &DistributionProps) -> Html {
    let visible = use_state(|| false);

    // Update visibility based on global visibility changes
    {
        let visible = visible.clone();
        let global_visibility = props.global_visibility;
        use_effect_with_deps(
            move |_| {
                visible.set(global_visibility);
                || ()
            },
            props.global_visibility,
        );
    }

    let toggle_visibility = {
        let visible = visible.clone();
        Callback::from(move |_| {
            visible.set(!*visible);
        })
    };

    let is_visible = *visible;

    html! {
        <li>
            <div onclick={toggle_visibility} style="cursor: pointer;">
                { if is_visible { format!("{} ▼", props.name) } else { format!("{} ►", props.name) } }
            </div>
            if is_visible {
                <div style="margin-left: 20px;">
                    { for props.children.iter() }
                </div>
            }
        </li>
    }
}

#[function_component(Distribution)]
pub fn distribution() -> Html {
    let global_visibility = use_state(|| false);
    let toggle_global_visibility = {
        let global_visibility = global_visibility.clone();
        Callback::from(move |_| global_visibility.set(!*global_visibility))
    };

    let background_color = if *global_visibility {
        "lightgreen"
    } else {
        "lightcoral"
    };
    let button_style = format!(
        "
        position: fixed;
        right: 20px;
        top: 120px;
        background-color: {};
        border: none;
        border-radius: 5px;
        cursor: pointer;
        width: 100px;
        height: 50px;
        color: white;
        font-size: 16px;
        font-weight: bold;
        ",
        background_color
    );

    html! {
        <>
            <h1>{ "Distribution" }</h1>
            <div style="position: relative;">
                <button onclick={toggle_global_visibility} style={button_style}>
                    { if *global_visibility { "Close" } else { "Open" } }
                </button>
                <h2>{"🤯 Discrete Distributions" }</h2>
                <ul>
                    <DistributionItem name="Binomial Distribution" global_visibility={*global_visibility}>
                        <BinomialDistribution />
                    </DistributionItem>
                    <DistributionItem name="Poisson Distribution" global_visibility={*global_visibility}>
                        <PoissonDistribution />
                    </DistributionItem>
                    <DistributionItem name="Geometric Distribution" global_visibility={*global_visibility}>
                        <GeometricDistribution />
                    </DistributionItem>
                    <DistributionItem name="Negative Binomial Distribution" global_visibility={*global_visibility}>
                        <NegativeBinomialDistribution />
                    </DistributionItem>
                    <DistributionItem name="Hypergeometric Distribution" global_visibility={*global_visibility}>
                        <HypergeometricDistribution />
                    </DistributionItem>
                </ul>
                <h2>{"😂 Continuous Distributions" }</h2>
                <ul>
                    <DistributionItem name="Exponential Distribution" global_visibility={*global_visibility}>
                        <ExponentialDistribution />
                    </DistributionItem>
                    <DistributionItem name="Weibull Distribution" global_visibility={*global_visibility}>
                        <WeibullDistribution />
                    </DistributionItem>
                    <DistributionItem name="Gamma Distribution" global_visibility={*global_visibility}>
                        <GammaDistribution />
                    </DistributionItem>
                    <DistributionItem name="Beta Distribution" global_visibility={*global_visibility}>
                        <BetaDistribution />
                    </DistributionItem>
                    <DistributionItem name="Chi Squared Distribution" global_visibility={*global_visibility}>
                        <ChiSquaredDistribution />
                    </DistributionItem>
                    <DistributionItem name="Student's T Distribution" global_visibility={*global_visibility}>
                        <TDistribution />
                    </DistributionItem>
                    <DistributionItem name="F Distribution" global_visibility={*global_visibility}>
                        <FDistribution />
                    </DistributionItem>
                    <DistributionItem name="Log Normal Distribution" global_visibility={*global_visibility}>
                        <LogNormalDistribution />
                    </DistributionItem>
                    <DistributionItem name="Bivariate Normal Distribution" global_visibility={*global_visibility}>
                        <BivariateNormalDistribution />
                    </DistributionItem>
                </ul>
            </div>
        </>
    }
}
