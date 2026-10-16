//! `use_close_on_scroll` alone, for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_close_on_scroll.rs`): watching a plain trigger button
//! (`#perf-trigger`) in a scrolling container (`#perf-scroller`), next to another scrolling
//! container (`#perf-other-scroller`) not around it, mounted at first, closed. The open state is
//! the app's (shown in `#perf-open`), and its `on_close` closes it. Its controls ([`Control`])
//! mount and unmount it, and open and close it.

use leptonic::{
    CapturedElement,
    hooks::overlay::{UseCloseOnScrollInput, use_close_on_scroll},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let open = RwSignal::new(false);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("open", move || open.set(true)),
        Control::action("close", move || open.set(false)),
    ];
    view! {
        <h1>"use_close_on_scroll performance"</h1>
        <PerfControls controls />
        <span id="perf-open">{move || open.get().to_string()}</span>
        <div id="perf-scroller" style="overflow: auto; height: 100px;">
            <div style="height: 300px;">
                <Show when=move || mounted.get()>
                    <Subject open />
                </Show>
            </div>
        </div>
        <div id="perf-other-scroller" style="overflow: auto; height: 100px;">
            <div style="height: 300px;">"Elsewhere"</div>
        </div>
    }
}

/// Calls the hook on a plain trigger.
#[component]
fn Subject(open: RwSignal<bool>) -> impl IntoView {
    let trigger = CapturedElement::new();
    use_close_on_scroll(UseCloseOnScrollInput {
        is_open: open.into(),
        trigger_element: trigger,
        on_close: Callback::new(move |()| open.set(false)),
    });
    view! {
        <button id="perf-trigger" {..trigger.attr()}>
            "Trigger"
        </button>
    }
}
