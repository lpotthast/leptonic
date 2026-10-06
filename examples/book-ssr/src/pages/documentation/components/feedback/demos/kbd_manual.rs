use leptonic::{components::prelude::*, utils::key::KeyboardKey};
use leptos::prelude::*;

#[component]
pub fn KbdManualDemo() -> impl IntoView {
    view! {
        <KbdShortcutRoot>
            <KbdKey key=KeyboardKey::Command/>
            <KbdConcatenate with="+"/>
            <KbdKey key=KeyboardKey::Enter/>
        </KbdShortcutRoot>
    }
}
