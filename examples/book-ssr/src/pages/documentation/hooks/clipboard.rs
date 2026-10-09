use indoc::indoc;
use leptos::prelude::*;

use super::demos::{clipboard::ClipboardDemo, clipboard_write_text::ClipboardWriteTextDemo};
use crate::{kit::*, routes};

#[component]
pub fn PageUseClipboard() -> impl IntoView {
    view! {
        <DocPage title="use_clipboard">
            <p>
                "Text fields get cut, copy and paste from the browser. Other elements, such as a list of items or a canvas, "
                "don\u{2019}t: the browser has no idea what their data is. The "<Code inline=true>"use_clipboard"</Code>
                " hook connects an element to the clipboard while it has focus: the usual shortcuts copy and cut your "
                "app\u{2019}s data and paste data into it, in the same format as "
                <Link href=routes::doc::DragAndDrop.materialize()>"drag and drop"</Link>". To write text to the clipboard "
                "from your own code, e.g. for a copy button, use "<AnchorLink href="#write-text">"write_text"</AnchorLink>"."
            </p>

            <ReactAria hook="useClipboard"/>

            <Section title="Input">
                <p>
                    <Code inline=true>"UseClipboardInput"</Code>" implements "<Code inline=true>"Default"</Code>
                    ". Each callback you leave out keeps the browser\u{2019}s own behavior for its action."
                </p>

                <ApiTable kind=ApiKind::Input of="UseClipboardInput">
                    <ApiRow name="get_items" ty="Option<Callback<ClipboardAction, Vec<DragItem>>>" default="None">
                        "The data to copy or cut, called with "<Code inline=true>"ClipboardAction::Copy"</Code>" or "
                        <Code inline=true>"ClipboardAction::Cut"</Code>". Without it, copy and cut keep their default."
                    </ApiRow>
                    <ApiRow name="on_copy" ty="Option<Callback<()>>" default="None">
                        "Called after the items were copied."
                    </ApiRow>
                    <ApiRow name="on_cut" ty="Option<Callback<()>>" default="None">
                        "Called after the items were written to the clipboard: remove them from your data. Cut needs both "
                        "this and "<Code inline=true>"get_items"</Code>"."
                    </ApiRow>
                    <ApiRow name="on_paste" ty="Option<Callback<Vec<DropItem>>>" default="None">
                        "Receives the pasted data. Without it, paste keeps its default."
                    </ApiRow>
                    <ApiRow name="is_disabled" ty="Signal<bool>" default="false">
                        "Leave all clipboard events to the browser."
                    </ApiRow>
                </ApiTable>
            </Section>

            <Section title="Return">
                <ApiTable kind=ApiKind::Return of="UseClipboardReturn">
                    <ApiRow name="clipboard_props" ty="UseFocusProps">
                        "Focus and blur handlers that track whether the element has focus. Spread with "
                        <Code inline=true>"{..clipboard_props.into_attrs()}"</Code>"."
                    </ApiRow>
                </ApiTable>
            </Section>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::{
                            IntoAttrs,
                            hooks::{
                                clipboard::{UseClipboardInput, use_clipboard},
                                dnd::{DragItem, DropItem},
                            },
                        };

                        let note = RwSignal::new(String::from("Water the plants"));

                        let clipboard = use_clipboard(UseClipboardInput {
                            get_items: Some(Callback::new(move |_| vec![DragItem::text(note.get_untracked())])),
                            on_paste: Some(Callback::new(move |items: Vec<DropItem>| {
                                if let Some(DropItem::Text(text)) = items.first()
                                    && let Some(text) = text.get_text("text/plain")
                                {
                                    note.set(text.to_owned());
                                }
                            })),
                            ..Default::default()
                        });

                        view! {
                            // The element must be focusable: the clipboard acts only while it has focus.
                            <div {..clipboard.clipboard_props.into_attrs()} tabindex="0" role="group" aria-label="Note">
                                {note}
                            </div>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Demo">
                <p>
                    "Focus the list and copy it with "<Keys keys="Control + C"/>" ("<Keys keys="Meta + C"/>" on macOS), "
                    "then paste it back with "<Keys keys="Control + V"/>" ("<Keys keys="Meta + V"/>"), or paste lines of "
                    "text copied anywhere else. Cut ("<Keys keys="Control + X"/>" / "<Keys keys="Meta + X"/>") empties "
                    "the list."
                </p>

                <Demo description="Shopping list with keyboard copy, cut and paste" source=include_str!("demos/clipboard.rs")>
                    <ClipboardDemo/>
                </Demo>
            </Section>

            <Section title="Clipboard Data">
                <p>
                    "Copied data is a list of "<Code inline=true>"DragItem"</Code>"s, pasted data a list of "
                    <Code inline=true>"DropItem"</Code>"s, the same types drag and drop uses (see "
                    <Link href=routes::doc::DragAndDrop.materialize()>"Drag & Drop"</Link>"). An item can carry several "
                    "representations, e.g. a link as "<Code inline=true>"text/uri-list"</Code>" and "
                    <Code inline=true>"text/plain"</Code>":"
                </p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        DragItem::text("Milk");                    // text/plain
                        DragItem::new()
                            .with("text/uri-list", "https://leptos.dev")
                            .with("text/plain", "Leptos");
                    "#)}
                </Code>
                <p>
                    "Other applications see one entry per type: the items\u{2019} "<Code inline=true>"text/plain"</Code>", "
                    <Code inline=true>"text/uri-list"</Code>" and "<Code inline=true>"text/html"</Code>" data joined by line "
                    "breaks, of other types the first item\u{2019}s. When that loses information (several items of one type, or "
                    "an item with several types), the clipboard also gets all items as JSON, so pasting into "
                    "an element using leptonic\u{2019}s clipboard or drop hooks restores the separate items. Text pasted from "
                    "elsewhere arrives as one "<Code inline=true>"DropItem::Text"</Code>" with a representation per type; "
                    "pasted files arrive as "<Code inline=true>"DropItem::File"</Code>"."
                </p>
            </Section>

            <Section title="When It Acts">
                <ul>
                    <li>
                        "Only while the element itself has focus, so give it a "<Code inline=true>"tabindex"</Code>
                        " (or use a focusable element) and an accessible name."
                    </li>
                    <li>
                        "Only for the actions you handle: without "<Code inline=true>"get_items"</Code>" copy and cut "
                        "keep their default, without "<Code inline=true>"on_paste"</Code>" paste does."
                    </li>
                    <li>
                        "The hook listens to the clipboard events of the document, one shared listener per event for all "
                        "elements, added once the page runs in the browser. Server-side rendering doesn\u{2019}t touch the "
                        "clipboard."
                    </li>
                    <li>
                        <Code inline=true>"use_clipboard"</Code>" is part of the hooks and needs no feature. The "
                        <Code inline=true>"clipboard"</Code>" feature adds "<AnchorLink href="#write-text">"write_text"</AnchorLink>"."
                    </li>
                </ul>
            </Section>

            <Section title="write_text">
                <p>
                    <Code inline=true>"use_clipboard"</Code>" reacts to the clipboard shortcuts. To put text on the clipboard "
                    "yourself, e.g. when a \u{201c}Copy\u{201d} button is pressed, call "
                    <Code inline=true>"leptonic::write_text"</Code>". It needs the "
                    <Code inline=true>"clipboard"</Code>" feature (part of "<Code inline=true>"full"</Code>")."
                </p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::write_text;
                        use leptos::task::spawn_local;

                        let copy = move |_| {
                            spawn_local(async move {
                                if let Err(err) = write_text("cargo add leptonic").await {
                                    leptos::logging::warn!("{err}");
                                }
                            });
                        };
                    "#)}
                </Code>
                <p>
                    "Browsers only allow writing while handling a user gesture such as a press, so call it right in the "
                    "handler, not after awaiting something else. It fails with "<Code inline=true>"ClipboardError::Unavailable"</Code>
                    " without a browser window (on the server) and with "<Code inline=true>"ClipboardError::Denied"</Code>
                    " when the browser refuses, e.g. in an insecure context or without permission."
                </p>
                <Demo description="A button copying a command, with the outcome shown" source=include_str!("demos/clipboard_write_text.rs")>
                    <ClipboardWriteTextDemo/>
                </Demo>
            </Section>

            <Section title="Keyboard">
                <KeyboardTable>
                    <KeyRow keys="Control + C / Command + C">"Copies the element\u{2019}s items."</KeyRow>
                    <KeyRow keys="Control + X / Command + X">
                        "Cuts the element\u{2019}s items: copies them and calls "<Code inline=true>"on_cut"</Code>"."
                    </KeyRow>
                    <KeyRow keys="Control + V / Command + V">"Pastes into the element."</KeyRow>
                </KeyboardTable>
                <p><Keys keys="Meta"/>" is the modifier on macOS, "<Keys keys="Control"/>" everywhere else."</p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::DragAndDrop.materialize()>"Drag & Drop"</Link>" \u{2014} the same data types, moved with the pointer"</li>
                <li><Link href=routes::doc::focus::UseFocus.materialize()>"use_focus"</Link></li>
                <li><Link href=routes::doc::utilities::UseSpinButton.materialize()>"use_spin_button"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
