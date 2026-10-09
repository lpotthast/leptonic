use leptonic::atoms::dismiss_button::DismissButton;
use leptos::prelude::*;

/// `DismissButton`s (react-aria's `DismissButton.test.tsx` setups), each in a container with an id:
/// - `#test-dismiss-default`: no label (named "Dismiss").
/// - `#test-dismiss-label`: `aria_label="foo"`.
/// - `#test-dismiss-labelledby`: `aria_labelledby` the span `#test-dismiss-span` ("bar").
/// - `#test-dismiss-both`: both, with the id `self`.
/// - `#test-dismiss-form`: a form with a text field `#test-dismiss-field` and a dismiss button;
///   submitting it counts in `#test-dismiss-submits`.
///
/// Activating one counts in `#test-dismiss-count`.
#[component]
pub fn PageAtomDismissButton() -> impl IntoView {
    let count = RwSignal::new(0_u32);
    let dismiss = move || count.update(|count| *count += 1);
    let submits = RwSignal::new(0_u32);
    view! {
        <h1>"Dismiss button"</h1>
        <span id="test-dismiss-span">"bar"</span>
        <div id="test-dismiss-default">
            <DismissButton on_dismiss=dismiss />
        </div>
        <div id="test-dismiss-label">
            <DismissButton on_dismiss=dismiss aria_label="foo" />
        </div>
        <div id="test-dismiss-labelledby">
            <DismissButton on_dismiss=dismiss aria_labelledby="test-dismiss-span" />
        </div>
        <div id="test-dismiss-both">
            <DismissButton
                on_dismiss=dismiss
                aria_labelledby="test-dismiss-span"
                aria_label="foo"
                id="self"
            />
        </div>
        <form
            id="test-dismiss-form"
            on:submit=move |e| {
                e.prevent_default();
                submits.update(|submits| *submits += 1);
            }
        >
            <input id="test-dismiss-field" aria-label="Name" />
            <DismissButton on_dismiss=dismiss />
        </form>
        <p>"Dismissed: " <span id="test-dismiss-count">{count}</span></p>
        <p>"Submitted: " <span id="test-dismiss-submits">{submits}</span></p>
    }
}
