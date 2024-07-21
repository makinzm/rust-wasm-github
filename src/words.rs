use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[derive(Clone, PartialEq, Serialize, Deserialize, Default)]
struct WordEntry {
    priority: i32,
    word: String,
    meaning: String,
    learning_history: String,
}

#[derive(Clone, PartialEq, Properties)]
struct WordsProps {
    words: Vec<WordEntry>,
}

#[function_component(Words)]
pub fn words() -> Html {
    let json_data = r#"
    [
        {"priority": 2, "word": "Hello", "meaning": "こんにちは", "learning_history": "{}"},
        {"priority": 1, "word": "World", "meaning": "世界", "learning_history": "{}"},
        {"priority": 3, "word": "Rust", "meaning": "錆", "learning_history": "{}"},
        {"priority": 4, "word": "Programming", "meaning": "プログラミング", "learning_history": "{}"},
        {"priority": 20, "word": "prerequisite", "meaning": "前提条件", "learning_history": "{}"},
        {"priority": 30, "word": "subsequently", "meaning": "その後", "learning_history": "{}"},
        {"priority": 40, "word": "consequently", "meaning": "その結果", "learning_history": "{}"}
    ]
    "#;

    let mut words: Vec<WordEntry> = serde_json::from_str(json_data).unwrap();
    words.sort_by(|a, b| b.priority.cmp(&a.priority));

    let words_props = WordsProps { words };

    html! {
        <div class="bg-white dark:bg-gray-800 dark:text-white">
            <h1 class="text-5xl text-center font-bold p-8">
                { "Words" }
            </h1>
            <WordList ..words_props />
        </div>
    }
}

#[function_component(WordList)]
fn word_list(WordsProps { words }: &WordsProps) -> Html {
    let page_index = use_state(|| 0);
    let words_per_page = 2;
    let selected_word = use_state(|| None);

    let total_pages = (words.len() + words_per_page - 1) / words_per_page;

    let on_next = {
        let page_index = page_index.clone();
        Callback::from(move |_| {
            if *page_index < total_pages - 1 {
                page_index.set(*page_index + 1);
            }
        })
    };

    let on_prev = {
        let page_index = page_index.clone();
        Callback::from(move |_| {
            if *page_index > 0 {
                page_index.set(*page_index - 1);
            }
        })
    };

    let start_index = *page_index * words_per_page;
    let end_index = (start_index + words_per_page).min(words.len());
    let current_words = &words[start_index..end_index];

    let on_word_click = {
        let selected_word = selected_word.clone();
        Callback::from(move |word: WordEntry| {
            selected_word.set(Some(word));
        })
    };

    html! {
        <div class="text-center p-4">
            <table class="table-auto w-full">
                <thead>
                    <tr>
                        <th class="px-4 py-2">{ "Word" }</th>
                        <th class="px-4 py-2">{ "Meaning" }</th>
                    </tr>
                </thead>
                <tbody>
                    {for current_words.iter().map(|word| {
                        let word_clone = word.clone();
                        let on_click = {
                            let on_word_click = on_word_click.clone();
                            Callback::from(move |_| on_word_click.emit(word_clone.clone()))
                        };
                        html! {
                            <tr class="cursor-pointer" onclick={on_click}>
                                <td class="border px-4 py-2">{ &word.word }</td>
                                <td class="border px-4 py-2">
                                    { if let Some(selected) = &*selected_word {
                                        if selected.word == word.word {
                                            html! { &word.meaning }
                                        } else {
                                            html! { "" }
                                        }
                                    } else {
                                        html! { "" }
                                    }}
                                </td>
                            </tr>
                        }
                    })}
                </tbody>
            </table>
             <div class="flex justify-center space-x-4 mt-4">
                {
                    if *page_index > 0 {
                        html! {
                            <button onclick={on_prev}>{ "Previous" }</button>
                        }
                    } else {
                        html! {}
                    }
                }
                {
                    if *page_index < total_pages - 1 {
                        html! {
                            <button onclick={on_next}>{ "Next" }</button>
                        }
                    } else {
                        html! {}
                    }
                }
            </div>
            <p class="mt-4">{ format!("Page {} of {}", *page_index + 1, total_pages) }</p>
        </div>
    }
}
