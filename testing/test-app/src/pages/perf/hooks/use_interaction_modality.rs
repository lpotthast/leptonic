//! `use_interaction_modality` in a component of its own, showing the modality as
//! `data-modality` on `#perf-modality` (`none` before the first interaction), for its
//! performance test (`leptonic/tests/perf_tests/hooks/use_interaction_modality.rs`): between two
//! plain buttons (`#perf-before`, `#perf-after`) to click and type on, mounted at first. Its
//! controls ([`Control`]) mount and unmount it.

use leptonic::hooks::focus::{Modality, use_interaction_modality};
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
        <h1>"use_interaction_modality performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <InteractionModality />
        </Show>
        <button id="perf-after">"After"</button>
    }
}

/// Shows the interaction modality.
#[component]
fn InteractionModality() -> impl IntoView {
    let modality = use_interaction_modality();
    let name = move || match modality.get() {
        None => "none",
        Some(Modality::Pointer) => "pointer",
        Some(Modality::Keyboard) => "keyboard",
        Some(Modality::Virtual) => "virtual",
    };
    view! { <div id="perf-modality" data-modality=name></div> }
}
