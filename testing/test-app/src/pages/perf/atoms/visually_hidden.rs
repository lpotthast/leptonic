//! The `VisuallyHidden` alone (`.perf-hidden`), around a button (`#perf-skip`, as a "skip to
//! content" link would be), for its performance test
//! (`leptonic/tests/perf_tests/atoms/visually_hidden.rs`): after a plain button (`#perf-before`)
//! to tab from, before another (`#perf-after`), mounted at first and focusable (shown while
//! focus is within it). Its controls ([`Control`]) mount and unmount it, and switch whether it is
//! focusable.

use leptonic::atoms::visually_hidden::VisuallyHidden;
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let focusable = RwSignal::new(true);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("make-focusable", move || focusable.set(true)),
        Control::action("make-unfocusable", move || focusable.set(false)),
    ];
    view! {
        <h1>"VisuallyHidden performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <VisuallyHidden is_focusable=focusable classes="perf-hidden">
                <button id="perf-skip">"Skip to content"</button>
            </VisuallyHidden>
        </Show>
        <button id="perf-after">"After"</button>
    }
}
