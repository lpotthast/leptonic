use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn ProgressControlledDemo() -> impl IntoView {
    let (progress, set_progress) = signal(Some(34.0));

    view! {
        <ProgressBar progress=progress/>

        <NumberInput
            get=Signal::derive(move || progress.get().unwrap_or_default())
            set=move |v| set_progress.set(Some(v))
            attr:style="margin-top: 1em;"
        />

        <Slider
            value=Signal::derive(move || progress.get().unwrap_or(0.0))
            set_value=move |v: f64| set_progress.set(Some((v * 100.0).round() / 100.0))
            min=0.0
            max=100.0
            step=0.01
        />
    }
}
