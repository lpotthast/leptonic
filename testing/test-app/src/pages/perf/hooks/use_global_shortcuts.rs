//! `use_global_shortcuts` in a component of its own, binding `Mod+K` anywhere and `/` outside
//! text fields, for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_global_shortcuts.rs`): with a plain button
//! (`#perf-before`) and a text input (`#perf-input`) to type in, mounted at first. Its controls
//! ([`Control`]) mount and unmount it, and read how often each shortcut ran (`palette`, `slash`).

use leptonic::{
    KeyboardKey, KeyboardShortcuts, Shortcut,
    hooks::interactions::{UseGlobalShortcutsInput, use_global_shortcuts},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let palette_count = StoredValue::new(0_u32);
    let slash_count = StoredValue::new(0_u32);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::read("palette", move || f64::from(palette_count.get_value())),
        Control::read("slash", move || f64::from(slash_count.get_value())),
    ];
    view! {
        <h1>"use_global_shortcuts performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <input id="perf-input" aria-label="Text" />
        <Show when=move || mounted.get()>
            <Shortcuts palette_count slash_count />
        </Show>
    }
}

/// Binds the shortcuts while mounted.
#[component]
fn Shortcuts(palette_count: StoredValue<u32>, slash_count: StoredValue<u32>) -> impl IntoView {
    use_global_shortcuts(UseGlobalShortcutsInput {
        anywhere: KeyboardShortcuts::new().on(Shortcut::new(KeyboardKey::K).primary(), move |_| {
            palette_count.update_value(|count| *count += 1);
        }),
        outside_text_fields: KeyboardShortcuts::new().on(
            Shortcut::new(KeyboardKey::Slash),
            move |_| {
                slash_count.update_value(|count| *count += 1);
            },
        ),
    });
    view! { <p id="perf-shortcuts">"Shortcuts bound"</p> }
}
