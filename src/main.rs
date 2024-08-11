use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use yew::html::ChildrenProps;
use yew::prelude::*;
use yew_router::prelude::*;

mod distribution;
mod words;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/rust-wasm-github")]
    Home,
    #[at("/rust-wasm-github/words")]
    Words,
    #[at("/rust-wasm-github/distribution")]
    Distribution,
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
                <div class="text-center mt-4 grid-buttons">
                    <Link<Route> to={Route::Words}>
                        <button>
                            { "📚 Words to improve your vocabulary" }
                        </button>
                    </Link<Route>>
                    <Link<Route> to={Route::Distribution}>
                        <button>
                            { "📊 Distribution to associate your statistics with images" }
                        </button>
                    </Link<Route>>
                    <button>
                        { "(TBD) 道 Roadmap to see the future of this project" }
                    </button>
                </div>
            </div>
        },
        Route::Words => html! {
            <div>
                <words::Words />
            </div>
        },
        Route::Distribution => html! {
            <div>
                <distribution::Distribution />
            </div>
        },
        Route::NotFound => html! { <h1>{ "NotFound" }</h1> },
    }
}

#[function_component(Layout)]
fn layout(props: &ChildrenProps) -> Html {
    let menu_open = use_state(|| false);

    let toggle_menu = {
        let menu_open = menu_open.clone();
        Callback::from(move |_| menu_open.set(!*menu_open))
    };

    {
        let menu_open = menu_open.clone();
        use_effect_with_deps(
            move |_| {
                let window = web_sys::window().unwrap();
                let window_clone = window.clone();

                let closure = Closure::wrap(Box::new(move || {
                    if window_clone.inner_width().unwrap().as_f64().unwrap() >= 768.0 {
                        menu_open.set(false);
                    }
                }) as Box<dyn FnMut()>);

                window
                    .add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())
                    .unwrap();

                closure.forget();

                || {}
            },
            (),
        );
    }

    let links = html! {
        <>
            <a href="https://github.com/makinzm" class="hover:text-gray-300">
                { "GitHub(External)" }
            </a>
            <a href="https://x.com/NozomiMaki2" class="hover:text-gray-300">
                { "X(External)" }
            </a>
            <a href="https://bookmeter.com/users/1409315" class="hover:text-gray-300">
                { "Bookmeter(External)" }
            </a>
            <a href="https://atcoder.jp/users/makinzm2" class="hover:text-gray-300">
                { "AtCoder(External)" }
            </a>
        </>
    };

    html! {
        <div class="flex flex-col min-h-screen relative">
            <header class="bg-gray-900 text-white p-4">
                <div class="container mx-auto flex justify-between items-center">
                    <Link<Route> to={Route::Home}>
                        <h1 class="text-3xl">{"makinzm"}</h1>
                    </Link<Route>>
                    <div class="relative">
                        <button class="md:hidden z-10" onclick={toggle_menu.clone()}>
                            <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16m-7 6h7"></path>
                            </svg>
                        </button>
                        // Mobile menu
                        <nav class={format!("absolute right-0 bg-gray-900 text-white mt-2 transition-all duration-300 z-0 {}", if *menu_open { "block" } else { "hidden" })}>
                            <div class="flex flex-col space-y-2 p-4">
                                { links.clone() }
                            </div>
                        </nav>
                    </div>
                    <nav class="hidden md:flex space-x-4">
                        { links }
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
