use leptonic::prelude::*;
use leptos::*;

use crate::app::Routes;

#[component]
pub fn PageLink(cx: Scope) -> impl IntoView {
    view! { cx,
        <H2>"Links"</H2>

        <Link href=Routes::Link class="item">"This is a link"</Link>
    }
}
