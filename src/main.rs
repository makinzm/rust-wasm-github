use rust_wasm_github::demo::counter::Counter;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div class="bg-white dark:bg-gray-800 dark:text-white">
            <h1 class="text-5xl text-center font-bold p-8">
                { "Hello World" }
            </h1>
            <Counter />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
