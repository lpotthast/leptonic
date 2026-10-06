use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn ProgressBarConceptDemo() -> impl IntoView {
    view! {
        <ProgressBar value=75.0 label="Uploading" />
    }
}
