//! The hook `use_drag_modality` alone, for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_drag_modality.rs`): `#perf-modality` shows the drag
//! modality it returns (`data-modality`: `keyboard`, `touch` or `virtual`), mounted at first,
//! between two plain buttons (`#perf-before`, `#perf-after`) to tab to and click. Its controls
//! ([`Control`]) mount and unmount it.

use leptonic::hooks::dnd::{DragModality, use_drag_modality};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The modality's name.
fn name(modality: DragModality) -> &'static str {
    match modality {
        DragModality::Keyboard => "keyboard",
        DragModality::Touch => "touch",
        DragModality::Virtual => "virtual",
    }
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
    ];
    view! {
        <h1>"use_drag_modality performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <Modality />
        </Show>
        <button id="perf-after">"After"</button>
    }
}

/// The drag modality.
#[component]
fn Modality() -> impl IntoView {
    let modality = use_drag_modality();
    view! { <p id="perf-modality" data-modality=move || name(modality.get())></p> }
}
