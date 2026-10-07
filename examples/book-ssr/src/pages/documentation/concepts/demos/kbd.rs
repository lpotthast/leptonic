use leptonic::{components::prelude::*, utils::key::KeyboardKey};
use leptos::prelude::*;

#[component]
pub fn KbdConceptDemo() -> impl IntoView {
    view! {
        <p>
            "Press "<KbdKey key=KeyboardKey::Escape/>" to discard the draft, "
            <KbdShortcut keys=[KeyboardKey::Control, KeyboardKey::Enter]/>" to send it."
        </p>
    }
}
