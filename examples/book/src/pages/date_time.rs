use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn PageDateTime(cx: Scope) -> impl IntoView {
    view! { cx,
        <Typography variant=TypographyVariant::H2>"Date & Time"</Typography>
    }
}
