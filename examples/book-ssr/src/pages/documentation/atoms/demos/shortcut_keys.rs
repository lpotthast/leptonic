use leptonic::{atoms::prelude as atoms, utils::keyboard_shortcut::Shortcut};
use leptos::prelude::*;

#[component]
pub fn ShortcutKeysDemo() -> impl IntoView {
    let shortcuts = [
        ("Search", Shortcut::key("k").primary()),
        ("Redo", Shortcut::key("z").primary().shift()),
        ("Move line down", Shortcut::key("ArrowDown").alt()),
        ("Close", Shortcut::key("Escape")),
    ];

    view! {
        <dl class="demo-shortcut-list">
            {shortcuts
                .into_iter()
                .map(|(action, shortcut)| view! {
                    <dt>{action}</dt>
                    <dd><atoms::ShortcutKeys shortcut classes="demo-shortcut-keys"/></dd>
                })
                .collect_view()}
        </dl>
    }
}
