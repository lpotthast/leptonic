use std::borrow::Cow;

use leptos::*;

use crate::prelude::*;

#[component]
pub fn Anchor(
    cx: Scope,
    #[prop(into)] href: Cow<'static, str>,
    #[prop(into)] title: Cow<'static, str>,
) -> impl IntoView {
    view! { cx,
        <Link href=move || href.to_string() class="anchor">
            "#"
        </Link>
    }
}
