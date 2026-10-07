use leptonic::atoms::prelude::DismissButton;
use leptos::prelude::*;

/// `DismissButton`s (react-aria's `DismissButton.test.tsx` setups), each in a container with an id:
/// - `#test-dismiss-default`: no label (named "Dismiss").
/// - `#test-dismiss-label`: `aria_label="foo"`.
/// - `#test-dismiss-labelledby`: `aria_labelledby` the span `#test-dismiss-span` ("bar").
/// - `#test-dismiss-both`: both, with the id `self`.
///
/// Activating one counts in `#test-dismiss-count`.
#[component]
pub fn PageAtomDismissButton() -> impl IntoView {
    let count = RwSignal::new(0_u32);
    let dismiss = move || count.update(|count| *count += 1);
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
        <p>"Dismissed: " <span id="test-dismiss-count">{count}</span></p>
    }
}
