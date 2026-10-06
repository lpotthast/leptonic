use leptonic::{components::prelude::*, utils::key::KeyboardKey};
use leptos::prelude::*;

#[component]
pub fn KbdShortcutDemo() -> impl IntoView {
    view! {
        <p>"Send the message with " <KbdShortcut keys=[KeyboardKey::Control, KeyboardKey::Enter]/> "."</p>
    }
}
