use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn ProgressIndeterminateDemo() -> impl IntoView {
    view! {
        <ProgressBar progress=signal(None).0 />
    }
}
