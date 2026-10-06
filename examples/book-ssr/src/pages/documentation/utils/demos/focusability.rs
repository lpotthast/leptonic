use leptonic::utils::focusability::{is_focusable, is_tabbable};
use leptos::{html, prelude::*};
use wasm_bindgen::JsCast;

fn yes_no(value: bool) -> &'static str {
    if value { "yes" } else { "no" }
}

#[component]
pub fn FocusabilityDemo() -> impl IntoView {
    let samples = NodeRef::<html::Div>::new();
    let (results, set_results) = signal(Vec::<(String, bool, bool)>::new());

    // Check every sample element once it is rendered (the checks need the DOM).
    Effect::new(move |_| {
        let Some(container) = samples.get() else {
            return;
        };
        let Ok(nodes) = container.query_selector_all("[data-sample]") else {
            return;
        };
        let checked = (0..nodes.length())
            .filter_map(|index| nodes.get(index))
            .filter_map(|node| node.dyn_into::<web_sys::Element>().ok())
            .map(|el| {
                let name = el.text_content().unwrap_or_default();
                (name, is_focusable(&el), is_tabbable(&el))
            })
            .collect();
        set_results.set(checked);
    });

    view! {
        <div node_ref=samples class="demo-focus-row">
            <button type="button" class="demo-focus-item" data-sample>"Button"</button>
            <button type="button" class="demo-focus-item" data-sample disabled>"Disabled button"</button>
            <button type="button" class="demo-focus-item" data-sample tabindex="-1">"Button with tabindex -1"</button>
            <a class="demo-focus-item" data-sample>"Link without href"</a>
            <div inert="">
                <button type="button" class="demo-focus-item" data-sample>"Button in an inert subtree"</button>
            </div>
        </div>

        <ul class="demo-focusability-results">
            {move || {
                results
                    .get()
                    .into_iter()
                    .map(|(name, focusable, tabbable)| {
                        view! {
                            <li>
                                <strong>{name}</strong>
                                ": focusable " {yes_no(focusable)} ", tabbable " {yes_no(tabbable)} "."
                            </li>
                        }
                    })
                    .collect_view()
            }}
        </ul>
    }
}
