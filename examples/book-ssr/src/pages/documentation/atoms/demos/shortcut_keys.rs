use leptonic::{KeyboardKey, Shortcut, atoms};
use leptos::prelude::*;

#[component]
pub fn ShortcutKeysDemo() -> impl IntoView {
    let shortcuts = [
        ("Search", Shortcut::new(KeyboardKey::K).primary()),
        ("Redo", Shortcut::new(KeyboardKey::Z).primary().shift()),
        (
            "Move line down",
            Shortcut::new(KeyboardKey::ArrowDown).alt(),
        ),
        ("Close", Shortcut::new(KeyboardKey::Escape)),
    ];

    view! {
        <dl class="demo-shortcut-list">
            {shortcuts
                .into_iter()
                .map(|(action, shortcut)| view! {
                    <dt>{action}</dt>
                    <dd><atoms::kbd::ShortcutKeys shortcut classes="demo-shortcut-keys"/></dd>
                })
                .collect_view()}
        </dl>
    }
}
