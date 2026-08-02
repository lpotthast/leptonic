use indoc::indoc;
use leptonic::{atoms::link::AnchorLink, components::prelude::*, prelude::*};
use leptos::prelude::*;

use crate::pages::documentation::{article::Article, toc::Toc};

const INITIAL_CONTENT: &str = r#"
    <h1>This is a simple <em><s>paragraph</s></em> ... <strong>H1</strong>!</h1>
    <p style="text-align: center">
        <strong>Lorem ipsum dolor sit amet, <mark>sed diam nonumy</mark> eirmod tempor.</strong>
    </p>
    <p style="text-align: justify">Edit this document to see on_change pull the latest HTML.</p>
"#;

#[component]
pub fn PageTiptapEditor() -> impl IntoView {
    let handle = TiptapEditorHandle::new();
    let (disabled, set_disabled) = signal(false);
    let (latest_html, set_latest_html) = signal(String::new());

    let on_change = move || {
        let result: TiptapEditorResult<String> = handle.get_html();
        match result {
            Ok(html) => set_latest_html.set(html),
            Err(report) => tracing::error!(error = %report, "Could not read Tiptap HTML"),
        }
    };

    view! {
        <Article>
            <h1 id="editor" class="anchor">
                "Tiptap editor"
                <AnchorLink href="#editor" description="Direct link to article header"/>
            </h1>

            <p>"Embed a simple WYSIWYG editor into your app."</p>

            <p>
                <Code inline=true>"TiptapToolbarEditor"</Code>
                " adds Leptonic's toolbar to the lower-level "
                <Code inline=true>"leptos_tiptap::TiptapEditor"</Code>
                ". Create one handle per logical editor and provide a stable, globally unique ID. Initial content is "
                "consumed once; use the handle to read or replace content after the editor is ready."
            </p>

            <p>
                "HTML reads return "<Code inline=true>"TiptapEditorResult<String>"</Code>
                ". JSON content and "<Code inline=true>"handle.get_json()"</Code>
                " use structured "<Code inline=true>"serde_json::Value"</Code>" values."
            </p>

            <p style="display: flex;">
                <Toggle state=disabled set_state=set_disabled attr:style="margin-right: 0.5em;"/>
                <span style="font-style: italic; color: gray;">
                    {move || if disabled.get() { "disabled" } else { "enabled" }}
                </span>
            </p>

            <Code>
                {indoc!(r#"
                    let handle = TiptapEditorHandle::new();
                    let on_change = move || {
                        let result: TiptapEditorResult<String> = handle.get_html();
                        match result {
                            Ok(html) => set_html.set(html),
                            Err(report) => tracing::error!(error = %report, "Could not read Tiptap HTML"),
                        }
                    };

                    view! {
                        <TiptapToolbarEditor
                            id="article-editor"
                            handle=handle
                            initial_content=TiptapContent::html("<p>Edit me.</p>")
                            disabled=disabled
                            on_change=on_change
                            on_error=move |report| tracing::error!(error = %report, "Tiptap bridge error")
                        />
                    }
                "#)}
            </Code>

            <TiptapToolbarEditor
                id="book-tiptap-toolbar-editor"
                handle=handle
                initial_content=TiptapContent::html(INITIAL_CONTENT)
                disabled=disabled
                on_change=on_change
                on_error=move |report| tracing::error!(error = %report, "Tiptap bridge error")
            />

            <p>
                "Latest HTML read after a change: "
                {move || {
                    let html = latest_html.get();
                    if html.is_empty() {
                        "No edits yet.".to_owned()
                    } else {
                        format!("{} bytes", html.len())
                    }
                }}
            </p>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Editor", link: "#editor" },
            ]
        }/>
    }
}
