use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn PageProgressIndicator(cx: Scope) -> impl IntoView {
    view! { cx,
        <H2>"Progress indicator"</H2>

        <ProgressBar progress=create_signal(cx, 34).0/>
    }
}
