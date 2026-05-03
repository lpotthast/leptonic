use indoc::indoc;
use leptonic::{components::prelude::*, utils::key::Key};
use leptos::prelude::*;

use super::demos::kbd_custom::KbdCustomDemo;
use super::demos::kbd_manual::KbdManualDemo;
use super::demos::kbd_shortcut::KbdShortcutDemo;
use super::demos::kbd_single::KbdSingleDemo;
use crate::pages::documentation::demo_shell::DemoShell;
use crate::pages::documentation::{article::Article, toc::Toc};

#[component]
pub fn PageKbd() -> impl IntoView {
    let known_keys = Key::known_keys();

    view! {
        <Article>
            <h1 id="keyboard" class="anchor">
                "Keyboard"
                <AnchorLink href="#keyboard" description="Direct link to article header"/>
            </h1>

            <p>
                "Display labeled keyboard key-caps using the "<Code inline=true>"<KbdKey>"</Code>" component."
                " Leptonic provides the "<Code inline=true>"Key"</Code>" enum which provides well-known keys and their display properties."
            </p>

            <DemoShell source=include_str!("demos/kbd_single.rs")>
                <KbdSingleDemo />
            </DemoShell>

            <h2 id="shortcuts" class="anchor">
                "Shortcuts"
                <AnchorLink href="#shortcuts" description="Direct link to section: Shortcuts"/>
            </h2>

            <p>
                "You may use this component to display a hint to a keyboard shortcut your users can use to interact with your app."
                " As shortcuts mostly consist of two or more keys, Leptonic also provide the "<Code inline=true>"<KbdShortcut>"</Code>
                " component to make this task as easy as possible. Simply provide the keys which must be pressed in order to activate the shortcut."
            </p>

            <p>"Note that these component do not listen for key-presses. Their sole purpose is to unify rendering of key caps and shortcuts!"</p>

            <DemoShell source=include_str!("demos/kbd_shortcut.rs")>
                <KbdShortcutDemo />
            </DemoShell>

            <p>"This could also be rendered manually using the following markup."</p>

            <DemoShell source=include_str!("demos/kbd_manual.rs")>
                <KbdManualDemo />
            </DemoShell>

            <h2 id="keys" class="anchor">
                "Keys"
                <AnchorLink href="#keys" description="Direct link to section: Keys"/>
            </h2>

            <p>"Here is a list of all keys provided by the "<Code inline=true>"Key"</Code>" enum."</p>

            {
                known_keys
                    .map(|key| view! {
                        <KbdKey key/>
                    })
                    .collect_view()
            }

            <p>"If you need custom content in a "<Code inline=true>"<Kbd>"</Code>" element, use the "<Code inline=true>"Key::Other(Cow::Borrowed(...))"</Code>" variant."</p>

            <DemoShell source=include_str!("demos/kbd_custom.rs")>
                <KbdCustomDemo />
            </DemoShell>

            <h2 id="styling" class="anchor">
                "Styling"
                <AnchorLink href="#styling" description="Direct link to section: Styling"/>
            </h2>

            <p>"You may overwrite any of the following CSS variables to meet your styling needs."</p>

            <Code language=Language::Rust>
                {indoc!(r"
                    --leptonic-kbd-key-color
                    --leptonic-kbd-key-background-color
                    --leptonic-kbd-key-margin
                    --leptonic-kbd-key-padding
                    --leptonic-kbd-key-border-radius
                    --leptonic-kbd-key-border-color
                    --leptonic-kbd-concatenate-color
                    --leptonic-kbd-concatenate-background-color
                    --leptonic-kbd-concatenate-margin
                    --leptonic-kbd-concatenate-padding
                    --leptonic-kbd-concatenate-border-radius
                ")}
            </Code>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Keyboard", link: "#keyboard" },
                Toc::Leaf { title: "Shortcuts", link: "#shortcuts" },
                Toc::Leaf { title: "Keys", link: "#keys" },
                Toc::Leaf { title: "Styling", link: "#styling" },
            ]
        }/>
    }
}
