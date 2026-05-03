use leptonic::{components::prelude::*, hooks::*, utils::css::em};
use leptos::prelude::*;

#[component]
pub fn ProgressDeterminateDemo() -> impl IntoView {
    let (value, set_value) = signal(65.0);

    let UseProgressBarReturn {
        progress_props,
        label_props,
        ..
    } = use_progress_bar(UseProgressBarInput {
        label: Some("Loading progress".into()),
        value: Signal::derive(move || Some(value.get())),
        min_value: 0.0,
        max_value: 100.0,
        show_value_label: true,
        is_indeterminate: false,
    });

    view! {
        <div style="margin: 0 0 1em 0; max-width: 400px;">
            <div style="display: flex; justify-content: space-between; margin-bottom: 0.5em;">
                <label id=label_props.id.clone()>"Loading progress"</label>
                <span>{ move || format!("{:.0}%", value.get()) }</span>
            </div>
            <div
                {..progress_props.into_attrs()}
                style="height: 8px; background: #ddd; border-radius: 4px; overflow: hidden;"
            >
                <div style=move || format!(
                    "height: 100%; background: var(--brand-color); transition: width 0.3s; width: {}%;",
                    value.get()
                )></div>
            </div>
        </div>

        <Stack orientation=StackOrientation::Horizontal spacing=em(0.5)>
            <button
                on:click=move |_| set_value.update(|v| *v = (*v - 10.0).max(0.0))
                style="padding: 0.5em 1em; border-radius: 4px; cursor: pointer; border: 1px solid #ccc;"
            >
                "-10%"
            </button>
            <button
                on:click=move |_| set_value.update(|v| *v = (*v + 10.0).min(100.0))
                style="padding: 0.5em 1em; border-radius: 4px; cursor: pointer; border: 1px solid #ccc;"
            >
                "+10%"
            </button>
        </Stack>
    }
}
