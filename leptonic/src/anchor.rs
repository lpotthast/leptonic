use std::borrow::Cow;

use leptos::*;

use crate::prelude::*;

#[component]
pub fn Anchor(
    cx: Scope,
    #[prop(into)] href: Cow<'static, str>,
    #[prop(into)] title: Cow<'static, str>,
    #[prop(into, optional)] children: Option<Children>,
) -> impl IntoView {
    view! { cx,
        <Link href=href.to_string() title=title class="leptonic-anchor">
            { match children {
                Some(children) => children(cx).into_view(cx),
                None => "#".into_view(cx),
            } }
        </Link>
    }
}
