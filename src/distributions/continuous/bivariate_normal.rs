use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use web_sys::HtmlCanvasElement;
use yew::prelude::*;

// TODO
// - Make 3D Graph more understandable
// - Improve Client Side Performance

#[function_component(BivariateNormalDistribution)]
pub fn bivariate_normal_distribution() -> Html {
    let mean_x = use_state(|| 0.0);
    let mean_y = use_state(|| 0.0);
    let variance_x = use_state(|| 1.0);
    let variance_y = use_state(|| 1.0);
    let correlation = use_state(|| 0.0);
    let conditional_x = use_state(|| 0.0);
    let canvas_ref_contour = use_node_ref();
    let canvas_ref_conditional = use_node_ref();
    let conditional_mean_y = use_state(|| 0.0);
    let conditional_variance_y = use_state(|| 0.0);

    {
        let mean_x = *mean_x;
        let mean_y = *mean_y;
        let variance_x: f64 = *variance_x;
        let variance_y: f64 = *variance_y;
        let correlation: f64 = *correlation;
        let conditional_x: f64 = *conditional_x;
        let canvas_ref_contour = canvas_ref_contour.clone();
        let canvas_ref_conditional = canvas_ref_conditional.clone();
        let conditional_mean_y = conditional_mean_y.clone();
        let conditional_variance_y = conditional_variance_y.clone();

        use_effect_with_deps(
            move |_| {
                if let Some(canvas) = canvas_ref_contour.cast::<HtmlCanvasElement>() {
                    let parent = canvas.parent_element().unwrap();
                    let width = parent.client_width();
                    let height = (width as f64 * 0.75) as i32;
                    canvas.set_width(width as u32);
                    canvas.set_height(height as u32);

                    let backend = CanvasBackend::with_canvas_object(canvas).unwrap();
                    let root = backend.into_drawing_area();
                    root.fill(&WHITE).unwrap();

                    let mut chart = ChartBuilder::on(&root)
                        .margin(10)
                        .caption("Bivariate Normal Distribution", ("sans-serif", 20))
                        .x_label_area_size(30)
                        .y_label_area_size(30)
                        .build_cartesian_3d(-3.0..3.0, 0.0..0.5, -3.0..3.0)
                        .unwrap();

                    chart
                        .configure_axes()
                        .light_grid_style(WHITE.mix(0.8))
                        .max_light_lines(3)
                        .draw()
                        .unwrap();

                    let pdf = |x: f64, y: f64| -> f64 {
                        let std_dev_x = variance_x.sqrt();
                        let std_dev_y = variance_y.sqrt();
                        let rho = correlation;

                        let z = (x - mean_x) / std_dev_x;
                        let w = (y - mean_y) / std_dev_y;

                        let rho2 = rho.powi(2);
                        let z2 = z.powi(2);
                        let w2 = w.powi(2);

                        let exp = (-0.5 / (1.0 - rho2) * (z2 - 2.0 * rho * z * w + w2)).exp();
                        let norm = 1.0
                            / (2.0
                                * std_dev_x
                                * std_dev_y
                                * (1.0 - rho2).sqrt()
                                * std::f64::consts::PI);

                        norm * exp
                    };

                    chart
                        .draw_series(SurfaceSeries::xoz(
                            (-300..=300).map(|i| -3.0 + 6.0 * i as f64 / 300.0),
                            (-300..=300).map(|i| -3.0 + 6.0 * i as f64 / 300.0),
                            pdf,
                        ))
                        .unwrap();
                }

                if let Some(canvas) = canvas_ref_conditional.cast::<HtmlCanvasElement>() {
                    let parent = canvas.parent_element().unwrap();
                    let width = parent.client_width();
                    let height = (width as f64 * 0.75) as i32;
                    canvas.set_width(width as u32);
                    canvas.set_height(height as u32);

                    let backend = CanvasBackend::with_canvas_object(canvas).unwrap();
                    let root = backend.into_drawing_area();
                    root.fill(&WHITE).unwrap();

                    let mut chart = ChartBuilder::on(&root)
                        .margin(10)
                        .caption("Conditional Distribution of Y given X", ("sans-serif", 20))
                        .x_label_area_size(30)
                        .y_label_area_size(30)
                        .build_cartesian_2d(-3.0..3.0, 0.0..1.0)
                        .unwrap();

                    chart.configure_mesh().draw().unwrap();

                    let new_conditional_mean_y = mean_y
                        + correlation * (variance_y / variance_x).sqrt() * (conditional_x - mean_x);
                    conditional_mean_y.set(new_conditional_mean_y);

                    let new_conditional_variance_y = variance_y * (1.0 - correlation.powi(2));
                    conditional_variance_y.set(new_conditional_variance_y);

                    let normal_pdf = |x: f64, mean: f64, variance: f64| -> f64 {
                        let sigma = variance.sqrt();
                        let z = (x - mean) / sigma;
                        (-0.5 * z.powi(2)).exp() / (sigma * (2.0 * std::f64::consts::PI).sqrt())
                    };

                    chart
                        .draw_series((0..=100).map(|i| {
                            let y = -3.0 + 6.0 * i as f64 / 100.0;
                            let mean = new_conditional_mean_y;
                            let variance = new_conditional_variance_y;
                            let pdf = normal_pdf(y, mean, variance);
                            Circle::new((y, pdf), 1, RED)
                        }))
                        .unwrap();
                }

                || ()
            },
            (
                mean_x,
                mean_y,
                variance_x,
                variance_y,
                correlation,
                conditional_x,
            ),
        );
    }

    let oninput_mean_x = {
        let mean_x = mean_x.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                if let Ok(value) = input.value().parse::<f64>() {
                    mean_x.set(value);
                }
            }
        })
    };

    let oninput_mean_y = {
        let mean_y = mean_y.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                if let Ok(value) = input.value().parse::<f64>() {
                    mean_y.set(value);
                }
            }
        })
    };

    let oninput_variance_x = {
        let variance_x = variance_x.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                if let Ok(value) = input.value().parse::<f64>() {
                    variance_x.set(value);
                }
            }
        })
    };

    let oninput_variance_y = {
        let variance_y = variance_y.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                if let Ok(value) = input.value().parse::<f64>() {
                    variance_y.set(value);
                }
            }
        })
    };

    let oninput_correlation = {
        let correlation = correlation.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                if let Ok(value) = input.value().parse::<f64>() {
                    correlation.set(value);
                }
            }
        })
    };

    let oninput_conditional_x = {
        let conditional_x = conditional_x.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                if let Ok(value) = input.value().parse::<f64>() {
                    conditional_x.set(value);
                }
            }
        })
    };

    html! {
        <div style="width: 100%; max-width: 800px; margin: auto; display: flex; flex-wrap: wrap;">
            <div style="flex: 1 1 100%; margin-bottom: 20px;">
                <h2>{ "Bivariate Normal Distribution" }</h2>
                <p> {"This distribution models the joint behavior of two normally distributed random variables."} </p>
                <div>
                    <label>{ "Mean of X: " }</label>
                    <input type="range" min="-3" max="3" step="0.1" value={(*mean_x).to_string()}
                        oninput={oninput_mean_x} style="width: 70%; " />
                    <span>{ format!("{:.1}", *mean_x) }</span>
                </div>
                <div>
                    <label>{ "Mean of Y: " }</label>
                    <input type="range" min="-3" max="3" step="0.1" value={(*mean_y).to_string()}
                        oninput={oninput_mean_y} style="width: 70%; " />
                    <span>{ format!("{:.1}", *mean_y) }</span>
                </div>
                <div>
                    <label>{ "Variance of X: " }</label>
                    <input type="range" min="0.1" max="3" step="0.1" value={(*variance_x).to_string()}
                        oninput={oninput_variance_x} style="width: 70%; " />
                    <span>{ format!("{:.1}", *variance_x) }</span>
                </div>
                <div>
                    <label>{ "Variance of Y: " }</label>
                    <input type="range" min="0.1" max="3" step="0.1" value={(*variance_y).to_string()}
                        oninput={oninput_variance_y} style="width: 70%; " />
                    <span>{ format!("{:.1}", *variance_y) }</span>
                </div>
                <div>
                    <label>{ "Correlation: " }</label>
                    <input type="range" min="-1" max="1" step="0.01" value={(*correlation).to_string()}
                        oninput={oninput_correlation} style="width: 70%; " />
                    <span>{ format!("{:.2}", *correlation) }</span>
                </div>
                <div>
                    <label>{ "Conditional X: " }</label>
                    <input type="range" min="-3" max="3" step="0.1" value={(*conditional_x).to_string()}
                        oninput={oninput_conditional_x} style="width: 70%; " />
                    <span>{ format!("{:.1}", *conditional_x) }</span>
                </div>
            </div>
            <div style="flex: 1 1 50%; padding: 10px;">
                <canvas id="contour-plot" ref={canvas_ref_contour} style="width: 100%; height: auto;"></canvas>
            </div>
            <div style="flex: 1 1 50%; padding: 10px;">
                <p>
                    { "Conditional Mean of Y: " }
                    { format!("{:.2}", *conditional_mean_y) }
                </p>
                <p>
                    { "Conditional Variance of Y: " }
                    { format!("{:.2}", *conditional_variance_y) }
                </p>
                <canvas id="conditional-plot" ref={canvas_ref_conditional} style="width: 100%; height: auto;"></canvas>
            </div>
        </div>
    }
}
