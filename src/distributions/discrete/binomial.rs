use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use web_sys::HtmlCanvasElement;
use yew::prelude::*;

#[function_component(BinomialDistribution)]
pub fn binomial_distribution() -> Html {
    let n = use_state(|| 10);
    let p = use_state(|| 0.5);
    let canvas_ref = use_node_ref();

    {
        let n = *n;
        let p = *p;
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

                    let mean = n as f64 * p;
                    let variance = n as f64 * p * (1.0 - p);
                    let caption = format!("Mean(np): {:.2}, Variance: {:.2}", mean, variance);
                    let mut chart = ChartBuilder::on(&root)
                        .margin(10)
                        .caption(caption, ("sans-serif", 20))
                        .x_label_area_size(30)
                        .y_label_area_size(30)
                        .build_cartesian_2d(0..n + 1, 0.0..1.0)
                        .unwrap();

                    chart.configure_mesh().draw().unwrap();

                    let binomial = |k: i32, n: i32, p: f64| -> f64 {
                        let mut log_coeff = 0.0;
                        for i in 1..=n {
                            log_coeff += (i as f64).ln();
                        }
                        for i in 1..=k {
                            log_coeff -= (i as f64).ln();
                        }
                        for i in 1..=(n - k) {
                            log_coeff -= (i as f64).ln();
                        }
                        (log_coeff + k as f64 * p.ln() + (n - k) as f64 * (1.0 - p).ln()).exp()
                    };

                    chart
                        .draw_series((0..=n).map(|x| {
                            let y = binomial(x, n, p);
                            Rectangle::new([(x, 0.0), (x + 1, y)], BLUE.filled())
                        }))
                        .unwrap()
                        .label(format!("n = {}, p = {:.2}", n, p))
                        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLUE));

                    chart
                        .configure_series_labels()
                        .border_style(BLACK)
                        .draw()
                        .unwrap();
                }
                || ()
            },
            (n, p),
        );
    }

    let oninput_n = {
        let n = n.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                if let Ok(value) = input.value().parse::<i32>() {
                    n.set(value);
                }
            }
        })
    };

    let oninput_p = {
        let p = p.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                if let Ok(value) = input.value().parse::<f64>() {
                    p.set(value);
                }
            }
        })
    };

    html! {
        <div style="width: 100%; max-width: 800px; margin: auto;">
            <h2>{ "Binomial Distribution" }</h2>
            <p>{ "This distribution models the number of successes in a fixed number of independent Bernoulli trials." }</p>
            <p>{ "Probability Function: P(X=k) = C(n,k) * p^k * (1-p)^(n-k)" }</p>
            <div>
                <label>{ "n (number of trials): " }</label>
                <input type="range" min="1" max="500" step="1" value={n.to_string()}
                oninput={oninput_n} style="width: 70%;" />
                <span>{ *n }</span>
            </div>
            <div>
                <label>{ "p (probability of success): " }</label>
                <input type="range" min="0.001" max="0.999" step="0.001" value={p.to_string()}
                oninput={oninput_p} style="width: 70%;" />
                <span>{ format!("{:.2}", *p) }</span>
            </div>
            <p>{ "Mean: np, Variance: np(1-p)" }</p>
            <canvas id="plot" ref={canvas_ref} style="width: 100%; height: auto;"></canvas>
        </div>
    }
}
