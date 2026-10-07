use leptos::prelude::*;

use super::demos::tiptap_editor::TiptapEditorDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageTiptapEditor() -> impl IntoView {
    view! {
        <DocPage title="Rich Text Editor Component">
            <p>
                "A rich text editor lets users write formatted text \u{2014} headings, emphasis, quotes \u{2014} where plain "
                "text isn\u{2019}t enough, such as for articles, notes or comments, and shows it as it will look. The "
                <Code inline=true>"TiptapEditor"</Code>" component embeds the "
                <Link href="https://tiptap.dev" target=LinkTarget::Blank>"Tiptap"</Link>" editor: it wraps a headless "
                <Code inline=true>"TiptapInstance"</Code>" from the "<Code inline=true>"leptos-tiptap"</Code>
                " crate and adds a menu bar for headings, text formatting, block quotes, highlighting and text alignment."
            </p>

            <Demo description="Rich text editor with notes for a trip, reporting its edits" source=include_str!("demos/tiptap_editor.rs")>
                <TiptapEditorDemo/>
            </Demo>

            <Section title="TiptapEditor">
                <p>
                    "Without "<Code inline=true>"value"</Code>", the editor keeps the content itself: "
                    <Code inline=true>"default_value"</Code>" is its initial content, and "<Code inline=true>"on_change"</Code>
                    " reports every edit as HTML. Bind "<Code inline=true>"value"</Code>" and "<Code inline=true>"set_value"</Code>
                    " to control it: content set from outside replaces the editor\u{2019}s."
                </p>
                <Section title="Props" id="tiptap-editor-props">
                    <ApiTable kind=ApiKind::Props of="TiptapEditor">
                        <ApiRow name="default_value" ty="String" default="empty">
                            "The initial content as HTML. Ignored with "<Code inline=true>"value"</Code>"."
                        </ApiRow>
                        <ApiRow name="value" ty="Option<Signal<String>>" default="None">
                            "The content as HTML (controlled): a value or any signal."
                        </ApiRow>
                        <ApiRow name="set_value" ty="Option<Out<String>>" default="None">
                            "Receives the content after every edit: an "<Code inline=true>"RwSignal"</Code>", "
                            <Code inline=true>"WriteSignal"</Code>", closure, "<Code inline=true>"Callback"</Code>", \u{2026}"
                        </ApiRow>
                        <ApiRow name="on_change" ty="Option<Callback<String>>" default="None">
                            "Called with the content (HTML) after every edit."
                        </ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="\"Rich text editor\"">
                            "Names the editor, a group of its toolbar and content."
                        </ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>" default="false">
                            "Makes the content read-only and hides the menu bar."
                        </ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">
                            "Classes and styles of the editor\u{2019}s outer element."
                        </ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Setup">
                <p>
                    "The editor is behind leptonic\u{2019}s "<Code inline=true>"tiptap"</Code>" feature. Tiptap\u{2019}s "
                    "JavaScript ships with the "<Code inline=true>"leptos-tiptap"</Code>" crate and is bundled into your "
                    "app\u{2019}s WebAssembly package, so there is nothing to copy or load. Its toolbar buttons stay disabled "
                    "until the editor is ready. See "<Link href=routes::doc::Installation.materialize()>"Installation"</Link>"."
                </p>
                <p>
                    "For a different editor UI, build it on "<Code inline=true>"TiptapInstance"</Code>" directly."
                </p>
            </Section>

            <Section title="Accessibility">
                <p>
                    "The editor is a group named by "<Code inline=true>"aria_label"</Code>" (default \u{201c}Rich text "
                    "editor\u{201d}). Its menu is a "<Link href=routes::doc::Toolbar.materialize()>"toolbar"</Link>
                    " named \u{201c}Formatting\u{201d}: "<Keys keys="Tab"/>" enters it once, the arrow keys move between "
                    "its buttons, and each format button announces through "<Code inline=true>"aria-pressed"</Code>
                    " whether its format is active. The editable area itself can\u{2019}t be labelled yet: tiptap creates "
                    "it in JavaScript, so screen readers name it only through the surrounding group."
                </p>
            </Section>

            <Section title="Styling">
                <p>"Override any of these CSS variables to adapt the editor to your design:"</p>
                <CssVariables prefix="--tiptap-editor-" scss=theme_scss!("tiptap_editor")/>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Fields.materialize()>"Fields"</Link></li>
                <li><Link href=routes::doc::TextField.materialize()>"Text Field"</Link>" (plain text, on one line or several)"</li>
                <li><Link href=routes::doc::Installation.materialize()>"Installation"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
