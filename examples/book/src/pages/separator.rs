use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn PageSeparator(cx: Scope) -> impl IntoView {
    view! { cx,
        <H2>"Separators"</H2>

        <Separator />
    }
}
