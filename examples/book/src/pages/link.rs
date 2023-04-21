use leptonic::prelude::*;
use leptos::*;

use crate::app::Routes;

#[component]
pub fn PageLink(cx: Scope) -> impl IntoView {
    view! { cx,
        <Typography variant=TypographyVariant::H2>"Links"</Typography>

        <Link href=Routes::Link class="item">"This is a link"</Link>
    }
}
