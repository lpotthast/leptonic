use indoc::indoc;
use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn PageTiptapEditor() -> impl IntoView {
    let (value, set_value) = create_signal(r#"<h1>This is a simple <em><s>paragraph</s></em> ... <strong>H1</strong>!</h1><p style="text-align: center"><strong>Lorem ipsum dolor sit amet, consetetur sadipscing elitr, <mark>sed diam nonumy</mark> eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua.</strong></p><p style="text-align: justify">Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet.</p>"#.to_owned());
    let (disabled, set_disabled) = create_signal(false);

    view! {
        <H1>"Tiptap editor"</H1>

        <P>"Embed a simple WYSIWYG editor into your app."</P>

        <P>
            "The provided editor is a wrapper around a headless Tiptap editor instance obtainable via the leptos-tiptap integration. "
            "You may want to look into building you own editor UI!"
        </P>

        <P style="display: flex;">
            <Toggle state=disabled set_state=set_disabled style="margin-right: 0.5em;"/>
            <span style="font-style: italic; color: gray;">
                {
                    move || match disabled.get() {
                        true => "disabled",
                        false => "enabled",
                    }
                }
            </span>
        </P>

        <Code>
            {indoc!(r#"
                <TiptapEditor disabled=disabled value=value set_value=create_callback(move |content| match content {
                    TiptapContent::Html(content) | TiptapContent::Json(content) => set_value.set(content),
                }) />
            "#)}
        </Code>

        <TiptapEditor disabled=disabled value=value set_value=create_callback(move |content| match content {
            TiptapContent::Html(content) | TiptapContent::Json(content) => set_value.set(content),
        }) />
    }
}
