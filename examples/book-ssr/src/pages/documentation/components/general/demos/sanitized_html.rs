use leptonic::components::{prelude::*, sanitized_html::sanitize};
use leptos::prelude::*;

#[component]
pub fn SanitizedHtmlDemo() -> impl IntoView {
    // HTML as it might arrive from a user: harmless markup, a script and an event handler.
    let html = RwSignal::new(String::from(
        "<p>Hello <b>world</b>!</p>\n\
         <script>alert('Gotcha')</script>\n\
         <p onclick=\"alert('Gotcha')\">Click <a href=\"javascript:alert('Gotcha')\">me</a>.</p>",
    ));

    view! {
        <TextField label="HTML" multiline=true value=html set_value=html/>

        <p class="demo-caption">"Rendered"</p>
        <SanitizedHtml html=html classes="demo-sanitized-output"/>

        <p class="demo-caption">"Sanitized markup"</p>
        {move || view! { <Code language=Language::Html>{sanitize(&html.get(), None)}</Code> }}
    }
}
