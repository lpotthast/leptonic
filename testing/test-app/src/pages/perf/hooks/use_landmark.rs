//! `use_landmark` on 2 regions (or 50: controls `use-2`, `use-50`; `#perf-landmark-0` to
//! `#perf-landmark-<n - 1>`, named "Region <i>", each with a button `#perf-button-<i>`), for
//! its performance test (`leptonic/tests/perf_tests/hooks/use_landmark.rs`): mounted at first.
//! Its controls ([`Control`]) mount and unmount them, and rename the first one (`rename`).

use leptonic::{
    CapturedElement, IntoAttrs,
    hooks::landmark::{LandmarkRole, UseLandmarkInput, use_landmark},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let landmarks = RwSignal::new(2_usize);
    let first_label = RwSignal::new("Region 0".to_owned());
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("use-2", move || landmarks.set(2)),
        Control::action("use-50", move || landmarks.set(50)),
        Control::action("rename", move || first_label.set("Renamed".to_owned())),
    ];
    view! {
        <h1>"use_landmark performance"</h1>
        <PerfControls controls />
        <Show when=move || mounted.get()>
            {move || {
                (0..landmarks.get())
                    .map(|i| {
                        let label = if i == 0 {
                            first_label.into()
                        } else {
                            Signal::stored(format!("Region {i}"))
                        };
                        view! { <Landmark i label /> }
                    })
                    .collect_view()
            }}
        </Show>
    }
}

/// The region `i` with `use_landmark`.
#[component]
fn Landmark(i: usize, label: Signal<String>) -> impl IntoView {
    let element = CapturedElement::new();
    let landmark = use_landmark(UseLandmarkInput {
        element,
        role: LandmarkRole::Region,
        aria_label: label.into(),
        aria_labelledby: Signal::default(),
        focus: None,
    });
    view! {
        <div id=format!("perf-landmark-{i}") {..landmark.props.into_attrs()} {..element.attr()}>
            <button id=format!("perf-button-{i}")>{format!("Button {i}")}</button>
        </div>
    }
}
