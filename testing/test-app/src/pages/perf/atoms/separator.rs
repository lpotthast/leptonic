//! The `Separator` alone (`.perf-separator`: a class, which stays when it switches between
//! `<hr>` and `<div>`), for its performance test
//! (`leptonic/tests/perf_tests/atoms/separator.rs`): horizontal and mounted at first. Its controls
//! ([`Control`]) mount and unmount it, and change its orientation and label.

use leptonic::{Orientation, atoms::separator::Separator};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let orientation = RwSignal::new(Orientation::Horizontal);
    let label = RwSignal::new(None::<String>);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("vertical", move || orientation.set(Orientation::Vertical)),
        Control::action("horizontal", move || {
            orientation.set(Orientation::Horizontal);
        }),
        Control::action("label", move || label.set(Some("Section".to_owned()))),
        Control::action("unlabel", move || label.set(None)),
    ];
    view! {
        <h1>"Separator performance"</h1>
        <PerfControls controls />
        <Show when=move || mounted.get()>
            <Separator orientation=orientation aria_label=label classes="perf-separator" />
        </Show>
    }
}
