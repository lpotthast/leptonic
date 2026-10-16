//! The `Toolbar` alone (`#perf-toolbar`, labelled "Tools"), of 5 plain buttons (or 1,000:
//! controls `use-5`, `use-1000`; `#perf-tool-0` to `#perf-tool-<n - 1>`), for its performance
//! test (`leptonic/tests/perf_tests/atoms/toolbar.rs`): between two plain buttons
//! (`#perf-before`, `#perf-after`) to tab from and to, mounted at first. Its controls
//! ([`Control`]) mount and unmount it, and change its orientation and label.

use leptonic::{Orientation, atoms::toolbar::Toolbar};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let tools = RwSignal::new(5_usize);
    let orientation = RwSignal::new(Orientation::Horizontal);
    let label = RwSignal::new("Tools".to_owned());
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("use-5", move || tools.set(5)),
        Control::action("use-1000", move || tools.set(1000)),
        Control::action("vertical", move || orientation.set(Orientation::Vertical)),
        Control::action("horizontal", move || {
            orientation.set(Orientation::Horizontal);
        }),
        Control::action("rename", move || label.set("Editing".to_owned())),
    ];
    view! {
        <h1>"Toolbar performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <Toolbar attr:id="perf-toolbar" orientation=orientation aria_label=label>
                {move || {
                    (0..tools.get())
                        .map(|i| view! { <button id=format!("perf-tool-{i}")>{i}</button> })
                        .collect_view()
                }}
            </Toolbar>
        </Show>
        <button id="perf-after">"After"</button>
    }
}
