use leptonic::{components::prelude::*, utils::key::KeyboardKey};
use leptos::prelude::*;

use super::demos::{
    kbd_custom::KbdCustomDemo, kbd_manual::KbdManualDemo, kbd_shortcut::KbdShortcutDemo,
    kbd_single::KbdSingleDemo,
};
use crate::{kit::*, routes};

#[component]
pub fn PageKbd() -> impl IntoView {
    view! {
        <DocPage title="Kbd">
            <p>
                "The "<Code inline=true>"KbdKey"</Code>" component renders a labeled key cap in a "
                <Code inline=true>"<kbd>"</Code>" element. The "<Code inline=true>"KeyboardKey"</Code>
                " enum lists well-known keys and their labels."
            </p>

            <Demo description="Key cap of the Option key" source=include_str!("demos/kbd_single.rs")>
                <KbdSingleDemo/>
            </Demo>

            <Section title="Props">
                <Section title="KbdKey">
                    <ApiTable kind=ApiKind::Props of="KbdKey">
                        <ApiRow name="key" ty="KeyboardKey">"The key to display. Required."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">
                            "Additional classes and styles."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="KbdShortcut">
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

                <Section title="KbdShortcutRoot">
                    <ApiTable kind=ApiKind::Props of="KbdShortcutRoot">
                        <ApiRow name="children" ty="Children">"The keys and separators of the shortcut."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">
                            "Additional classes and styles."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="KbdConcatenate">
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

            <Section title="Shortcuts">
                <p>
                    "Use these components to hint at keyboard shortcuts of your app. As shortcuts usually consist of "
                    "two or more keys, "<Code inline=true>"KbdShortcut"</Code>" renders a whole combination from the keys "
                    "that must be pressed."
                </p>
                <p>
                    "These components only display keys. They don\u{2019}t listen for key presses; handle the shortcut "
                    "itself in your app."
                </p>

                <Demo description="Shortcut Command + Enter" source=include_str!("demos/kbd_shortcut.rs")>
                    <KbdShortcutDemo/>
                </Demo>

                <p>
                    "The same shortcut, built by hand from "<Code inline=true>"KbdShortcutRoot"</Code>", "
                    <Code inline=true>"KbdKey"</Code>" and "<Code inline=true>"KbdConcatenate"</Code>
                    ". Use this when you need control over the individual parts."
                </p>

                <Demo description="Shortcut Command + Enter composed by hand" source=include_str!("demos/kbd_manual.rs")>
                    <KbdManualDemo/>
                </Demo>
            </Section>

            <Section title="Keys">
                <p>"These are all keys of the "<Code inline=true>"KeyboardKey"</Code>" enum, with their labels:"</p>

                <Demo description="All keys of the KeyboardKey enum">
                    <div class="demo-clf-wrap">
                        {KeyboardKey::known_keys().map(|key| view! { <KbdKey key/> }).collect_view()}
                    </div>
                </Demo>

                <p>
                    "For any other content, use the "<Code inline=true>"KeyboardKey::Other(Cow<'static, str>)"</Code>" variant."
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
                <li><Link href=routes::doc::interactions::UseKeyboard.materialize()>"use_keyboard"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
