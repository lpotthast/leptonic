//! `use_prevent_scroll` alone, for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_prevent_scroll.rs`): on a page taller than the viewport
//! (scrolling, with a scrollbar), mounted at first, disabled (not preventing scrolling; shown in
//! `#perf-preventing`). Its controls ([`Control`]) mount and unmount it, and enable and disable it.

use leptonic::hooks::overlay::{UsePreventScrollInput, use_prevent_scroll};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let preventing = RwSignal::new(false);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("enable", move || preventing.set(true)),
        Control::action("disable", move || preventing.set(false)),
    ];
    view! {
        <h1>"use_prevent_scroll performance"</h1>
        <PerfControls controls />
        <span id="perf-preventing">{move || preventing.get().to_string()}</span>
        <Show when=move || mounted.get()>
            <Subject preventing=preventing.into() />
        </Show>
        <div style="height: 3000px;">"A tall page"</div>
    }
}

/// Calls the hook.
#[component]
fn Subject(preventing: Signal<bool>) -> impl IntoView {
    use_prevent_scroll(UsePreventScrollInput {
        is_disabled: Signal::derive(move || !preventing.get()),
    });
    view! { <span id="perf-subject">"Mounted"</span> }
}
