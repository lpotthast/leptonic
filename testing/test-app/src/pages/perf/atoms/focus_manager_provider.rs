//! The `FocusManagerProvider` alone (`.perf-manager`), around 10 buttons (or 1,000: controls
//! `use-10`, `use-1000`; `#perf-manager-0` to `#perf-manager-<n - 1>`) whose arrow keys, Home
//! and End move the focus with its focus manager (wrapping around), for its performance test
//! (`leptonic/tests/perf_tests/atoms/focus_manager_provider.rs`): mounted at first. Its controls
//! ([`Control`]) mount and unmount it.

use leptonic::{
    atoms::focus_manager::FocusManagerProvider,
    hooks::focus::{FocusManager, FocusManagerOptions},
};
use leptos::{ev, prelude::*};

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let buttons = RwSignal::new(10_usize);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("use-10", move || buttons.set(10)),
        Control::action("use-1000", move || buttons.set(1000)),
    ];
    view! {
        <h1>"FocusManagerProvider performance"</h1>
        <PerfControls controls />
        <Show when=move || mounted.get()>
            <FocusManagerProvider classes="perf-manager" let:manager>
                <div on:keydown=move |e| move_focus(manager, &e)>
                    {move || {
                        (0..buttons.get())
                            .map(|i| view! { <button id=format!("perf-manager-{i}")>{i}</button> })
                            .collect_view()
                    }}
                </div>
            </FocusManagerProvider>
        </Show>
    }
}

/// Moves the focus for the arrow keys, Home and End.
fn move_focus(manager: FocusManager, e: &ev::KeyboardEvent) {
    let wrap = FocusManagerOptions {
        wrap: true,
        ..FocusManagerOptions::default()
    };
    let moved = match e.key().as_str() {
        "ArrowRight" => manager.focus_next(wrap),
        "ArrowLeft" => manager.focus_previous(wrap),
        "Home" => manager.focus_first(FocusManagerOptions::default()),
        "End" => manager.focus_last(FocusManagerOptions::default()),
        _ => return,
    };
    if moved.is_some() {
        e.prevent_default();
    }
}
