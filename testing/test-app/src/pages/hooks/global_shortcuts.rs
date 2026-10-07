use leptonic::{
    atoms::{
        field::Label,
        input::Input,
        kbd::{Keys, ShortcutKeys},
        text_field::TextField,
    },
    hooks::{UseGlobalShortcutsInput, use_global_shortcuts},
    utils::{
        key::KeyboardKey,
        keyboard_shortcut::{KeyboardShortcuts, Shortcut},
    },
};
use leptos::{html, prelude::*};

/// Document-wide shortcuts: `Mod+K` (anywhere, also in text fields) counts in `#test-gs-palette`,
/// a bare `/` (not while typing) focuses the `TextField` atom's input in `#test-gs-filter` (through
/// the `Input`'s `node_ref`; it carries `aria-keyshortcuts="/"`), `?` (typed with Shift) counts in
/// `#test-gs-help`. `#test-gs-keys` shows `Mod+K`, `#test-gs-literal` the literal keys Command + X. `#test-gs-nested-toggle` mounts a later
/// binding of `Mod+K` counting in `#test-gs-nested` instead.
#[component]
pub fn PageHookGlobalShortcuts() -> impl IntoView {
    let palette = RwSignal::new(0);
    let help = RwSignal::new(0);
    let nested = RwSignal::new(0);
    let show_nested = RwSignal::new(false);
    let filter = NodeRef::<html::Input>::new();
    use_global_shortcuts(UseGlobalShortcutsInput {
        anywhere: KeyboardShortcuts::new().on(Shortcut::key("k").primary(), move |_| {
            palette.update(|count| *count += 1);
        }),
        outside_text_fields: KeyboardShortcuts::new()
            .on(Shortcut::key("/"), move |_| {
                if let Some(filter) = filter.get_untracked() {
                    let _ = filter.focus();
                }
            })
            .on(Shortcut::key("?"), move |_| {
                help.update(|count| *count += 1)
            }),
    });
    view! {
        <h1>"Global shortcuts"</h1>
        <button id="test-gs-before">"Before"</button>
        <div id="test-gs-filter">
            <TextField>
                <Label>"Filter"</Label>
                <Input
                    node_ref=filter
                    attr:aria-keyshortcuts=Shortcut::key("/").to_aria_keyshortcuts(false)
                />
            </TextField>
        </div>
        <input id="test-gs-other" aria-label="Other" />
        <p>"Palette opened: " <span id="test-gs-palette">{move || palette.get()}</span></p>
        <p>"Help opened: " <span id="test-gs-help">{move || help.get()}</span></p>
        <button id="test-gs-nested-toggle" on:click=move |_| show_nested.update(|shown| *shown = !*shown)>
            "Toggle nested"
        </button>
        <p>"Nested: " <span id="test-gs-nested">{move || nested.get()}</span></p>
        <Show when=move || show_nested.get()>
            <NestedShortcuts count=nested />
        </Show>
        <ShortcutKeys shortcut=Shortcut::key("k").primary() attr:id="test-gs-keys" />
        <Keys keys=vec![KeyboardKey::Command, KeyboardKey::X] attr:id="test-gs-literal" />
    }
}

/// Binds `Mod+K` while mounted, over the page's binding.
#[component]
fn NestedShortcuts(count: RwSignal<i32>) -> impl IntoView {
    use_global_shortcuts(UseGlobalShortcutsInput {
        anywhere: KeyboardShortcuts::new().on(Shortcut::key("k").primary(), move |_| {
            count.update(|count| *count += 1);
        }),
        ..UseGlobalShortcutsInput::default()
    });
    view! { <p id="test-gs-nested-shown">"Nested shortcuts bound"</p> }
}
