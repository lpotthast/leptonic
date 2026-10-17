//! `use_checkbox_group` on a group (`#perf-group`, labelled "Items") of checkboxes
//! (`use_checkbox_group_item`, labelled "Item 0", "Item 1", ...), for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_checkbox_group.rs`): mounted at first with 10
//! checkboxes, between two plain buttons (`#perf-before`, `#perf-after`). Its controls
//! ([`Control`]) mount it with 10 (`mount`) or 1,000 checkboxes (`mount-1000`), unmount it,
//! disable and enable it, and read how many checkboxes are checked (`checked`).

use std::collections::HashSet;

use leptonic::hooks::{
    collections::Key,
    form::{
        CheckboxGroupData, CheckboxGroupState, ToggleOptions, UseCheckboxGroupInput,
        UseCheckboxGroupItemInput, UseCheckboxGroupStateInput, use_checkbox_group,
        use_checkbox_group_item, use_checkbox_group_state,
    },
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let size = RwSignal::new(Some(10_i32));
    let disabled = RwSignal::new(false);
    // The mounted group's state, for the controls.
    let state = StoredValue::new(None::<CheckboxGroupState>);
    let controls = vec![
        Control::action("mount", move || size.set(Some(10))),
        Control::action("mount-1000", move || size.set(Some(1_000))),
        Control::action("unmount", move || size.set(None)),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
        Control::read("checked", move || {
            state.get_value().map_or(0.0, |state| {
                state.value.with_untracked(|value| {
                    f64::from(u32::try_from(value.len()).unwrap_or(u32::MAX))
                })
            })
        }),
    ];
    view! {
        <h1>"use_checkbox_group performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        {move || size.get().map(|size| view! { <Group size disabled state /> })}
        <button id="perf-after">"After"</button>
    }
}

/// The group with `size` checkboxes.
#[component]
fn Group(
    size: i32,
    disabled: RwSignal<bool>,
    state: StoredValue<Option<CheckboxGroupState>>,
) -> impl IntoView {
    let group_state = use_checkbox_group_state(UseCheckboxGroupStateInput {
        default_value: HashSet::new(),
        is_disabled: disabled.into(),
        name: Some("items".to_owned()),
        ..UseCheckboxGroupStateInput::default()
    });
    state.set_value(Some(group_state));
    on_cleanup(move || {
        state.try_update_value(|state| *state = None);
    });
    let group = use_checkbox_group(UseCheckboxGroupInput {
        state: group_state,
        id: Some("perf-group".into()),
        has_label: true.into(),
        aria_label: MaybeProp::default(),
        aria_labelledby: Signal::default(),
        aria_describedby: Signal::default(),
        form: None,
        on_focus: None,
        on_blur: None,
        on_focus_change: None,
    });
    let data = group.data;
    view! {
        <div {..group.group.attrs()}>
            <span {..group.label.attrs()}>"Items"</span>
            {(0..size)
                .map(|item| view! { <GroupCheckbox group=data.clone() item /> })
                .collect_view()}
        </div>
    }
}

/// A checkbox of the group.
#[component]
fn GroupCheckbox(group: CheckboxGroupData, item: i32) -> impl IntoView {
    let checkbox = use_checkbox_group_item(UseCheckboxGroupItemInput {
        group,
        value: Key::from(item),
        is_indeterminate: Signal::stored(false),
        on_change: None,
        options: ToggleOptions::default(),
    });
    let (label_attrs, label_styles) = checkbox
        .label
        .props()
        .expect("the first label takes its props")
        .into_parts();
    let (input_attrs, input_styles) = checkbox
        .input
        .props()
        .expect("the first input takes its props")
        .into_parts();
    view! {
        <label {..label_attrs} style=label_styles>
            <input {..input_attrs} style=input_styles />
            {format!("Item {item}")}
        </label>
    }
}
