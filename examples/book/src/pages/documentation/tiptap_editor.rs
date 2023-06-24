use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn PageTiptapEditor(cx: Scope) -> impl IntoView {
    let (value, set_value) = create_signal(cx, r#"<h1>This is a simple <em><s>paragraph</s></em> ... <strong>H1</strong>!</h1><p style="text-align: center"><strong>Lorem ipsum dolor sit amet, consetetur sadipscing elitr, <mark>sed diam nonumy</mark> eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua.</strong></p><p style="text-align: justify">Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet.</p>"#.to_owned());

    view! { cx,
        <H1>"Tiptap editor"</H1>

        <TiptapEditor value=value set_value=move |content| match content {
            TiptapContent::Html(content) => set_value.set(content),
            TiptapContent::Json(content) => set_value.set(content),
        } />
    }
}
