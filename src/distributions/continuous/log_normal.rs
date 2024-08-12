use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use web_sys::HtmlCanvasElement;
use yew::prelude::*;

#[function_component(LogNormalDistribution)]
pub fn log_normal_distribution() -> Html {
    let mean = use_state(|| 0.0);
    let std_dev = use_state(|| 1.0);
    let canvas_ref = use_node_ref();

    {
        let mean: f64 = *mean;
        let std_dev: f64 = *std_dev;
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
                        "Mean: {:.2}, Std Dev: {:.2}",
                        (mean.exp() * (std_dev.powi(2) / 2.0).exp()),
                        ((std_dev.exp().powi(2) - 1.0) * (2.0 * mean + std_dev.exp().powi(2)))
                    );
                    let mut chart = ChartBuilder::on(&root)
                        .margin(10)
                        .caption(caption, ("sans-serif", 20))
                        .x_label_area_size(30)
                        .y_label_area_size(30)
                        .build_cartesian_2d(0.0..5.0, 0.0..2.0)
                        .unwrap();

                    chart.configure_mesh().draw().unwrap();

                    let log_normal_pdf = |x: f64, mean: f64, std_dev: f64| -> f64 {
                        if x <= 0.0 {
                            0.0
                        } else {
                            (1.0 / (x * std_dev * (2.0 * std::f64::consts::PI).sqrt()))
                                * (-((x.ln() - mean).powi(2)) / (2.0 * std_dev.powi(2))).exp()
                        }
                    };

                    chart
                        .draw_series(LineSeries::new(
                            (1..1000)
                                .map(|x| x as f64 / 200.0)
                                .map(|x| (x, log_normal_pdf(x, mean, std_dev))),
                            BLUE,
                        ))
                        .unwrap()
                        .label(format!("μ = {}, σ = {}", mean, std_dev))
                        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLUE));

                    chart
                        .configure_series_labels()
                        .border_style(BLACK)
                        .draw()
                        .unwrap();
                }
                || ()
            },
            (mean, std_dev),
        );
    }

    let on_mean_input = {
        let mean = mean.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                if let Ok(value) = input.value().parse::<f64>() {
                    mean.set(value);
                }
            }
        })
    };

    let on_std_dev_input = {
        let std_dev = std_dev.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                if let Ok(value) = input.value().parse::<f64>() {
                    std_dev.set(value);
                }
            }
        })
    };

    html! {
        <div style="width: 100%; max-width: 800px; margin: auto;">
            <h2>{ "Log-Normal Distribution" }</h2>
            <p> {"This distribution is used to model the distribution of a variable whose logarithm is normally distributed."} </p>
            <p> {"Probability Density Function: f(x; μ, σ) = (1 / (xσ√(2π))) * exp(-(ln(x) - μ)² / (2σ²))"} </p>
            <div>
                <label>{ "Mean (μ): " }</label>
                <input type="range" min="-3" max="3" step="0.01" value={(*mean).to_string()}
                oninput={on_mean_input} style="width: 70%;" />
                <span>{ format!("{:.2}", *mean) }</span>
            </div>
            <div>
                <label>{ "Standard Deviation (σ): " }</label>
                <input type="range" min="0.1" max="3" step="0.01" value={(*std_dev).to_string()}
                oninput={on_std_dev_input} style="width: 70%;" />
                <span>{ format!("{:.2}", *std_dev) }</span>
            </div>
            <p> {"Mean: exp(μ + σ²/2), Variance: (exp(σ²) - 1) * exp(2μ + σ²)"} </p>
            <canvas id="plot" ref={canvas_ref} style="width: 100%; height: auto;"></canvas>
        </div>
    }
}
