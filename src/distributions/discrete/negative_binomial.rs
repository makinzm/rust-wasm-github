use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use web_sys::HtmlCanvasElement;
use yew::prelude::*;

#[function_component(NegativeBinomialDistribution)]
pub fn negative_binomial_distribution() -> Html {
    let p = use_state(|| 0.5);
    let r = use_state(|| 3);
    let canvas_ref = use_node_ref();

    {
        let p: f64 = *p;
        let r: i32 = *r;
        let canvas_ref = canvas_ref.clone();
        use_effect_with_deps(
            move |_| {
                if let Some(canvas) = canvas_ref.cast::<HtmlCanvasElement>() {
                    let parent = canvas.parent_element().unwrap();
                    let width = parent.client_width();
                    let height = (width as f64 * 0.75) as i32;
                    canvas.set_width(width as u32);
                    canvas.set_height(height as u32);

                    let backend = CanvasBackend::with_canvas_object(canvas).unwrap();
                    let root = backend.into_drawing_area();
                    root.fill(&WHITE).unwrap();

                    let caption = format!(
                        "Mean: {:.2}, Variance: {:.2}",
                        r as f64 / p,
                        r as f64 * (1.0 - p) / p.powi(2)
                    );

                    let mut chart = ChartBuilder::on(&root)
                        .margin(10)
                        .caption(caption, ("sans-serif", 20))
                        .x_label_area_size(30)
                        .y_label_area_size(30)
                        .build_cartesian_2d(0..50, 0.0..0.3)
                        .unwrap();

                    chart.configure_mesh().draw().unwrap();

                    chart
                        .draw_series((0..50).map(|k| {
                            let y = negative_binomial(k, r, p);
                            Rectangle::new([(k, 0.0), (k + 1, y)], CYAN.filled())
                        }))
                        .unwrap()
                        .label(format!("p = {:.2}, r = {}", p, r))
                        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], CYAN));

                    chart
                        .configure_series_labels()
                        .border_style(BLACK)
                        .draw()
                        .unwrap();
                }
                || ()
            },
            (p, r),
        );
    }

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

    let oninput_r = {
        let r = r.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                if let Ok(value) = input.value().parse::<i32>() {
                    r.set(value);
                }
            }
        })
    };

    html! {
        <div style="width: 100%; max-width: 800px; margin: auto;">
            <h2>{ "Negative Binomial Distribution" }</h2>
            <p> {"This models the number of trials needed to achieve a specified number of successes."} </p>
            <p> {"Probability Function: P(X=k) = C(r+k-1, k) * (1-p)^k * p^r"} </p>
            <div>
                <label>{ "p: " }</label>
                <input type="range" min="0.01" max="0.99" step="0.01" value={(*p).to_string()}
                oninput={oninput_p} style="width: 70%; " />
                <span>{ format!("{:.2}", *p) }</span>
            </div>
            <div>
                <label>{ "r: " }</label>
                <input type="range" min="1" max="10" step="1" value={(*r).to_string()}
                oninput={oninput_r} style="width: 70%; " />
                <span>{ format!("{}", *r) }</span>
            </div>
            <p> {"Mean: r/p, Variance: r(1-p)/p²"} </p>
            <canvas id="plot" ref={canvas_ref} style="width: 100%; height: auto;"></canvas>
        </div>
    }
}

fn negative_binomial(k: i32, r: i32, p: f64) -> f64 {
    // Calculate the natural logarithm of the combination to prevent overflow
    let log_comb = |n: i32, k: i32| -> f64 {
        if k == 0 || k == n {
            return 0.0;
        }
        let log_numerator = (n - k + 1..=n).fold(0.0, |acc, x| acc + (x as f64).ln());
        let log_denominator = (1..=k).fold(0.0, |acc, x| acc + (x as f64).ln());
        log_numerator - log_denominator
    };

    // Use the log_comb to calculate the probability
    let log_prob: f64 = log_comb(r + k - 1, k) + (k as f64) * (1.0 - p).ln() + (r as f64) * p.ln();

    log_prob.exp() // Convert back from log space
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_negative_binomial() {
        // Too Simple test
        assert!((negative_binomial(0, 1, 0.5) < 1.0));

        // Test with known values
        assert!((negative_binomial(2, 3, 0.5) - (3.0 / 16.0 as f64)).abs() < 1e-6);
        assert!((negative_binomial(1, 3, 0.5) - 0.1875).abs() < 1e-6);

        // Test edge cases
        assert!((negative_binomial(0, 1, 0.5) - 0.5).abs() < 1e-6);
        assert!((negative_binomial(0, 0, 0.5) - 1.0).abs() < 1e-6);
        assert!((negative_binomial(0, 1, 0.99) - 1.0).abs() < 0.1);
    }
}
