//! `use_separator` on a plain `<div>` (`#perf-separator`), for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_separator.rs`): horizontal and mounted at first. Its
//! controls ([`Control`]) mount and unmount it, and change its orientation and label.

use leptonic::{
    Orientation,
    hooks::separator::{SeparatorElementType, UseSeparatorInput, use_separator},
};
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
        <h1>"use_separator performance"</h1>
        <PerfControls controls />
        <Show when=move || mounted.get()>
            <PlainSeparator orientation label />
        </Show>
    }
}

/// The `<div>` with `use_separator`.
#[component]
fn PlainSeparator(
    orientation: RwSignal<Orientation>,
    label: RwSignal<Option<String>>,
) -> impl IntoView {
    let separator = use_separator(UseSeparatorInput {
        orientation: orientation.into(),
        element_type: SeparatorElementType::Div,
        id: Some("perf-separator".into()),
        aria_label: label.into(),
        aria_labelledby: Signal::default(),
    });
    view! { <div {..separator.separator.attrs()}></div> }
}
