//! `use_radio`: a radio ("Option 3", its input `#perf-input`) in a `use_radio_group`
//! (`#perf-group`, labelled "Options", vertical) of three more ("Option 0" selected at first),
//! for its performance test (`leptonic/tests/perf_tests/hooks/use_radio.rs`): mounted at first,
//! between two plain buttons (`#perf-before`, `#perf-after`). Its controls ([`Control`]) mount
//! and unmount it (the group stays), disable and enable it, and read the selected option
//! (`selected`, -1: none) and whether it is pressed (`pressed`).

use leptonic::{
    Orientation,
    hooks::{
        collections::Key,
        form::{
            RadioGroupData, UseRadioGroupInput, UseRadioGroupStateInput, UseRadioInput, use_radio,
            use_radio_group, use_radio_group_state,
        },
    },
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let disabled = RwSignal::new(false);
    let state = use_radio_group_state(UseRadioGroupStateInput {
        default_value: Some(Key::from(0)),
        name: Some("options".to_owned()),
        ..UseRadioGroupStateInput::default()
    });
    // Whether the mounted radio is pressed, for the controls.
    let pressed = StoredValue::new(None::<Signal<bool>>);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
        Control::read("selected", move || {
            state
                .selected_value
                .with_untracked(|selected| selected.as_ref().map(ToString::to_string))
                .and_then(|selected| selected.parse::<i32>().ok())
                .map_or(-1.0, f64::from)
        }),
        Control::read("pressed", move || {
            pressed
                .get_value()
                .map_or(0.0, |pressed| f64::from(u8::from(pressed.get_untracked())))
        }),
    ];
    let group = use_radio_group(UseRadioGroupInput {
        state,
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
        <h1>"use_radio performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <div {..group.group.attrs()}>
            <span {..group.label.attrs()}>"Options"</span>
            {(0..3)
                .map(|option| {
                    view! {
                        <GroupRadio group=data option disabled=Signal::stored(false) pressed=None />
                    }
                })
                .collect_view()}
            <Show when=move || mounted.get()>
                <GroupRadio group=data option=3 disabled=disabled.into() pressed=Some(pressed) />
            </Show>
        </div>
        <button id="perf-after">"After"</button>
    }
}

/// A radio of the group (the measured one: option 3, `#perf-input`, reporting whether it is
/// pressed to `pressed`).
#[component]
fn GroupRadio(
    group: RadioGroupData,
    option: i32,
    disabled: Signal<bool>,
    pressed: Option<StoredValue<Option<Signal<bool>>>>,
) -> impl IntoView {
    let radio = use_radio(UseRadioInput {
        group,
        value: Key::from(option),
        is_disabled: disabled,
        id: (option == 3).then(|| "perf-input".into()),
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
    if let Some(pressed) = pressed {
        pressed.set_value(Some(radio.is_pressed));
        on_cleanup(move || {
            pressed.try_update_value(|pressed| *pressed = None);
        });
    }
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
