use std::borrow::Cow;

use leptonic::{components::prelude::*, utils::key::Key};
use leptos::prelude::*;

#[component]
pub fn KbdCustomDemo() -> impl IntoView {
    view! {
        <KbdKey key=Key::Other(Cow::Borrowed("Foo"))/>
    }
}
