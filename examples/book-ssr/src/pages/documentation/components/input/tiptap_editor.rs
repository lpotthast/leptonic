use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::tiptap_editor::TiptapEditorDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageTiptapEditor() -> impl IntoView {
    view! {
        <DocPage title="Tiptap Editor">
            <p>
                "The "<Code inline=true>"TiptapEditor"</Code>" component embeds a WYSIWYG rich text editor in your app. "
                "It wraps a headless "<Code inline=true>"TiptapInstance"</Code>" from the "
                <Code inline=true>"leptos-tiptap"</Code>" crate and adds a menu bar for headings, text formatting, "
                "blockquotes, highlighting and text alignment."
            </p>

            <Demo description="Rich text editor with a toggle to disable it" source=include_str!("demos/tiptap_editor.rs")>
                <TiptapEditorDemo/>
            </Demo>

            <Section title="Props">
                <ApiTable kind=ApiKind::Props of="TiptapEditor">
                    <ApiRow name="value" ty="Signal<String>">
                        "The initial content as HTML. The editor keeps its own copy and reports edits through "
                        <Code inline=true>"set_value"</Code>". Required."
                    </ApiRow>
                    <ApiRow name="set_value" ty="Option<Out<TiptapContent>>" default="None">
                        "Receives the new content on every change, as "<Code inline=true>"TiptapContent::Html"</Code>
                        " or "<Code inline=true>"TiptapContent::Json"</Code>"."
                    </ApiRow>
                    <ApiRow name="disabled" ty="Signal<bool>" default="false">
                        "Makes the content read-only and hides the menu bar."
                    </ApiRow>
                    <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">
                        "Additional classes and styles."
                    </ApiRow>
                </ApiTable>
            </Section>

            <Section title="Setup">
                <p>
                    "The editor is behind leptonic\u{2019}s "<Code inline=true>"tiptap"</Code>" feature. It runs Tiptap\u{2019}s "
                    "JavaScript, which leptonic\u{2019}s build script copies into the directory configured as "
                    <Code inline=true>"js-dir"</Code>" in your app\u{2019}s "<Code inline=true>"[package.metadata.leptonic]"</Code>
                    ". The "<Code inline=true>"Root"</Code>" component loads it from there. See "
                    <Link href=routes::doc::Installation.materialize()>"Installation"</Link>"."
                </p>
                <p>
                    "If you want a different editor UI, build it on "<Code inline=true>"TiptapInstance"</Code>" directly."
                </p>
            </Section>

            <Section title="Styling">
                <p>"Override any of these CSS variables to adapt the editor to your design:"</p>
                <CssVariables prefix="--tiptap-editor-" scss=theme_scss!("tiptap_editor")/>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::InputCategory.materialize()>"Input components"</Link></li>
                <li><Link href=routes::doc::text_field::Component.materialize()>"Input component"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
