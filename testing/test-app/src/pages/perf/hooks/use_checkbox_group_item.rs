//! `use_checkbox_group_item`: a checkbox ("Item 3", its input `#perf-input`) in a
//! `use_checkbox_group` (`#perf-group`, labelled "Items") of three more, for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_checkbox_group_item.rs`): mounted at first, between two
//! plain buttons (`#perf-before`, `#perf-after`). Its controls ([`Control`]) mount and unmount
//! it (the group stays), disable and enable it, and read whether the group checks its value
//! (`checked`).

use std::collections::HashSet;

use leptonic::hooks::{
    collections::Key,
    form::{
        CheckboxGroupData, ToggleOptions, UseCheckboxGroupInput, UseCheckboxGroupItemInput,
        UseCheckboxGroupStateInput, use_checkbox_group, use_checkbox_group_item,
        use_checkbox_group_state,
    },
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let disabled = RwSignal::new(false);
    let state = use_checkbox_group_state(UseCheckboxGroupStateInput {
        default_value: HashSet::new(),
        name: Some("items".to_owned()),
        ..UseCheckboxGroupStateInput::default()
    });
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
        Control::read("checked", move || {
            f64::from(u8::from(
                state
                    .value
                    .with_untracked(|value| value.contains(&Key::from(3))),
            ))
        }),
    ];
    let group = use_checkbox_group(UseCheckboxGroupInput {
        state,
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
    let measured = data.clone();
    view! {
        <h1>"use_checkbox_group_item performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <div {..group.group.attrs()}>
            <span {..group.label.attrs()}>"Items"</span>
            {(0..3)
                .map(|item| {
                    view! {
                        <GroupCheckbox group=data.clone() item disabled=Signal::stored(false) />
                    }
                })
                .collect_view()}
            <Show when=move || mounted.get()>
                <GroupCheckbox group=measured.clone() item=3 disabled=disabled.into() />
            </Show>
        </div>
        <button id="perf-after">"After"</button>
    }
}

/// A checkbox of the group (the measured one: item 3, `#perf-input`).
#[component]
fn GroupCheckbox(group: CheckboxGroupData, item: i32, disabled: Signal<bool>) -> impl IntoView {
    let checkbox = use_checkbox_group_item(UseCheckboxGroupItemInput {
        group,
        value: Key::from(item),
        is_indeterminate: Signal::stored(false),
        on_change: None,
        options: ToggleOptions {
            id: (item == 3).then(|| "perf-input".into()),
            is_disabled: disabled,
            ..ToggleOptions::default()
        },
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
