use leptonic::{is_focusable, is_tabbable};
use leptos::{html, prelude::*, web_sys};
use wasm_bindgen::JsCast;

/// `is_focusable` and `is_tabbable` (react-aria's `isFocusable` and `isTabbable`) of elements
/// with different `tabindex` values. Once mounted, every element with a `data-case` gets
/// `data-focusable` and `data-tabbable` (`true`/`false`) with their results.
#[component]
pub fn PageHookFocusability() -> impl IntoView {
    let cases = NodeRef::<html::Div>::new();
    Effect::new(move |_| {
        let Some(container) = cases.get() else {
            return;
        };
        let Ok(elements) = container.query_selector_all("[data-case]") else {
            return;
        };
        for index in 0..elements.length() {
            let Some(element) = elements
                .item(index)
                .and_then(|node| node.dyn_into::<web_sys::Element>().ok())
            else {
                continue;
            };
            let focusable = is_focusable(&element).to_string();
            let tabbable = is_tabbable(&element).to_string();
            let _ = element.set_attribute("data-focusable", &focusable);
            let _ = element.set_attribute("data-tabbable", &tabbable);
        }
    });

    view! {
        <h1>"Focusability"</h1>
        <div node_ref=cases>
            <div data-case="div">"No tabindex"</div>
            <div data-case="div-0" tabindex="0">"0"</div>
            <div data-case="div-1" tabindex="1">"1"</div>
            <div data-case="div-minus-1" tabindex="-1">"-1"</div>
            <div data-case="div-minus-2" tabindex="-2">"-2"</div>
            <div data-case="div-invalid" tabindex="abc">"abc"</div>
            <div data-case="div-disabled" tabindex="0" disabled="">"Disabled"</div>
            <button data-case="button">"Button"</button>
            <button data-case="button-minus-1" tabindex="-1">"Button -1"</button>
            <button data-case="button-minus-2" tabindex="-2">"Button -2"</button>
            <button data-case="button-invalid" tabindex="abc">"Button abc"</button>
            <button data-case="button-disabled" disabled=true>"Disabled button"</button>
        </div>
    }
}
