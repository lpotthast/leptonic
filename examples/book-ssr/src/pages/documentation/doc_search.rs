use leptonic::components::prelude::*;
use leptonic::prelude::*;
use leptos::prelude::{Oco, *};
use leptos_router::components::A;

use crate::search::{SearchResult, search_docs};

#[component]
pub fn DocSearchTrigger(set_open: WriteSignal<bool>) -> impl IntoView {
    view! {
        <button
            class="doc-search-trigger"
            title="Search documentation (Cmd+K)"
            on:click=move |_| set_open.set(true)
        >
            <Icon icon=icondata::BsSearch/>
            <span class="doc-search-trigger-text">"Search docs..."</span>
            <kbd class="doc-search-trigger-kbd">"⌘K"</kbd>
        </button>
    }
}

#[component]
pub fn DocSearch() -> impl IntoView {
    let (show, set_show) = signal(false);
    let (input, set_input) = signal(String::new());

    // Keyboard shortcut: Cmd+K / Ctrl+K
    #[cfg(not(feature = "ssr"))]
    {
        use wasm_bindgen::JsCast;

        let handler = wasm_bindgen::closure::Closure::<dyn Fn(web_sys::KeyboardEvent)>::new(
            move |e: web_sys::KeyboardEvent| {
                if (e.meta_key() || e.ctrl_key()) && e.key() == "k" {
                    e.prevent_default();
                    set_show.update(|v| *v = !*v);
                }
            },
        );

        if let Some(window) = web_sys::window() {
            let _ = window
                .add_event_listener_with_callback("keydown", handler.as_ref().unchecked_ref());
        }

        // Prevent the closure from being dropped while we still need the listener.
        // Leak is bounded: at most one instance lives for the app lifetime.
        handler.forget();
    }

    let results = Resource::new(
        move || input.get(),
        |q| async move {
            if q.len() < 2 {
                return Vec::new();
            }
            search_docs(q).await.unwrap_or_default()
        },
    );

    let close = move || {
        set_show.set(false);
        set_input.set(String::new());
    };

    view! {
        <DocSearchTrigger set_open=set_show/>

        <Modal
            show_when=show
            on_close=move |()| close()
            classes="doc-search-modal"
        >
            <ModalHeader>
                <TextInput
                    get=input
                    set=set_input
                    placeholder=Signal::from(Oco::Borrowed("Search documentation..."))
                    should_be_focused=show
                />
            </ModalHeader>
            <ModalBody>
                <div class="doc-search-results">
                    <Suspense fallback=move || view! { <div class="doc-search-loading">"Searching..."</div> }>
                        {move || Suspend::new(async move {
                            let results: Vec<SearchResult> = results.await;
                            if results.is_empty() && input.get().len() >= 2 {
                                view! { <div class="doc-search-empty">"No results found."</div> }.into_any()
                            } else {
                                results.into_iter().map(|r| {
                                    let path = r.path.clone();
                                    view! {
                                        <A href=path attr:class="doc-search-result" on:click=move |_| close()>
                                            <div class="doc-search-result-title">{r.title}</div>
                                            <div class="doc-search-result-snippet">{r.snippet}</div>
                                        </A>
                                    }
                                }).collect_view().into_any()
                            }
                        })}
                    </Suspense>
                </div>
            </ModalBody>
        </Modal>
    }
}
