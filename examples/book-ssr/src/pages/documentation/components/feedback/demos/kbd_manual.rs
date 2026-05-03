use leptonic::{components::prelude::*, utils::key::Key};
use leptos::prelude::*;

#[component]
pub fn KbdManualDemo() -> impl IntoView {
    view! {
        <KbdShortcutRoot>
            <KbdKey key=Key::Command/>
            <KbdConcatenate with="+"/>
            <KbdKey key=Key::Enter/>
        </KbdShortcutRoot>
    }
}
