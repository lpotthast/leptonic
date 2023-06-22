use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn PageSkeleton(cx: Scope) -> impl IntoView {
    view! { cx,
        <H2>"Skeleton"</H2>

        <Skeleton height=Size::Em(5.0)/>
    }
}
