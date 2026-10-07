use indoc::indoc;
use leptos::prelude::*;

use super::demos::{
    kbd_custom::KbdCustomDemo, kbd_keys::KbdKeysDemo, kbd_manual::KbdManualDemo,
    kbd_shortcut::KbdShortcutDemo, kbd_single::KbdSingleDemo,
};
use crate::{kit::*, routes};

#[component]
pub fn PageKbd() -> impl IntoView {
    view! {
        <DocPage title="Kbd Components">
            <p>
                "The themed "<Code inline=true>"KbdKey"</Code>" component renders a key cap in a "
                <Code inline=true>"<kbd>"</Code>", "<Code inline=true>"KbdShortcut"</Code>" a combination of keys. The "
                "keys come from the "<Code inline=true>"KeyboardKey"</Code>" enum ("<Code inline=true>"leptonic::utils::key"</Code>
                "), which knows their labels. See the "<Link href=routes::doc::Kbd.materialize()>"Kbd overview"</Link>
                " for concept guidance."
            </p>

            <Demo description="Sentence with the key cap of the Escape key" source=include_str!("demos/kbd_single.rs")>
                <KbdSingleDemo/>
            </Demo>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::{components::prelude::*, utils::key::KeyboardKey};

                        view! {
                            <p>"Search with " <KbdShortcut keys=[KeyboardKey::Control, KeyboardKey::K]/> "."</p>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="KbdKey">
                <p>"A single key cap."</p>
                <Section title="Props" id="kbd-key-props">
                    <ApiTable kind=ApiKind::Props of="KbdKey">
                        <ApiRow name="key" ty="KeyboardKey">"The key to display. Required."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">
                            "Additional classes and styles."
                        </ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="KbdShortcut">
                <p>
                    "A combination of keys pressed together, joined by a "<Code inline=true>"+"</Code>" (or the text you "
                    "pass as "<Code inline=true>"concatenate_with"</Code>")."
                </p>
                <Demo description="Sentence with the shortcut Control + Enter" source=include_str!("demos/kbd_shortcut.rs")>
                    <KbdShortcutDemo/>
                </Demo>
                <Section title="Props" id="kbd-shortcut-props">
                    <ApiTable kind=ApiKind::Props of="KbdShortcut">
                        <ApiRow name="keys" ty="[KeyboardKey; N]">"The keys of the shortcut, in order. Required."</ApiRow>
                        <ApiRow name="concatenate_with" ty="Option<Cow<'static, str>>" default="None">
                            "Text between two keys. "<Code inline=true>"None"</Code>" means "<Code inline=true>"\"+\""</Code>"."
                        </ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">
                            "Additional classes and styles of the outer "<Code inline=true>"<kbd>"</Code>"."
                        </ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="KbdShortcutRoot">
                <p>
                    "The outer "<Code inline=true>"<kbd>"</Code>" of a shortcut. Together with "
                    <Code inline=true>"KbdKey"</Code>" and "<Code inline=true>"KbdConcatenate"</Code>", it builds a "
                    "shortcut from its parts, e.g. to give one of them classes of its own:"
                </p>
                <Demo description="The shortcut Control + Enter built from its parts" source=include_str!("demos/kbd_manual.rs")>
                    <KbdManualDemo/>
                </Demo>
                <Section title="Props" id="kbd-shortcut-root-props">
                    <ApiTable kind=ApiKind::Props of="KbdShortcutRoot">
                        <ApiRow name="children" ty="Children">"The keys and separators of the shortcut."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">
                            "Additional classes and styles."
                        </ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="KbdConcatenate">
                <p>"The text between two keys of a shortcut."</p>
                <Section title="Props" id="kbd-concatenate-props">
                    <ApiTable kind=ApiKind::Props of="KbdConcatenate">
                        <ApiRow name="with" ty="Option<Cow<'static, str>>" default="None">
                            "The separator text. "<Code inline=true>"None"</Code>" means "<Code inline=true>"\"+\""</Code>"."
                        </ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">
                            "Additional classes and styles."
                        </ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="KeyboardKey">
                <p>"These are the keys of the "<Code inline=true>"KeyboardKey"</Code>" enum, with their labels:"</p>
                <Demo description="Key caps of all keys of the KeyboardKey enum" source=include_str!("demos/kbd_keys.rs")>
                    <KbdKeysDemo/>
                </Demo>
                <p>
                    "For any other key, use "<Code inline=true>"KeyboardKey::Other(Cow<'static, str>)"</Code>
                    " with the label to show:"
                </p>
                <Demo description="Key cap with custom text" source=include_str!("demos/kbd_custom.rs")>
                    <KbdCustomDemo/>
                </Demo>
            </Section>

            <Section title="Styling">
                <p>"Override any of these CSS variables to adapt key caps to your design:"</p>
                <CssVariables prefix="--leptonic-kbd-" scss=theme_scss!("kbd")/>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Kbd.materialize()>"Kbd overview"</Link></li>
                <li><Link href=routes::doc::kbd::Atom.materialize()>"Kbd Atom"</Link></li>
                <li><Link href=routes::doc::Typography.materialize()>"Typography Components"</Link></li>
                <li><Link href=routes::doc::interactions::UseKeyboard.materialize()>"use_keyboard"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
