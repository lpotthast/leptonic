//! `use_number_field` (of `i32`, 0 to 100) on a group, a `<label>`, an `<input>`
//! (`#perf-input`) and its stepper buttons ("-" and "+", `use_button`),
//! for its performance test (`leptonic/tests/perf_tests/hooks/use_number_field.rs`): mounted at
//! first, its value 5 at first, between two plain buttons (`#perf-before`, `#perf-after`). Its
//! controls ([`Control`]) mount and unmount it, disable and enable it, and read its value
//! (`value`, -1: empty).

use leptonic::{
    IntoAttrs,
    hooks::{
        button::use_button,
        form::{
            NumberFieldState, UseNumberFieldInput, UseNumberFieldStateInput, use_number_field,
            use_number_field_state,
        },
    },
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let disabled = RwSignal::new(false);
    // The mounted field's state, for the controls.
    let state = StoredValue::new(None::<NumberFieldState<i32>>);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
        Control::read("value", move || {
            state
                .get_value()
                .and_then(|state| state.value().get_untracked())
                .map_or(-1.0, f64::from)
        }),
    ];
    view! {
        <h1>"use_number_field performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <Field disabled state />
        </Show>
        <button id="perf-after">"After"</button>
    }
}

/// The number field.
#[component]
fn Field(
    disabled: RwSignal<bool>,
    state: StoredValue<Option<NumberFieldState<i32>>>,
) -> impl IntoView {
    let number = use_number_field_state(UseNumberFieldStateInput {
        default_value: Some(5_i32),
        min_value: Signal::stored(Some(0)),
        max_value: Signal::stored(Some(100)),
        is_disabled: disabled.into(),
        ..UseNumberFieldStateInput::default()
    });
    state.set_value(Some(number));
    on_cleanup(move || {
        state.try_update_value(|state| *state = None);
    });
    let field = use_number_field(UseNumberFieldInput {
        state: number,
        id: Some("perf-input".to_owned()),
        has_label: true.into(),
        aria_label: MaybeProp::default(),
        aria_labelledby: Signal::default(),
        aria_describedby: Signal::default(),
        is_required: Signal::stored(false),
        placeholder: MaybeProp::default(),
        auto_focus: false,
        is_wheel_disabled: false,
        increment_aria_label: MaybeProp::default(),
        decrement_aria_label: MaybeProp::default(),
        on_focus: None,
        on_blur: None,
        on_focus_change: None,
        on_key_down: None,
        on_key_up: None,
    });
    let (decrement_attrs, decrement_styles) = use_button(field.decrement_button).props.into_parts();
    let (increment_attrs, increment_styles) = use_button(field.increment_button).props.into_parts();
    view! {
        <div {..field.group_props.into_attrs()}>
            <label {..field.label_props.into_attrs()}>"Quantity"</label>
            <button {..decrement_attrs} style=decrement_styles>"-"</button>
            <input {..field.input_props.into_attrs()} />
            <button {..increment_attrs} style=increment_styles>"+"</button>
        </div>
    }
}
