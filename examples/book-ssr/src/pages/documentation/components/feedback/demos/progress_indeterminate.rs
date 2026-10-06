use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn ProgressIndeterminateDemo() -> impl IntoView {
    // `None`: the progress isn't known.
    let progress: Option<f64> = None;
    view! {
        <ProgressBar value=progress label="Connecting" />
    }
}
