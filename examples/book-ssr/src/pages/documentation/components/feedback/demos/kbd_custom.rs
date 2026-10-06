use std::borrow::Cow;

use leptonic::{components::prelude::*, utils::key::KeyboardKey};
use leptos::prelude::*;

#[component]
pub fn KbdCustomDemo() -> impl IntoView {
    view! {
        <p>"Press the " <KbdKey key=KeyboardKey::Other(Cow::Borrowed("Media Play"))/> " key to start the video."</p>
    }
}
