//! The `Keys` alone (`.perf-keys`, Control + Shift + K at first), for its performance test
//! (`leptonic/tests/perf_tests/atoms/keys.rs`): mounted at first. Its controls ([`Control`]) mount
//! and unmount it, and change its keys (`two-keys`: Alt + Enter, `three-keys`: back).

use leptonic::{KeyboardKey, atoms::kbd::Keys};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// Control + Shift + K.
fn three_keys() -> Vec<KeyboardKey> {
    vec![KeyboardKey::Control, KeyboardKey::Shift, KeyboardKey::K]
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let keys = RwSignal::new(three_keys());
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("two-keys", move || {
            keys.set(vec![KeyboardKey::Alt, KeyboardKey::Enter]);
        }),
        Control::action("three-keys", move || keys.set(three_keys())),
    ];
    view! {
        <h1>"Keys performance"</h1>
        <PerfControls controls />
        <Show when=move || mounted.get()>
            <Keys keys=keys classes="perf-keys" />
        </Show>
    }
}
