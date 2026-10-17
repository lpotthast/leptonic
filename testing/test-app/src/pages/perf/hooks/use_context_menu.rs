//! `use_context_menu` on a focusable `<div>` (`#perf-context-menu`), for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_context_menu.rs`): after a plain button
//! (`#perf-before`) to tab from, mounted at first, counting context menu requests. Its controls
//! ([`Control`]) mount and unmount it, and read the count (`requests`).

use leptonic::hooks::interactions::{UseContextMenuInput, use_context_menu};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let request_count = StoredValue::new(0_u32);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::read("requests", move || f64::from(request_count.get_value())),
    ];
    view! {
        <h1>"use_context_menu performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <WithContextMenu request_count />
        </Show>
    }
}

/// The element with `use_context_menu`.
#[component]
fn WithContextMenu(request_count: StoredValue<u32>) -> impl IntoView {
    let context_menu = use_context_menu(UseContextMenuInput {
        on_context_menu: Some(Callback::new(move |_| {
            request_count.update_value(|count| *count += 1);
        })),
    });
    view! {
        <div id="perf-context-menu" tabindex="0" {..context_menu.target.attrs()}>
            "Right-click me"
        </div>
    }
}
