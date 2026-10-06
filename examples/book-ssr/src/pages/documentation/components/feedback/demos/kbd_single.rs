use leptonic::{components::prelude::*, utils::key::KeyboardKey};
use leptos::prelude::*;

#[component]
pub fn KbdSingleDemo() -> impl IntoView {
    view! {
        <p>"Press " <KbdKey key=KeyboardKey::Escape/> " to close the dialog."</p>
    }
}
