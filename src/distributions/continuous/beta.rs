use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use web_sys::HtmlCanvasElement;
use yew::prelude::*;

#[function_component(BetaDistribution)]
pub fn beta_distribution() -> Html {
    let alpha = use_state(|| 1.0);
    let beta = use_state(|| 1.0);
    let canvas_ref = use_node_ref();

    {
        let alpha: f64 = *alpha;
        let beta: f64 = *beta;
        let canvas_ref = canvas_ref.clone();
        use_effect_with_deps(
            move |_| {
                if let Some(canvas) = canvas_ref.cast::<HtmlCanvasElement>() {
                    // Set the canvas size to match its parent container
                    let parent = canvas.parent_element().unwrap();
                    let width = parent.client_width();
                    let height = (width as f64 * 0.75) as i32; // Maintain aspect ratio
                    canvas.set_width(width as u32);
                    canvas.set_height(height as u32);

                    let backend = CanvasBackend::with_canvas_object(canvas).unwrap();
                    let root = backend.into_drawing_area();
                    root.fill(&WHITE).unwrap();

                    let caption = format!(
                        "Mean: {:.2}, Variance: {:.2}",
                        alpha / (alpha + beta),
                        (alpha * beta) / ((alpha + beta).powi(2) * (alpha + beta + 1.0))
                    );
                    let mut chart = ChartBuilder::on(&root)
                        .margin(10)
                        .caption(caption, ("sans-serif", 20))
                        .x_label_area_size(30)
                        .y_label_area_size(30)
                        .build_cartesian_2d(0.0..1.0, 0.0..2.0)
                        .unwrap();

                    chart.configure_mesh().draw().unwrap();

                    let beta_pdf = |x: f64, alpha: f64, beta: f64| -> f64 {
                        if !(0.0..1.0).contains(&x) {
                            0.0
                        } else {
                            (x.powf(alpha - 1.0) * (1.0 - x).powf(beta - 1.0))
                                / (gamma(alpha) * gamma(beta) / gamma(alpha + beta))
                        }
                    };

                    chart
                        .draw_series(LineSeries::new(
                            (0..1000)
                                .map(|x| x as f64 / 1000.0)
                                .map(|x| (x, beta_pdf(x, alpha, beta))),
                            BLUE,
                        ))
                        .unwrap()
                        .label(format!("α = {}, β = {}", alpha, beta))
                        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLUE));

                    chart
                        .configure_series_labels()
                        .border_style(BLACK)
                        .draw()
                        .unwrap();
                }
                || ()
            },
            (alpha, beta),
        );
    }

    let on_alpha_input = {
        let alpha = alpha.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                if let Ok(value) = input.value().parse::<f64>() {
                    alpha.set(value);
                }
            }
        })
    };

    let on_beta_input = {
        let beta = beta.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                if let Ok(value) = input.value().parse::<f64>() {
                    beta.set(value);
                }
            }
        })
    };

    html! {
        <div style="width: 100%; max-width: 800px; margin: auto;">
            <h2>{ "Beta Distribution" }</h2>
            <p> {"This distribution is used to model the behavior of random variables limited to intervals of finite length in a variety of disciplines."} </p>
            <p> {"Probability Density Function: f(x; α, β) = (x^(α-1) * (1-x)^(β-1)) / B(α, β)"} </p>
            <div>
                <label>{ "Alpha (α): " }</label>
                <input type="range" min="0.01" max="10" step="0.01" value={(*alpha).to_string()}
                oninput={on_alpha_input} style="width: 70%;" />
                <span>{ format!("{:.2}", *alpha) }</span>
            </div>
            <div>
                <label>{ "Beta (β): " }</label>
                <input type="range" min="0.01" max="10" step="0.01" value={(*beta).to_string()}
                oninput={on_beta_input} style="width: 70%;" />
                <span>{ format!("{:.2}", *beta) }</span>
            </div>
            <p> {"Mean: α / (α + β), Variance: (αβ) / ((α + β)²(α + β + 1))"} </p>
            <p> {"Beta Function: B(α, β) = Γ(α)Γ(β) / Γ(α + β)"} </p>
            <canvas id="plot" ref={canvas_ref} style="width: 100%; height: auto;"></canvas>
        </div>
    }
}

// Helper function to compute the gamma function
fn gamma(x: f64) -> f64 {
    libm::tgamma(x)
}
