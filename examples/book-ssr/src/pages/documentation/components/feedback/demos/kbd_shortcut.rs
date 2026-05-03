use leptonic::{components::prelude::*, utils::key::Key};
use leptos::prelude::*;

#[component]
pub fn KbdShortcutDemo() -> impl IntoView {
    view! {
        <KbdShortcut keys=[Key::Command, Key::Enter]/>
    }
}
