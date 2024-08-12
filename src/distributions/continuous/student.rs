use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use web_sys::HtmlCanvasElement;
use yew::prelude::*;

#[function_component(TDistribution)]
pub fn t_distribution() -> Html {
    let degrees_of_freedom = use_state(|| 1.0);
    let canvas_ref = use_node_ref();

    {
        let degrees_of_freedom = *degrees_of_freedom;
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

                    let caption = if degrees_of_freedom == 1.0 {
                        "Cauchy Distribution".to_string()
                    } else {
                        format!(
                            "Mean: 0, Variance: {:.2}",
                            degrees_of_freedom / (degrees_of_freedom - 2.0)
                        )
                    };
                    let mut chart = ChartBuilder::on(&root)
                        .margin(10)
                        .caption(caption, ("sans-serif", 20))
                        .x_label_area_size(30)
                        .y_label_area_size(30)
                        .build_cartesian_2d(-5.0..5.0, 0.0..0.4)
                        .unwrap();

                    chart.configure_mesh().draw().unwrap();

                    let t_distribution_pdf = |x: f64, n: f64| -> f64 {
                        let gamma_half_n_plus_1 = gamma((n + 1.0) / 2.0);
                        let gamma_half_n = gamma(n / 2.0);
                        let sqrt_pi_n = (std::f64::consts::PI * n).sqrt();
                        let coefficient = gamma_half_n_plus_1 / (sqrt_pi_n * gamma_half_n);
                        let exponent = -(n + 1.0) / 2.0;
                        coefficient * (1.0 + (x * x) / n).powf(exponent)
                    };

                    chart
                        .draw_series(LineSeries::new(
                            (-500..500)
                                .map(|x| x as f64 / 100.0)
                                .map(|x| (x, t_distribution_pdf(x, degrees_of_freedom))),
                            RED,
                        ))
                        .unwrap()
                        .label(format!("n = {}", degrees_of_freedom))
                        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED));

                    chart
                        .configure_series_labels()
                        .border_style(BLACK)
                        .draw()
                        .unwrap();
                }
                || ()
            },
            (degrees_of_freedom,),
        );
    }

    let on_degrees_of_freedom_input = {
        let degrees_of_freedom = degrees_of_freedom.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                if let Ok(value) = input.value().parse::<f64>() {
                    degrees_of_freedom.set(value);
                }
            }
        })
    };

    html! {
        <div style="width: 100%; max-width: 800px; margin: auto;">
            <h2>{ "T-Distribution" }</h2>
            <p> {"This distribution is used to estimate population parameters when the sample size is small and/or the population variance is unknown."} </p>
            <p> {"Probability Density Function: f(x; n) = Γ((n+1)/2) / (√(nπ) * Γ(n/2)) * (1 + x²/n)^(-(n+1)/2)"} </p>
            <p> {"Mean: 0, Variance: n/(n-2) for n > 2"} </p>
            <div>
                <label>{ "Degrees of Freedom (n): " }</label>
                <input type="range" min="1" max="30" step="1" value={(*degrees_of_freedom).to_string()}
                oninput={on_degrees_of_freedom_input} style="width: 70%;" />
                <span>{ format!("{:.2}", *degrees_of_freedom) }</span>
            </div>
            <canvas id="plot" ref={canvas_ref} style="width: 100%; height: auto;"></canvas>
        </div>
    }
}

// Helper function to compute the gamma function
fn gamma(x: f64) -> f64 {
    libm::tgamma(x)
}
