use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use web_sys::HtmlCanvasElement;
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

#[function_component(PoissonDistribution)]
fn poisson_distribution() -> Html {
    let lambda = use_state(|| 1.0);
    let canvas_ref = use_node_ref();

    {
        let lambda = *lambda;
        let canvas_ref = canvas_ref.clone();
        use_effect_with_deps(
            move |_| {
                if let Some(canvas) = canvas_ref.cast::<HtmlCanvasElement>() {
                    let backend = CanvasBackend::with_canvas_object(canvas).unwrap();
                    let root = backend.into_drawing_area();
                    root.fill(&WHITE).unwrap();

                    let mut chart = ChartBuilder::on(&root)
                        .margin(10)
                        .caption("Poisson Distribution", ("sans-serif", 20))
                        .x_label_area_size(30)
                        .y_label_area_size(30)
                        .build_cartesian_2d(0..20, 0.0..1.0) // Y-axis range remains 0.0 to 1.0
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
        <div>
            <h2>{ "Poisson Distribution" }</h2>
            <p> {"This is used to model the small probability of rare events."} </p>
            <p> {"P.D.F: P(X=k) = (λ^k * e^(-λ)) / k!"} </p>
            <input type="range" min="0.1" max="20" step="0.1" value={(*lambda).to_string()}
                oninput={oninput} style="width: 400px;" />
            <p>{ format!("λ: {}", *lambda) }</p>
            <canvas id="plot" width="800" height="600" ref={canvas_ref}></canvas>
        </div>
    }
}
