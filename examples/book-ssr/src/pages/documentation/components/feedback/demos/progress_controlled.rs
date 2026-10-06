use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn ProgressControlledDemo() -> impl IntoView {
    let (progress, set_progress) = signal(Some(34.0));

    view! {
        <ProgressBar value=progress label="Upload" />

        <div class="demo-form demo-mt-1">
            <NumberField label="Uploaded (%)" value=progress set_value=set_progress min_value=0.0 max_value=100.0 step=0.01/>

            <Slider
                value=Signal::derive(move || progress.get().unwrap_or(0.0))
                set_value=move |value| set_progress.set(Some(value))
                min_value=0.0
                max_value=100.0
                step=0.01
                aria_label="Uploaded"
            />
        </div>
    }
}
