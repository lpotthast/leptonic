use leptonic::{components::prelude::*, utils::key::Key};
use leptos::prelude::*;

#[component]
pub fn KbdSingleDemo() -> impl IntoView {
    view! {
        <KbdKey key=Key::Option/>
    }
}
