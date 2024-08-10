use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use web_sys::HtmlCanvasElement;
use yew::prelude::*;

#[function_component(GeometricDistribution)]
pub fn geometric_distribution() -> Html {
    let p = use_state(|| 0.5);
    let canvas_ref = use_node_ref();

    {
        let p: f64 = *p;
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
                        "Geometric Distribution (p = {:.2}), Mean: {:.2}, Variance: {:.2}",
                        p,
                        1.0 / p,
                        (1.0 - p) / p.powi(2)
                    );

                    let mut chart = ChartBuilder::on(&root)
                        .margin(10)
                        .caption(caption, ("sans-serif", 20))
                        .x_label_area_size(30)
                        .y_label_area_size(30)
                        .build_cartesian_2d(1..20, 0.0..1.0)
                        .unwrap();

                    chart.configure_mesh().draw().unwrap();

                    let geometric = |k: i32, p: f64| -> f64 { (1.0 - p).powi(k - 1) * p };

                    chart
                        .draw_series((1..20).map(|x| {
                            let y = geometric(x, p);
                            Rectangle::new([(x, 0.0), (x + 1, y)], BLUE.filled())
                        }))
                        .unwrap()
                        .label(format!("p = {:.2}", p))
                        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLUE));

                    chart
                        .configure_series_labels()
                        .border_style(BLACK)
                        .draw()
                        .unwrap();
                }
                || ()
            },
            p,
        );
    }

    let oninput = {
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
            <h2>{ "Geometric Distribution" }</h2>
            <p> {"This models the number of trials needed to get the first success."} </p>
            <p> {"Probability Function: P(X=k) = (1-p)^(k-1) * p"} </p>
            <div>
                <label>{ "p: " }</label>
                <input type="range" min="0.01" max="1.0" step="0.01" value={(*p).to_string()}
                oninput={oninput} style="width: 70%; " />
                <span>{ format!("{:.2}", *p) }</span>
            </div>
            <p> {"Mean: 1/p, Variance: (1-p)/p²"} </p>
            <canvas id="plot" ref={canvas_ref} style="width: 100%; height: auto;"></canvas>
        </div>
    }
}
