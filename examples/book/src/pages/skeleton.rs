use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn PageSkeleton(cx: Scope) -> impl IntoView {
    view! { cx,
        <Typography variant=TypographyVariant::H2>"Skeleton"</Typography>

        <Skeleton height=Size::Em(5.0)/>
    }
}
