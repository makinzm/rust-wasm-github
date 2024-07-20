use yew_router::prelude::*;
use yew::prelude::*;
use yew::html::ChildrenProps;

use rust_wasm_github::demo::counter::Counter;
mod words;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/rust-wasm-github")]
    Home,
    #[at("/rust-wasm-github/words")]
    Words,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {
            <div class="bg-white dark:bg-gray-800 dark:text-white">
            <h1 class="text-5xl text-center font-bold p-8">
                { "Hello World" }
            </h1>
                <div class="text-center mt-4">
                    <Link<Route> to={Route::Words}>
                        <button>
                            { "Go to Words" }
                        </button>
                    </Link<Route>>
                </div>
                <Counter />
            </div>
        },
        Route::Words => html! { 
            <div>
                <words::Words />
            </div>
        },
        Route::NotFound => html! { <h1>{ "NotFound" }</h1> },
    }
}

#[function_component(Layout)]
fn layout(props: &ChildrenProps) -> Html {
    html! {
        <div class="flex flex-col min-h-screen">
            <header class="bg-gray-900 text-white p-4">
                <div class="container mx-auto flex justify-between items-center">
                    <h1 class="text-3xl">{"makinzm"}</h1>
                    <nav>
                        <Link<Route> to={Route::Home} classes="hover:text-gray-300">
                            { "Home" }
                        </Link<Route>>
                    </nav>
                </div>
            </header>
            <main class="container mx-auto p-4 flex-grow">
                { for props.children.iter() }
            </main>
            <footer class="bg-gray-900 text-white p-4 mt-auto">
                <div class="container mx-auto text-center">
                    { "I like Rust and WebAssembly" }
                </div>
            </footer>
        </div>
    }
}

#[function_component(Main)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Layout>
                <Switch<Route> render={switch} />
            </Layout>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<Main>::new().render();
}

