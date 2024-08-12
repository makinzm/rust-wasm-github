use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use web_sys::HtmlCanvasElement;
use yew::prelude::*;

const FONT_SIZE_THRESHOLD: i32 = 300;
const DESKTOP_FONT_SIZE: i32 = 20;
const MOBILE_FONT_SIZE: i32 = 10;

#[function_component(HypergeometricDistribution)]
pub fn hypergeometric_distribution() -> Html {
    let n = use_state(|| 50); // Total population size
    let m = use_state(|| 20); // Number of success states in the population
    let k = use_state(|| 10); // Number of draws
    let canvas_ref = use_node_ref();

    {
        let n: i32 = *n;
        let m: i32 = *m;
        let k: i32 = *k;
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
                        "Hypergeometric Distribution (N = {}, M = {}, K = {}), Mean: {:.2}, Variance: {:.2}",
                        n, m, k,
                        (k * m) as f64 / n as f64,
                        (k * m * (n - m) * (n - k)) as f64 / (n * n * (n - 1)) as f64
                    );

                    let fontsize = if width > FONT_SIZE_THRESHOLD {
                        DESKTOP_FONT_SIZE
                    } else {
                        MOBILE_FONT_SIZE
                    };

                    let mut chart = ChartBuilder::on(&root)
                        .margin(10)
                        .caption(caption, ("sans-serif", fontsize))
                        .x_label_area_size(30)
                        .y_label_area_size(30)
                        .build_cartesian_2d(0..k + 1, 0.0..1.0)
                        .unwrap();

                    chart.configure_mesh().draw().unwrap();

                    chart
                        .draw_series((0..=k).map(|x| {
                            let y = hypergeometric(x, n, m, k);
                            Rectangle::new([(x, 0.0), (x + 1, y)], GREEN.filled())
                        }))
                        .unwrap()
                        .label(format!("N = {}, M = {}, K = {}", n, m, k))
                        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], GREEN));

                    chart
                        .configure_series_labels()
                        .border_style(BLACK)
                        .draw()
                        .unwrap();
                }
                || ()
            },
            (n, m, k),
        );
    }

    let oninput_n = {
        let n = n.clone();
        let m = m.clone();
        let k = k.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                if let Ok(value) = input.value().parse::<i32>() {
                    n.set(value);
                    if *m > value {
                        m.set(value);
                    }
                    if *k > value {
                        k.set(value);
                    }
                }
            }
        })
    };

    let oninput_m = {
        let m = m.clone();
        let n = n.clone();
        let k = k.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                if let Ok(value) = input.value().parse::<i32>() {
                    if value <= *n && value >= 0 {
                        m.set(value);
                        if *k > value {
                            k.set(value);
                        }
                    }
                }
            }
        })
    };

    let oninput_k = {
        let k = k.clone();
        let n = n.clone();
        let m = m.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                if let Ok(value) = input.value().parse::<i32>() {
                    let max_k = std::cmp::min(*n, *m);
                    if value <= max_k {
                        k.set(value);
                    }
                }
            }
        })
    };

    html! {
        <div style="width: 100%; max-width: 800px; margin: auto;">
            <h2>{ "Hypergeometric Distribution" }</h2>
            <p> {"This models the number of successes in a sequence of draws without replacement."} </p>
            <p> {"Probability Function: P(X=x) = C(M,x) * C(N-M,K-x) / C(N,K)"} </p>
            <p> {"Mean: M * K / N, Variance: M * (N-M) * K * (N-K) / N^2 / (N-1)"} </p>
            <p> {"WARNING: N >= M >= K >= 0"} </p>
            <div>
                <label>{ "N: " }</label>
                <input type="range" min={std::cmp::max(*m, 1).to_string()} max="100" step="1" value={(*n).to_string()}
                oninput={oninput_n} style="width: 70%; " />
                <span>{ format!("{}", *n) }</span>
            </div>
            <div>
                <label>{ "M: " }</label>
                <input type="range" min={std::cmp::max(*k, 0).to_string()} max={(*n).to_string()} step="1" value={(*m).to_string()}
                oninput={oninput_m} style="width: 70%; " />
                <span>{ format!("{}", *m) }</span>
            </div>
            <div>
                <label>{ "K: " }</label>
                <input type="range" min="0" max={std::cmp::min(*n, *m).to_string()} step="1" value={(*k).to_string()}
                oninput={oninput_k} style="width: 70%; " />
                <span>{ format!("{}", *k) }</span>
            </div>
            <canvas id="plot" ref={canvas_ref} style="width: 100%; height: auto;"></canvas>
        </div>
    }
}

fn hypergeometric(x: i32, n: i32, m: i32, k: i32) -> f64 {
    let log_comb = |n: i32, k: i32| -> f64 {
        if k == 0 || k == n {
            return 0.0;
        }
        let log_numerator = (n - k + 1..=n).fold(0.0, |acc, x| acc + (x as f64).ln());
        let log_denominator = (1..=k).fold(0.0, |acc, x| acc + (x as f64).ln());
        log_numerator - log_denominator
    };

    let log_prob: f64 = log_comb(m, x) + log_comb(n - m, k - x) - log_comb(n, k);
    log_prob.exp()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hypergeometric() {
        // Test edge cases
        assert!((hypergeometric(0, 1, 1, 1) - 0.0).abs() < 1e-6);
        assert!((hypergeometric(1, 1, 1, 1) - 1.0).abs() < 1e-6);
    }
}
