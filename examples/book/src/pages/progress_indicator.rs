use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn PageProgressIndicator(cx: Scope) -> impl IntoView {
    let (progress, set_progress) = create_signal(cx, Some(34.0));
    let (progress2, set_progres2s) = create_signal(cx, None);

    view! { cx,
        <H2>"Progress bar"</H2>

        <ProgressBar progress=progress/>

        <P>"Indeterminate"</P>

        <ProgressBar progress=progress2/>
    }
}
