use leptonic::prelude::*;
use leptos::*;

use crate::pages::documentation::doc_root::DocRoutes;

#[component]
pub fn PageLink(cx: Scope) -> impl IntoView {
    view! { cx,
        <H2>"Links"</H2>

        <Link href=DocRoutes::Link class="item">"This is a link"</Link>
    }
}
