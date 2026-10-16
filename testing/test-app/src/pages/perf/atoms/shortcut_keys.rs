//! The `ShortcutKeys` alone (`.perf-shortcut-keys`, `Mod+Shift+K`), for its performance test
//! (`leptonic/tests/perf_tests/atoms/shortcut_keys.rs`): mounted at first. Its controls
//! ([`Control`]) mount and unmount it.

use leptonic::{KeyboardKey, Shortcut, atoms::kbd::ShortcutKeys};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
    ];
    view! {
        <h1>"ShortcutKeys performance"</h1>
        <PerfControls controls />
        <Show when=move || mounted.get()>
            <ShortcutKeys
                shortcut=Shortcut::new(KeyboardKey::K).primary().shift()
                classes="perf-shortcut-keys"
            />
        </Show>
    }
}
