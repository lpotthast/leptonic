//! `use_radio_group` on a group (`#perf-group`, labelled "Options", vertical) of radios
//! (`use_radio`, labelled "Option 0", "Option 1", ...; "Option 0" selected at first), for its
//! performance test (`leptonic/tests/perf_tests/hooks/use_radio_group.rs`): mounted at first
//! with 10 radios, between two plain buttons (`#perf-before`, `#perf-after`). Its controls
//! ([`Control`]) mount it with 10 (`mount`) or 1,000 radios (`mount-1000`), unmount it, disable
//! and enable it, and read the selected option (`selected`, -1: none).

use leptonic::{
    Orientation,
    hooks::{
        collections::Key,
        form::{
            RadioGroupData, RadioGroupState, UseRadioGroupInput, UseRadioGroupStateInput,
            UseRadioInput, use_radio, use_radio_group, use_radio_group_state,
        },
    },
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let size = RwSignal::new(Some(10_i32));
    let disabled = RwSignal::new(false);
    // The mounted group's state, for the controls.
    let state = StoredValue::new(None::<RadioGroupState>);
    let controls = vec![
        Control::action("mount", move || size.set(Some(10))),
        Control::action("mount-1000", move || size.set(Some(1_000))),
        Control::action("unmount", move || size.set(None)),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
        Control::read("selected", move || {
            state
                .get_value()
                .and_then(|state| {
                    state
                        .selected_value
                        .with_untracked(|selected| selected.as_ref().map(ToString::to_string))
                })
                .and_then(|selected| selected.parse::<i32>().ok())
                .map_or(-1.0, f64::from)
        }),
    ];
    view! {
        <h1>"use_radio_group performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        {move || size.get().map(|size| view! { <Group size disabled state /> })}
        <button id="perf-after">"After"</button>
    }
}

/// The group with `size` radios.
#[component]
fn Group(
    size: i32,
    disabled: RwSignal<bool>,
    state: StoredValue<Option<RadioGroupState>>,
) -> impl IntoView {
    let group_state = use_radio_group_state(UseRadioGroupStateInput {
        default_value: Some(Key::from(0)),
        name: Some("options".to_owned()),
        is_disabled: disabled.into(),
        ..UseRadioGroupStateInput::default()
    });
    state.set_value(Some(group_state));
    on_cleanup(move || {
        state.try_update_value(|state| *state = None);
    });
    let group = use_radio_group(UseRadioGroupInput {
        state: group_state,
        id: Some("perf-group".into()),
        has_label: true.into(),
        aria_label: MaybeProp::default(),
        aria_labelledby: Signal::default(),
        aria_describedby: Signal::default(),
        aria_errormessage: Signal::default(),
        orientation: Orientation::Vertical.into(),
        form: None,
        on_focus: None,
        on_blur: None,
        on_focus_change: None,
    });
    let data = group.data;
    view! {
        <div {..group.group.attrs()}>
            <span {..group.label.attrs()}>"Options"</span>
            {(0..size).map(|option| view! { <GroupRadio group=data option /> }).collect_view()}
        </div>
    }
}

/// A radio of the group.
#[component]
fn GroupRadio(group: RadioGroupData, option: i32) -> impl IntoView {
    let radio = use_radio(UseRadioInput {
        group,
        value: Key::from(option),
        is_disabled: Signal::stored(false),
        id: None,
        aria_label: MaybeProp::default(),
        aria_labelledby: Signal::default(),
        aria_describedby: Signal::default(),
        auto_focus: false,
        on_focus: None,
        on_blur: None,
        on_focus_change: None,
        on_press_start: None,
        on_press_end: None,
        on_press_up: None,
        on_press: None,
        on_press_change: None,
    });
    let (label_attrs, label_styles) = radio
        .label
        .props()
        .expect("the first label takes its props")
        .into_parts();
    let (input_attrs, input_styles) = radio
        .input
        .props()
        .expect("the first input takes its props")
        .into_parts();
    view! {
        <label {..label_attrs} style=label_styles>
            <input {..input_attrs} style=input_styles />
            {format!("Option {option}")}
        </label>
    }
}
