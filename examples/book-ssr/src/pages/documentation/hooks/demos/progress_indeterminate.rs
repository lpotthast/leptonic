use leptonic::hooks::*;
use leptos::prelude::*;

#[component]
pub fn ProgressIndeterminateDemo() -> impl IntoView {
    let UseProgressBarReturn {
        progress_props,
        label_props,
        ..
    } = use_progress_bar(UseProgressBarInput {
        label: Some("Loading".into()),
        value: Signal::derive(|| None),
        is_indeterminate: true,
        ..Default::default()
    });

    // `.demo-value-bar-indeterminate` slides the fill back and forth with a CSS animation.
    view! {
        <div class="demo-value-bar-container">
            <div class="demo-value-bar-header">
                <label id=label_props.id>"Loading"</label>
            </div>
            <div {..progress_props.into_attrs()} class="demo-value-bar demo-value-bar-indeterminate">
                <div class="demo-value-bar-fill"></div>
            </div>
        </div>
    }
}
