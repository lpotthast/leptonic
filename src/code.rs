use leptos::*;

use crate::prelude::*;

#[component]
pub fn Code(cx: Scope, children: Children) -> impl IntoView {
    view! { cx,
        <Typography variant=TypographyVariant::Code>
            { children(cx) }
        </Typography>
    }
}
