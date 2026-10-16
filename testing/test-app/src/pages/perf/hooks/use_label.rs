//! `use_label` on a `<label>` (`#perf-label`) and an `<input>` (`#perf-input`), for its
//! performance test (`leptonic/tests/perf_tests/hooks/use_label.rs`): mounted at first, with a
//! visible label, beside a further label (`#perf-other-label`), between two plain buttons
//! (`#perf-before`, `#perf-after`). Its controls ([`Control`]) mount and unmount it and change
//! its props: whether the label is visible, a further label (`aria-labelledby`), an
//! `aria-label`.

use leptonic::{
    IntoAttrs,
    hooks::form::{LabelElementType, UseLabelInput, use_label},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let has_label = RwSignal::new(true);
    let labelled_by = RwSignal::new(None::<String>);
    let aria_label = RwSignal::new(None::<String>);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("show-label", move || has_label.set(true)),
        Control::action("hide-label", move || has_label.set(false)),
        Control::action("add-other-label", move || {
            labelled_by.set(Some("perf-other-label".to_owned()));
        }),
        Control::action("remove-other-label", move || labelled_by.set(None)),
        Control::action("set-aria-label", move || {
            aria_label.set(Some("Full name".to_owned()));
        }),
        Control::action("unset-aria-label", move || aria_label.set(None)),
    ];
    view! {
        <h1>"use_label performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <span id="perf-other-label">"Person"</span>
        <Show when=move || mounted.get()>
            <LabelledInput has_label labelled_by aria_label />
        </Show>
        <button id="perf-after">"After"</button>
    }
}

/// The label and the input it labels.
#[component]
fn LabelledInput(
    has_label: RwSignal<bool>,
    labelled_by: RwSignal<Option<String>>,
    aria_label: RwSignal<Option<String>>,
) -> impl IntoView {
    let label = use_label(UseLabelInput {
        id: Some("perf-input".to_owned()),
        label_id: Some("perf-label".to_owned()),
        has_label: has_label.into(),
        label_element_type: LabelElementType::Label,
        aria_label: aria_label.into(),
        aria_labelledby: labelled_by.into(),
    });
    view! {
        <label {..label.label_props.into_attrs()}>"Name"</label>
        <input {..label.field_props.into_attrs()} />
    }
}
