use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn ProgressConceptDemo() -> impl IntoView {
    view! {
        <ProgressBar progress=Signal::derive(move || Some(75.0)) />
    }
}
