use leptonic::hooks::*;
use leptos::prelude::*;

#[component]
pub fn ProgressIndeterminateDemo() -> impl IntoView {
    let UseProgressBarReturn {
        progress_props: indeterminate_props,
        ..
    } = use_progress_bar(UseProgressBarInput {
        label: Some("Loading".into()),
        value: Signal::derive(|| None),
        min_value: 0.0,
        max_value: 100.0,
        show_value_label: false,
        is_indeterminate: true,
    });

    view! {
        <div style="max-width: 400px;">
            <div
                {..indeterminate_props.into_attrs()}
                style="height: 8px; background: #ddd; border-radius: 4px; overflow: hidden; position: relative;"
            >
                <div style="
                    position: absolute;
                    width: 40%;
                    height: 100%;
                    background: var(--brand-color);
                    animation: indeterminate 1.5s infinite ease-in-out;
                "></div>
            </div>
        </div>

        <style>
            r"@keyframes indeterminate {
                0% { left: -40%; }
                100% { left: 100%; }
            }"
        </style>
    }
}
