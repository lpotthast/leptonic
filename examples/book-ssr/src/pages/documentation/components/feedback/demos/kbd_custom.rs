use std::borrow::Cow;

use leptonic::{components::prelude::*, utils::key::KeyboardKey};
use leptos::prelude::*;

#[component]
pub fn KbdCustomDemo() -> impl IntoView {
    view! {
        <KbdKey key=KeyboardKey::Other(Cow::Borrowed("Foo"))/>
    }
}
