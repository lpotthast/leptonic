use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::tiptap_editor::TiptapEditorDemo;
use crate::pages::documentation::demo_shell::DemoShell;
use crate::pages::documentation::{article::Article, toc::Toc};

#[component]
pub fn PageTiptapEditor() -> impl IntoView {
    view! {
        <Article>
            <h1 id="editor" class="anchor">
                "Tiptap editor"
                <AnchorLink href="#editor" description="Direct link to article header"/>
            </h1>

            <p>"Embed a simple WYSIWYG editor into your app."</p>

            <p>
                "The provided editor is a wrapper around a headless Tiptap editor instance obtainable via the leptos-tiptap integration. "
                "You may want to look into building you own editor UI!"
            </p>

            <DemoShell source=include_str!("demos/tiptap_editor.rs")>
                <TiptapEditorDemo />
            </DemoShell>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Editor", link: "#editor" },
            ]
        }/>
    }
}
