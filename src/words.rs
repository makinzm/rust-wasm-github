use yew::prelude::*;

#[function_component(Words)]
pub fn words() -> Html {
    html! {
        <div class="bg-white dark:bg-gray-800 dark:text-white">
            <h1 class="text-5xl text-center font-bold p-8">
                { "Words" }
            </h1>
            <div class="text-center p-4 text-3xl">
                <p>{ "Words" }</p>
            </div>
        </div>
    }
}
