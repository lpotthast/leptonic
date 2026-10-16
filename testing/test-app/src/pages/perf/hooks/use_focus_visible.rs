//! `use_focus_visible` in a component of its own, showing its result as `data-visible` on
//! `#perf-focus-visible`, for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_focus_visible.rs`): between two plain buttons
//! (`#perf-before`, `#perf-after`) to click and type on, mounted at first. Its controls
//! ([`Control`]) mount and unmount it.

use leptonic::hooks::focus::{UseFocusVisibleInput, use_focus_visible};
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
        <h1>"use_focus_visible performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <FocusVisible />
        </Show>
        <button id="perf-after">"After"</button>
    }
}

/// Shows `use_focus_visible`'s result.
#[component]
fn FocusVisible() -> impl IntoView {
    let visible = use_focus_visible(UseFocusVisibleInput::default()).focus_should_be_visible;
    view! { <div id="perf-focus-visible" data-visible=move || visible.get().to_string()></div> }
}
