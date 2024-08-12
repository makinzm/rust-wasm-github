use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use web_sys::HtmlCanvasElement;
use yew::prelude::*;

#[function_component(WeibullDistribution)]
pub fn weibull_distribution() -> Html {
    let shape = use_state(|| 1.0);
    let scale = use_state(|| 1.0);
    let canvas_ref = use_node_ref();

    {
        let shape = *shape;
        let scale = *scale;
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
                        scale * gamma(1.0 + 1.0 / shape),
                        scale
                            * scale
                            * (gamma(1.0 + 2.0 / shape) - gamma(1.0 + 1.0 / shape).powi(2))
                    );
                    let mut chart = ChartBuilder::on(&root)
                        .margin(10)
                        .caption(caption, ("sans-serif", 20))
                        .x_label_area_size(30)
                        .y_label_area_size(30)
                        .build_cartesian_2d(0.0..5.0, 0.0..1.0)
                        .unwrap();

                    chart.configure_mesh().draw().unwrap();

                    let weibull = |x: f64, k: f64, lambda: f64| -> f64 {
                        if x < 0.0 {
                            0.0
                        } else {
                            (k / lambda)
                                * (x / lambda).powf(k - 1.0)
                                * (-(x / lambda).powf(k)).exp()
                        }
                    };

                    chart
                        .draw_series(LineSeries::new(
                            (0..500)
                                .map(|x| x as f64 / 100.0)
                                .map(|x| (x, weibull(x, shape, scale))),
                            RED,
                        ))
                        .unwrap()
                        .label(format!("k = {}, λ = {}", shape, scale))
                        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED));

                    chart
                        .configure_series_labels()
                        .border_style(BLACK)
                        .draw()
                        .unwrap();
                }
                || ()
            },
            (shape, scale),
        );
    }

    let on_shape_input = {
        let shape = shape.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                if let Ok(value) = input.value().parse::<f64>() {
                    shape.set(value);
                }
            }
        })
    };

    let on_scale_input = {
        let scale = scale.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                if let Ok(value) = input.value().parse::<f64>() {
                    scale.set(value);
                }
            }
        })
    };

    html! {
        <div style="width: 100%; max-width: 800px; margin: auto;">
            <h2>{ "Weibull Distribution" }</h2>
            <p> {"This distribution is used to model life data and reliability."} </p>
            <p> {"Probability Density Function: f(x; k, λ) = (k/λ) * (x/λ)^(k-1) * e^(-(x/λ)^k)"} </p>
            <div>
                <label>{ "Shape (k): " }</label>
                <input type="range" min="0.01" max="10" step="0.01" value={(*shape).to_string()}
                oninput={on_shape_input} style="width: 70%;" />
                <span>{ format!("{:.2}", *shape) }</span>
            </div>
            <div>
                <label>{ "Scale (λ): " }</label>
                <input type="range" min="0.01" max="10" step="0.01" value={(*scale).to_string()}
                oninput={on_scale_input} style="width: 70%;" />
                <span>{ format!("{:.2}", *scale) }</span>
            </div>
            <p> {"Mean: λ * Γ(1 + 1/k), Variance: λ² * (Γ(1 + 2/k) - (Γ(1 + 1/k))²)"} </p>
            <p> {" Gamma Function: Γ(x) = ∫^∞_{0} t^{x-1}exp(-t) dt " } </p>
            <canvas id="plot" ref={canvas_ref} style="width: 100%; height: auto;"></canvas>
        </div>
    }
}

// Helper function to compute the gamma function
fn gamma(x: f64) -> f64 {
    libm::tgamma(x)
}
