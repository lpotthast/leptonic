use leptonic::{components::prelude::*, utils::key::KeyboardKey};
use leptos::prelude::*;

#[component]
pub fn KbdManualDemo() -> impl IntoView {
    view! {
        // The same as `KbdShortcut`, built from its parts.
        <p>
            "Send the message with "
            <KbdShortcutRoot>
                <KbdKey key=KeyboardKey::Control/>
                <KbdConcatenate/>
                <KbdKey key=KeyboardKey::Enter/>
            </KbdShortcutRoot>
            "."
        </p>
    }
}
