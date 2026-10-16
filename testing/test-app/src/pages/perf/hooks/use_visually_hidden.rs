//! `use_visually_hidden` on a plain `<div>` (`#perf-hidden`) around a button (`#perf-skip`), for
//! its performance test (`leptonic/tests/perf_tests/hooks/use_visually_hidden.rs`): after a
//! plain button (`#perf-before`) to tab from, before another (`#perf-after`), mounted at first
//! and focusable (shown while focus is within it). Its controls ([`Control`]) mount and unmount
//! it, and switch whether it is focusable.

use leptonic::{
    IntoAttrs,
    hooks::visually_hidden::{UseVisuallyHiddenInput, use_visually_hidden},
};
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
        <h1>"use_visually_hidden performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <Hidden focusable />
        </Show>
        <button id="perf-after">"After"</button>
    }
}

/// The `<div>` with `use_visually_hidden`.
#[component]
fn Hidden(focusable: RwSignal<bool>) -> impl IntoView {
    let hidden = use_visually_hidden(UseVisuallyHiddenInput {
        is_focusable: focusable.into(),
    });
    view! {
        <div id="perf-hidden" {..hidden.props.into_attrs()}>
            <button id="perf-skip">"Skip to content"</button>
        </div>
    }
}
