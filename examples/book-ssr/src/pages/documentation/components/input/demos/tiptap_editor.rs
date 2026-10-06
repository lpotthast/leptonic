use leptonic::{components::prelude::*, prelude::*};
use leptos::prelude::*;

#[component]
pub fn TiptapEditorDemo() -> impl IntoView {
    let (value, set_value) = signal(
        r#"<h1>This is a simple <em><s>paragraph</s></em> ... <strong>H1</strong>!</h1>
<p style="text-align: center"><strong>Lorem ipsum dolor sit amet, consetetur sadipscing elitr,
<mark>sed diam nonumy</mark> eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua.</strong></p>
<p style="text-align: justify">Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor
invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea
rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet.</p>"#
            .to_owned(),
    );
    let (disabled, set_disabled) = signal(false);

    view! {
        <div class="demo-control-row demo-mb-1">
            <Switch state=(disabled, set_disabled)>"Disable the editor"</Switch>
            <span class="demo-status">{move || if disabled.get() { "Disabled" } else { "Enabled" }}</span>
        </div>

        <TiptapEditor
            disabled=disabled
            value=value
            set_value=move |content| match content {
                TiptapContent::Html(content) | TiptapContent::Json(content) => set_value.set(content),
            }
        />
    }
}
