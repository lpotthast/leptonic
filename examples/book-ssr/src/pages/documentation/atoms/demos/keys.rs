use leptonic::{KeyboardKey, atoms::kbd::Keys};
use leptos::prelude::*;

#[component]
pub fn KeysDemo() -> impl IntoView {
    view! {
        // The keys as given, the same on every platform.
        <p>
            "Press "<Keys keys=vec![KeyboardKey::Escape] classes="demo-shortcut-keys"/>" to discard the draft, "
            <Keys keys=vec![KeyboardKey::Control, KeyboardKey::Enter] classes="demo-shortcut-keys"/>" to send it."
        </p>
    }
}
