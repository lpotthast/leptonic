use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn PageProgressIndicator(cx: Scope) -> impl IntoView {
    view! { cx,
        <Typography variant=TypographyVariant::H2>"Progress indicator"</Typography>

        <ProgressBar progress=create_signal(cx, 34).0/>
    }
}
