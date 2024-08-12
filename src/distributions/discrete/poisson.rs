use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use web_sys::HtmlCanvasElement;
use yew::prelude::*;

#[function_component(PoissonDistribution)]
pub fn poisson_distribution() -> Html {
    let lambda = use_state(|| 1.0);
    let canvas_ref = use_node_ref();

    {
        let lambda = *lambda;
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

                    let caption = format!("Mean and Variance: {:.2}", lambda);
                    let mut chart = ChartBuilder::on(&root)
                        .margin(10)
                        .caption(caption, ("sans-serif", 20))
                        .x_label_area_size(30)
                        .y_label_area_size(30)
                        .build_cartesian_2d(0..20, 0.0..1.0)
                        .unwrap();

                    chart.configure_mesh().draw().unwrap();

                    let poisson = |k: i32, lambda: f64| -> f64 {
                        let mut log_factorial = 0.0;
                        for i in 1..=k {
                            log_factorial += (i as f64).ln();
                        }
                        (-lambda + k as f64 * lambda.ln() - log_factorial).exp()
                    };

                    chart
                        .draw_series((0..20).map(|x| {
                            let y = poisson(x, lambda);
                            Rectangle::new([(x, 0.0), (x + 1, y)], RED.filled())
                        }))
                        .unwrap()
                        .label(format!("λ = {}", lambda))
                        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED));

                    chart
                        .configure_series_labels()
                        .border_style(BLACK)
                        .draw()
                        .unwrap();
                }
                || ()
            },
            lambda,
        );
    }

    let oninput = {
        let lambda = lambda.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                if let Ok(value) = input.value().parse::<f64>() {
                    lambda.set(value);
                }
            }
        })
    };

    html! {
        <div style="width: 100%; max-width: 800px; margin: auto;">
            <h2>{ "Poisson Distribution" }</h2>
            <p> {"This is used to model the small probability of rare events."} </p>
            <p> {"Probability Function: P(X=k) = (λ^k * e^(-λ)) / k!"} </p>
              <div>
                <label>{ "λ: " }</label>
                <input type="range" min="0.01" max="20" step="0.01" value={(*lambda).to_string()}
                oninput={oninput} style="width: 70%; " />
                <span>{ format!("{:.2}", *lambda) }</span>
            </div>
            <p> {"Mean and Variance: λ"} </p>
            <canvas id="plot" ref={canvas_ref} style="width: 100%; height: auto;"></canvas>
        </div>
    }
}
