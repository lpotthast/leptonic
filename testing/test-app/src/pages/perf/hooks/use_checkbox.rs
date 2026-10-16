//! `use_checkbox` on a `<label>` ("Subscribe") wrapping an `<input type="checkbox">`
//! (`#perf-input`), for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_checkbox.rs`): mounted at first, its state a
//! `use_toggle_state` of the page (not selected at first), between two plain buttons
//! (`#perf-before`, `#perf-after`). Its controls ([`Control`]) mount and unmount it, change its
//! reactive props (disabled, read-only, invalid, indeterminate), select it (`select`), and read
//! whether it is selected (`selected`) and pressed (`pressed`).

use leptonic::hooks::form::{
    ToggleOptions, ToggleState, UseCheckboxInput, UseToggleStateInput, use_checkbox,
    use_toggle_state,
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The props of the checkbox the page changes.
#[derive(Clone, Copy)]
struct Props {
    disabled: RwSignal<bool>,
    read_only: RwSignal<bool>,
    invalid: RwSignal<bool>,
    indeterminate: RwSignal<bool>,
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let props = Props {
        disabled: RwSignal::new(false),
        read_only: RwSignal::new(false),
        invalid: RwSignal::new(false),
        indeterminate: RwSignal::new(false),
    };
    let state = use_toggle_state(UseToggleStateInput {
        is_read_only: props.read_only.into(),
        ..UseToggleStateInput::default()
    });
    // Whether the mounted checkbox is pressed, for the controls.
    let pressed = StoredValue::new(None::<Signal<bool>>);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("disable", move || props.disabled.set(true)),
        Control::action("enable", move || props.disabled.set(false)),
        Control::action("set-read-only", move || props.read_only.set(true)),
        Control::action("unset-read-only", move || props.read_only.set(false)),
        Control::action("invalidate", move || props.invalid.set(true)),
        Control::action("uninvalidate", move || props.invalid.set(false)),
        Control::action("set-indeterminate", move || props.indeterminate.set(true)),
        Control::action("unset-indeterminate", move || {
            props.indeterminate.set(false);
        }),
        Control::action("select", move || state.set_selected(true)),
        Control::read("selected", move || {
            f64::from(u8::from(state.is_selected.get_untracked()))
        }),
        Control::read("pressed", move || {
            pressed
                .get_value()
                .map_or(0.0, |pressed| f64::from(u8::from(pressed.get_untracked())))
        }),
    ];
    view! {
        <h1>"use_checkbox performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <Checkbox state props pressed />
        </Show>
        <button id="perf-after">"After"</button>
    }
}

/// The checkbox in its label.
#[component]
fn Checkbox(
    state: ToggleState,
    props: Props,
    pressed: StoredValue<Option<Signal<bool>>>,
) -> impl IntoView {
    let checkbox = use_checkbox(UseCheckboxInput {
        state,
        is_indeterminate: props.indeterminate.into(),
        options: ToggleOptions {
            id: Some("perf-input".to_owned()),
            is_disabled: props.disabled.into(),
            is_read_only: props.read_only.into(),
            is_invalid: props.invalid.into(),
            name: Some("subscribe".to_owned()),
            ..ToggleOptions::default()
        },
    });
    pressed.set_value(Some(checkbox.is_pressed));
    on_cleanup(move || {
        pressed.try_update_value(|pressed| *pressed = None);
    });
    let (label_attrs, label_styles) = checkbox.label_props.into_parts();
    let (input_attrs, input_styles) = checkbox.input_props.into_parts();
    view! {
        <label {..label_attrs} style=label_styles>
            <input {..input_attrs} style=input_styles />
            "Subscribe"
        </label>
    }
}
