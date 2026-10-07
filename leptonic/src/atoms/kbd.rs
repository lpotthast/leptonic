// Upstream: react-aria-components/src/Keyboard.tsx @ 99e6102368
//! Headless keyboard shortcut display.

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## ADDITIONS
// - The keys are rendered from typed values (`Shortcut`, `KeyboardKey`): one `<kbd>` per key,
//   with names for glyphs, in the platform's form for `ShortcutKeys`. react-aria-components'
//   `Keyboard` is a plain `<kbd dir="ltr">` around the app's text.
//
// =============================================================================
use leptos::prelude::*;

use crate::{
    Language,
    utils::{
        classes::Classes, default_class::with_default_class, key::KeyboardKey,
        keyboard_shortcut::Shortcut, platform::device::is_mac, styles::Styles,
        visually_hidden::visually_hidden_styles,
    },
};

/// A keyboard shortcut as keys: a `<kbd>` holding one `<kbd>` per key, modifiers first in the
/// platform's order, glyphs and abbreviations read by their names (e.g. "⌘" as "Command").
///
/// The server can't know the platform: it and the first client render show the generic form
/// ("Ctrl + K"), then (after hydration) the platform's ("⌃⌥⇧⌘", no separators, on Apple
/// platforms: "⌘K"). Style the keys with `kbd kbd`, the separators with `[data-separator]`.
///
/// Default class: `leptonic-ShortcutKeys`.
#[component]
pub fn ShortcutKeys(
    shortcut: Shortcut,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
) -> impl IntoView {
    let classes = with_default_class("leptonic-ShortcutKeys", classes);
    let apple = RwSignal::new(false);
    // As `Shortcut::matches` (react-aria's `Mod`): Command where `isMac()`.
    Effect::new(move |_| apple.set(is_mac()));
    let shortcut = StoredValue::new(shortcut);
    let keys = move || {
        let apple = apple.get();
        render_keys(shortcut.with_value(|shortcut| shortcut.keys(apple)), !apple)
    };
    // Left to right also in right-to-left text, as react-aria-components' `Keyboard`.
    view! {
        <kbd dir="ltr" class=classes style=styles>
            {keys}
        </kbd>
    }
}

/// Keys as given, e.g. for documentation ("Command + X" on every platform; [`ShortcutKeys`] shows
/// a shortcut in the reader's platform's form): a `<kbd>` holding one `<kbd>` per key, glyphs and
/// abbreviations read by their names, with `+` separators between them (`separators=false`
/// writes them together, as Apple platforms do: "⌘X").
///
/// Default class: `leptonic-Keys`.
#[component]
pub fn Keys(
    #[prop(into)] keys: Signal<Vec<KeyboardKey>>,
    #[prop(default = true)] separators: bool,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
) -> impl IntoView {
    let classes = with_default_class("leptonic-Keys", classes);
    view! {
        <kbd dir="ltr" class=classes style=styles>
            {move || render_keys(keys.get(), separators)}
        </kbd>
    }
}

/// One `<kbd>` per key, with `+` separators between them if `separators`.
fn render_keys(keys: Vec<KeyboardKey>, separators: bool) -> impl IntoView {
    let count = keys.len();
    keys.into_iter()
        .enumerate()
        .map(|(index, key)| {
            let display = key.display(Language::En).to_owned();
            let content = match key.spoken_name() {
                Some(name) => view! {
                    <span aria-hidden="true">{display}</span>
                    <span style=visually_hidden_styles()>{name}</span>
                }
                .into_any(),
                None => display.into_any(),
            };
            let separator = (separators && index + 1 < count)
                .then(|| view! { <span data-separator="" aria-hidden="true">"+"</span> });
            view! {
                <kbd>{content}</kbd>
                {separator}
            }
        })
        .collect_view()
}
