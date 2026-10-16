//! `use_overlay_focus_contain` alone, for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_overlay_focus_contain.rs`): called in an overlay (a fresh
//! `OverlayFocusContain` context per mount, whose `contain()` it sets, shown in
//! `#perf-contain`), and outside any overlay (`#perf-outside`, where it does nothing). Both are
//! mounted at first. Its controls ([`Control`]) mount and unmount each.

use leptonic::hooks::overlay::{OverlayFocusContain, use_overlay_focus_contain};
use leptos::{context::Provider, prelude::*};

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let inside = RwSignal::new(true);
    let outside = RwSignal::new(true);
    let controls = vec![
        Control::action("mount", move || inside.set(true)),
        Control::action("unmount", move || inside.set(false)),
        Control::action("mount-outside", move || outside.set(true)),
        Control::action("unmount-outside", move || outside.set(false)),
    ];
    view! {
        <h1>"use_overlay_focus_contain performance"</h1>
        <PerfControls controls />
        <Show when=move || inside.get()>
            {
                let overlay = OverlayFocusContain::new();
                let contain = overlay.contain();
                view! {
                    <Provider value=overlay>
                        <Subject />
                    </Provider>
                    <span id="perf-contain">{move || contain.get().to_string()}</span>
                }
            }
        </Show>
        <Show when=move || outside.get()>
            <Subject />
            <span id="perf-outside">"Outside"</span>
        </Show>
    }
}

/// Calls the hook.
#[component]
fn Subject() -> impl IntoView {
    use_overlay_focus_contain();
}
