use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn PageSeparator(cx: Scope) -> impl IntoView {
    view! { cx,
        <Typography variant=TypographyVariant::H2>"Separators"</Typography>

        <Separator />
    }
}
