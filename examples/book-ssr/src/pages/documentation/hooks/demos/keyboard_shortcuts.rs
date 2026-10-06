use leptonic::{
    hooks::*,
    utils::keyboard_shortcut::{KeyboardShortcuts, Shortcut},
};
use leptos::prelude::*;

#[component]
pub fn KeyboardShortcutsDemo() -> impl IntoView {
    let position = RwSignal::new(0i32);
    let saves = RwSignal::new(0u32);

    let shortcuts = KeyboardShortcuts::new()
        .on(Shortcut::key("ArrowRight"), move |_| {
            position.update(|p| *p += 1);
        })
        .on(Shortcut::key("ArrowLeft"), move |_| {
            position.update(|p| *p -= 1);
        })
        // `primary()` is Command on Apple platforms and Control everywhere else.
        .on(Shortcut::key("s").primary(), move |e| {
            // Repeats are allowed (for the arrow keys), so ignore them here: holding the keys
            // should save once, not over and over.
            if !e.repeat() {
                saves.update(|s| *s += 1);
            }
        })
        // Returning `false` leaves the event alone (no `preventDefault`, it keeps bubbling).
        .on(Shortcut::key("Home"), move |_| {
            if position.get_untracked() == 0 {
                return false;
            }
            position.set(0);
            true
        });

    let UseKeyboardReturn { props } = use_keyboard(UseKeyboardInput {
        shortcuts: Some(shortcuts),
        allow_repeats: true,
        ..Default::default()
    });

    view! {
        <div {..props.into_attrs()} tabindex="0" class="demo-keyboard-target">
            "Focus me, then press the arrow keys, Home, or Control + S (Command + S on a Mac)"
        </div>
        <p class="demo-status">
            {move || {
                let saved = match saves.get() {
                    1 => "1 time".to_owned(),
                    n => format!("{n} times"),
                };
                format!("Position {}, saved {saved}.", position.get())
            }}
        </p>
    }
}
