use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn PageProgressIndicator(cx: Scope) -> impl IntoView {
    let (progress, set_progress) = create_signal(cx, 34.0);

    view! { cx,
        <H2>"Progress indicator"</H2>

        <ProgressBar progress=progress/>
    }
}
