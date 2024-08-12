use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use web_sys::HtmlCanvasElement;
use yew::prelude::*;

#[function_component(FDistribution)]
pub fn f_distribution() -> Html {
    let df1 = use_state(|| 1.0);
    let df2 = use_state(|| 1.0);
    let canvas_ref = use_node_ref();

    {
        let df1: f64 = *df1;
        let df2: f64 = *df2;
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

                    let caption = if df2 > 4.0 {
                        format!(
                            "Mean: {:.2}, Variance: {:.2}",
                            df2 / (df2 - 2.0),
                            (2.0 * df2.powf(2.0) * (df1 + df2 - 2.0))
                                / (df1 * (df2 - 2.0).powf(2.0) * (df2 - 4.0))
                        )
                    } else if df2 > 2.0 {
                        format!(
                            "Mean: {:.2}, Variance: {:.2}",
                            df2 / (df2 - 2.0),
                            f64::INFINITY
                        )
                    } else {
                        "Mean: ∞, Variance: ∞".to_string()
                    };

                    let mut chart = ChartBuilder::on(&root)
                        .margin(10)
                        .caption(caption, ("sans-serif", 20))
                        .x_label_area_size(30)
                        .y_label_area_size(30)
                        .build_cartesian_2d(0.0..5.0, 0.0..1.0)
                        .unwrap();

                    chart.configure_mesh().draw().unwrap();

                    let f_distribution_pdf = |x: f64, df1: f64, df2: f64| -> f64 {
                        let numerator = (df1 * x).powf(df1) * df2.powf(df2);
                        let denominator = (df1 * x + df2).powf(df1 + df2);
                        let beta = gamma(df1 / 2.0) * gamma(df2 / 2.0) / gamma((df1 + df2) / 2.0);
                        (numerator / denominator).sqrt() / (x * beta)
                    };

                    chart
                        .draw_series(LineSeries::new(
                            (1..500)
                                .map(|x| x as f64 / 100.0)
                                .map(|x| (x, f_distribution_pdf(x, df1, df2))),
                            RED,
                        ))
                        .unwrap()
                        .label(format!("df1 = {}, df2 = {}", df1, df2))
                        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED));

                    chart
                        .configure_series_labels()
                        .border_style(BLACK)
                        .draw()
                        .unwrap();
                }
                || ()
            },
            (df1, df2),
        );
    }

    let on_df1_input = {
        let df1 = df1.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                if let Ok(value) = input.value().parse::<f64>() {
                    df1.set(value);
                }
            }
        })
    };

    let on_df2_input = {
        let df2 = df2.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                if let Ok(value) = input.value().parse::<f64>() {
                    df2.set(value);
                }
            }
        })
    };

    html! {
        <div style="width: 100%; max-width: 800px; margin: auto;">
            <h2>{ "F-Distribution" }</h2>
            <p> {"This distribution is used to compare variances and is commonly used in ANOVA tests."} </p>
            <p> {"Probability Density Function: "} </p>
            <p> { "f(x; df1, df2) = √((df1 * x)^df1 * df2^df2 / (df1 * x + df2)^(df1 + df2)) ・ 1 / (x * B(df1/2, df2/2))"} </p>
            <div>
                <label>{ "Degrees of Freedom 1 (df1): " }</label>
                <input type="range" min="1" max="30" step="1" value={(*df1).to_string()}
                oninput={on_df1_input} style="width: 70%;" />
                <span>{ format!("{:.2}", *df1) }</span>
            </div>
            <div>
                <label>{ "Degrees of Freedom 2 (df2): " }</label>
                <input type="range" min="1" max="30" step="1" value={(*df2).to_string()}
                oninput={on_df2_input} style="width: 70%;" />
                <span>{ format!("{:.2}", *df2) }</span>
            </div>
            <canvas id="plot" ref={canvas_ref} style="width: 100%; height: auto;"></canvas>
        </div>
    }
}

// Helper function to compute the gamma function
fn gamma(x: f64) -> f64 {
    libm::tgamma(x)
}
