use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn PageChangelog(cx: Scope) -> impl IntoView {
    view! { cx,
        <H1>"Changelog"</H1>

        <H3>"0.0.1"</H3>

        <P>"Initial release, adding the following components:"</P>
        <ul>
            <li>"Stack"</li>
            <li>"Grid"</li>
        </ul>
    }
}
