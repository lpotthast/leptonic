use indoc::indoc;
use leptos::prelude::*;

use super::demos::global_shortcuts::GlobalShortcutsDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageUseGlobalShortcuts() -> impl IntoView {
    view! {
        <DocPage title="use_global_shortcuts">
            <p>
                "The "<Code inline=true>"use_global_shortcuts"</Code>" hook binds keyboard shortcuts that work wherever "
                "the focus is, for as long as the component calling it lives: "<Keys keys="Control + K"/>" ("
                <Keys keys="Meta + K"/>" on macOS) opening a command palette, or "<Keys keys="/"/>" focusing the search "
                "field. This book\u{2019}s search opens with the former. For keys pressed on one element, use "
                <Link href=routes::doc::interactions::UseKeyboard.materialize()>"use_keyboard"</Link>
                ". See the "<Link href=routes::doc::Interactions.materialize()>"Interactions overview"</Link>
                " for the other interaction building blocks."
            </p>

            <Section title="Input">
                <p>
                    "Both fields are sets of "<Code inline=true>"KeyboardShortcuts"</Code>" (see "
                    <Link href=format!("{}#keyboard-shortcuts", routes::doc::interactions::UseKeyboard.materialize())>
                        "Keyboard Shortcuts"
                    </Link>"). "<Code inline=true>"UseGlobalShortcutsInput"</Code>" implements "
                    <Code inline=true>"Default"</Code>" (no shortcuts), so you set the sets you need and finish with "
                    <Code inline=true>"..Default::default()"</Code>". The hook returns nothing: there is no element to "
                    "spread attributes onto."
                </p>

                <ApiTable kind=ApiKind::Input of="UseGlobalShortcutsInput">
                    <ApiRow name="anywhere" ty="KeyboardShortcuts" default="empty">
                        "Shortcuts that work everywhere, also while the user types in a text field. Give them a modifier, "
                        "e.g. "<Code inline=true>"Shortcut::key(\"k\").primary()"</Code>"."
                    </ApiRow>
                    <ApiRow name="outside_text_fields" ty="KeyboardShortcuts" default="empty">
                        "Shortcuts that work only while the user isn\u{2019}t typing: not in text fields, "
                        <Code inline=true>"<select>"</Code>"s and editable content. For keys without modifiers, e.g. "
                        <Code inline=true>"Shortcut::key(\"/\")"</Code>". See "
                        <AnchorLink href="#typing">"Typing"</AnchorLink>"."
                    </ApiRow>
                </ApiTable>
            </Section>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::{
                            hooks::{UseGlobalShortcutsInput, use_global_shortcuts},
                            utils::keyboard_shortcut::{KeyboardShortcuts, Shortcut},
                        };
                        use leptos::prelude::*;

                        let palette_open = RwSignal::new(false);
                        let search = NodeRef::<leptos::html::Input>::new();

                        use_global_shortcuts(UseGlobalShortcutsInput {
                            // Control + K (Command + K on Apple devices), also while typing.
                            anywhere: KeyboardShortcuts::new().on(Shortcut::key("k").primary(), move |_| {
                                palette_open.update(|open| *open = !*open);
                            }),
                            // A bare "/" is text while the user types: only outside text fields.
                            outside_text_fields: KeyboardShortcuts::new().on(Shortcut::key("/"), move |_| {
                                if let Some(input) = search.get_untracked() {
                                    let _ = input.focus();
                                }
                            }),
                        });

                        view! { <input node_ref=search type="search" aria-label="Search" aria-keyshortcuts="/"/> }
                    "#)}
                </Code>
            </Section>

            <Section title="Demo">
                <Demo description="Slash focuses a message field, the primary modifier with Enter sends the message, also while typing" source=include_str!("demos/global_shortcuts.rs")>
                    <GlobalShortcutsDemo/>
                </Demo>
            </Section>

            <Section title="Typing">
                <p>
                    "While the user types, most keys are text: a "<Keys keys="/"/>" belongs in the field, not to a "
                    "shortcut. The "<Code inline=true>"outside_text_fields"</Code>" shortcuts therefore don\u{2019}t fire "
                    "when the key press goes to a typing target ("
                    <Link href=format!("{}#is-typing-target", routes::doc::focus::Focusability.materialize())>
                        <Code inline=true>"is_typing_target"</Code>
                    </Link>"): an "<Code inline=true>"<input>"</Code>" of a text-like type, a "
                    <Code inline=true>"<textarea>"</Code>", editable content, or a "<Code inline=true>"<select>"</Code>
                    " (typing picks an option). Everywhere else, they are checked before the "
                    <Code inline=true>"anywhere"</Code>" shortcuts: a shortcut bound in both sets runs only its "
                    <Code inline=true>"outside_text_fields"</Code>" handler there."
                </p>
                <p>
                    "A shortcut requires exactly its modifiers, with one exception: for a character without case ("
                    <Code inline=true>"/"</Code>", "<Code inline=true>"?"</Code>", digits), "<Keys keys="Shift"/>" is ignored unless the shortcut "
                    "requires it, as the keyboard layout decides whether typing the character takes it. So "
                    <Code inline=true>"Shortcut::key(\"/\")"</Code>" also fires on a German keyboard, where "<Keys keys="/"/>
                    " is "<Keys keys="Shift + 7"/>", and "<Code inline=true>"Shortcut::key(\"?\")"</Code>" needs no "
                    <Code inline=true>".shift()"</Code>"."
                </p>
            </Section>

            <Section title="Event Handling">
                <ul>
                    <li>
                        "The hook listens for "<Code inline=true>"keydown"</Code>" on the document in the capture phase, so "
                        "its shortcuts see every key press before the focused element does. Leptonic\u{2019}s atoms "
                        "stop their events from bubbling (see "
                        <Link href=routes::doc::EventPropagation.materialize()>"Event Propagation"</Link>"), so a "
                        <Code inline=true>"use_keyboard"</Code>" listener on your app\u{2019}s root element would miss "
                        "the keys pressed inside them."
                    </li>
                    <li>
                        "A handled shortcut prevents the browser\u{2019}s default and stops the event: the focused element "
                        "never sees the key. That keeps the browser from running a shortcut of its own on the same keys, "
                        "and the field that "<Keys keys="/"/>" just focused from receiving the slash. A handler "
                        "returning "<Code inline=true>"false"</Code>" leaves the event alone (see "
                        <Link href=format!("{}#shortcutoutcome", routes::doc::interactions::UseKeyboard.materialize())>
                            <Code inline=true>"ShortcutOutcome"</Code>
                        </Link>")."
                    </li>
                    <li>
                        "Key presses that compose text in an input method editor (IME), e.g. while typing Japanese, are "
                        "ignored."
                    </li>
                    <li>
                        "The listener is removed when the calling component\u{2019}s owner is cleaned up. Call the hook in "
                        "the component of a page or a view for shortcuts that only work there. Each call adds a listener "
                        "of its own: when two mounted components bind the same shortcut, both handlers run."
                    </li>
                    <li>
                        "Server-side rendering binds nothing; the shortcuts work once the page is hydrated."
                    </li>
                </ul>
            </Section>

            <Section title="Accessibility">
                <ul>
                    <li>
                        "Show shortcuts where users meet the action, e.g. next to a button or in its tooltip. The "
                        <Link href=format!("{}#shortcutkeys", routes::doc::Kbd.materialize())><Code inline=true>"ShortcutKeys"</Code></Link>
                        " atom shows a "<Code inline=true>"Shortcut"</Code>" as the user\u{2019}s platform writes it."
                    </li>
                    <li>
                        "Tell assistive technology about a shortcut with "<Code inline=true>"aria-keyshortcuts"</Code>
                        " on the element it activates or focuses, as in the example: "<Code inline=true>"\"/\""</Code>", "
                        <Code inline=true>"\"Control+K\""</Code>" or "<Code inline=true>"\"Meta+K\""</Code>"."
                    </li>
                    <li>
                        "Single-character shortcuts like "<Keys keys="/"/>" can be triggered by accident by speech "
                        "input. "
                        <Link href="https://www.w3.org/WAI/WCAG22/Understanding/character-key-shortcuts.html" target=LinkTarget::Blank>
                            "WCAG 2.1.4 Character Key Shortcuts"
                        </Link>
                        " asks that users can turn them off or remap them."
                    </li>
                    <li>
                        "Every shortcut needs another way to the same action (a button, a menu item), and shouldn\u{2019}t "
                        "take keys the browser or assistive technology rely on."
                    </li>
                </ul>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Interactions.materialize()>"Interactions overview"</Link></li>
                <li><Link href=routes::doc::interactions::UseKeyboard.materialize()>"use_keyboard"</Link></li>
                <li><Link href=routes::doc::Kbd.materialize()>"Kbd Atoms"</Link></li>
                <li><Link href=routes::doc::focus::Focusability.materialize()>"focusability"</Link></li>
                <li><Link href=routes::doc::EventPropagation.materialize()>"Event Propagation"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
