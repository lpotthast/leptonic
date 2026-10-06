use leptonic::{components::prelude::*, utils::key::KeyboardKey};
use leptos::prelude::*;

#[component]
pub fn KbdShortcutDemo() -> impl IntoView {
    view! {
        <KbdShortcut keys=[KeyboardKey::Command, KeyboardKey::Enter]/>
    }
}
