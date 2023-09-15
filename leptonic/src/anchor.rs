use std::borrow::Cow;

use leptos::*;

use crate::prelude::*;

#[component]
pub fn Anchor(
    #[prop(into)] href: Cow<'static, str>,
    #[prop(into)] title: Cow<'static, str>,
    #[prop(into, optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <Link href=href.to_string() title=title.to_string() class="leptonic-anchor">
            { match children {
                Some(children) => children().into_view(),
                None => "#".into_view(),
            } }
        </Link>
    }
}
