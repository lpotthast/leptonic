//! `use_toolbar` on a plain `<div>` (`#perf-toolbar`, labelled "Tools") of 5 plain buttons
//! (`#perf-tool-0` to `#perf-tool-4`), for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_toolbar.rs`): between two plain buttons
//! (`#perf-before`, `#perf-after`) to tab from and to, mounted at first. Its controls
//! ([`Control`]) mount and unmount it, and change its orientation and label.

use leptonic::{
    CapturedElement, Orientation,
    hooks::toolbar::{UseToolbarInput, use_toolbar},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let orientation = RwSignal::new(Orientation::Horizontal);
    let label = RwSignal::new("Tools".to_owned());
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("vertical", move || orientation.set(Orientation::Vertical)),
        Control::action("horizontal", move || {
            orientation.set(Orientation::Horizontal);
        }),
        Control::action("rename", move || label.set("Editing".to_owned())),
    ];
    view! {
        <h1>"use_toolbar performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <PlainToolbar orientation label />
        </Show>
        <button id="perf-after">"After"</button>
    }
}

/// The `<div>` with `use_toolbar`.
#[component]
fn PlainToolbar(orientation: RwSignal<Orientation>, label: RwSignal<String>) -> impl IntoView {
    let toolbar = use_toolbar(UseToolbarInput {
        element: CapturedElement::new(),
        orientation: orientation.into(),
        aria_label: label.into(),
        aria_labelledby: Signal::default(),
    });
    view! {
        <div id="perf-toolbar" {..toolbar.toolbar.attrs()}>
            {(0..5)
                .map(|i| view! { <button id=format!("perf-tool-{i}")>{i}</button> })
                .collect_view()}
        </div>
    }
}
