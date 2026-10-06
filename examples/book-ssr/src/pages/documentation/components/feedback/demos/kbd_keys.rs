use leptonic::{components::prelude::*, utils::key::KeyboardKey};
use leptos::prelude::*;

#[component]
pub fn KbdKeysDemo() -> impl IntoView {
    view! {
        <div class="demo-key-row">
            {KeyboardKey::known_keys().map(|key| view! { <KbdKey key/> }).collect_view()}
        </div>
    }
}
