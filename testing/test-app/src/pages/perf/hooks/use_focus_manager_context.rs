//! `use_focus_manager_context` in a group of 10 buttons (`#perf-group`, `#perf-button-0` to
//! `#perf-button-9`) inside a `FocusManagerProvider`, whose arrow keys, Home and End move the
//! focus with the focus manager the hook finds (wrapping around), for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_focus_manager_context.rs`): mounted at first. Its
//! controls ([`Control`]) mount and unmount the group (the provider stays).

use leptonic::{
    atoms::focus_manager::FocusManagerProvider,
    hooks::focus::{FocusManagerOptions, use_focus_manager_context},
};
use leptos::{ev, prelude::*};

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
    ];
    view! {
        <h1>"use_focus_manager_context performance"</h1>
        <PerfControls controls />
        <FocusManagerProvider let:_manager>
            <Show when=move || mounted.get()>
                <Group />
            </Show>
        </FocusManagerProvider>
    }
}

/// The buttons, moving the focus with the manager `use_focus_manager_context` finds.
#[component]
fn Group() -> impl IntoView {
    let manager = use_focus_manager_context();
    let move_focus = move |e: ev::KeyboardEvent| {
        let Some(manager) = manager else {
            return;
        };
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
    };
    view! {
        <div id="perf-group" on:keydown=move_focus>
            {(0..10)
                .map(|i| view! { <button id=format!("perf-button-{i}")>{i}</button> })
                .collect_view()}
        </div>
    }
}
