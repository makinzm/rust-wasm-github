use rust_wasm_github::demo::counter::Counter;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div>
            <h1>{ "Hello World" }</h1>
            <Counter />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
