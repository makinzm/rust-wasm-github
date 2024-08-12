use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use web_sys::HtmlCanvasElement;
use yew::prelude::*;

#[function_component(ChiSquaredDistribution)]
pub fn chi_squared_distribution() -> Html {
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

                    let mean = degrees_of_freedom;
                    let variance = 2.0 * degrees_of_freedom;
                    let caption = format!("Mean: {:.2}, Variance: {:.2}", mean, variance);
                    let mut chart = ChartBuilder::on(&root)
                        .margin(10)
                        .caption(caption, ("sans-serif", 20))
                        .x_label_area_size(30)
                        .y_label_area_size(30)
                        .build_cartesian_2d(0.0..20.0, 0.0..0.5)
                        .unwrap();

                    chart.configure_mesh().draw().unwrap();

                    let chi_squared_pdf = |x: f64, n: f64| -> f64 {
                        if x < 0.0 {
                            0.0
                        } else {
                            let k = n / 2.0;
                            let lambda: f64 = 0.5;
                            (lambda.powf(k) * x.powf(k - 1.0) * (-lambda * x).exp()) / gamma(k)
                        }
                    };

                    chart
                        .draw_series(LineSeries::new(
                            (0..2000)
                                .map(|x| x as f64 / 100.0)
                                .map(|x| (x, chi_squared_pdf(x, degrees_of_freedom))),
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
            <h2>{ "Chi-Squared Distribution" }</h2>
            <p> {"This distribution is used to model the sum of the squares of k independent standard normal random variables."} </p>
            <p> {"Probability Density Function: f(x; n) = (1/(2^(n/2) * Γ(n/2))) * x^(n/2 - 1) * e^(-x/2)"} </p>
            <div>
                <label>{ "Degrees of Freedom (n): " }</label>
                <input type="range" min="1" max="20" step="1" value={(*degrees_of_freedom).to_string()}
                oninput={on_degrees_of_freedom_input} style="width: 70%;" />
                <span>{ format!("{:.2}", *degrees_of_freedom) }</span>
            </div>
            <p> {"Mean: n, Variance: 2n"} </p>
            <canvas id="plot" ref={canvas_ref} style="width: 100%; height: auto;"></canvas>
        </div>
    }
}

// Helper function to compute the gamma function
fn gamma(x: f64) -> f64 {
    libm::tgamma(x)
}
