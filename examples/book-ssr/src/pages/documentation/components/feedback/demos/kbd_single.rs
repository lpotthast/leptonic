use leptonic::{components::prelude::*, utils::key::KeyboardKey};
use leptos::prelude::*;

#[component]
pub fn KbdSingleDemo() -> impl IntoView {
    view! {
        <KbdKey key=KeyboardKey::Option/>
    }
}
