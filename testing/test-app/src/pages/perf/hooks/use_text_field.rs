//! `use_text_field` on a `<label>`, an `<input>` (`#perf-input`), a description and an error
//! message (in `#perf-error-slot`, rendered while invalid), for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_text_field.rs`): mounted at first, its value the page's
//! ("Ada" at first), between two plain buttons (`#perf-before`, `#perf-after`). Its controls
//! ([`Control`]) mount and unmount it, change its reactive props, set its value (`set-value`:
//! "Grace", `set-long-value`: 10,000 characters), and read whether it is focused (`focused`) and
//! shows a focus ring (`focus-visible`).

use leptonic::{
    IntoAttrs, ValueBinding,
    hooks::form::{
        InputType, TextFieldElement, UseTextFieldInput, UseTextFieldStateInput, ValidationBehavior,
        use_text_field, use_text_field_state,
    },
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The length of the long value (`set-long-value`).
const LONG_VALUE_LENGTH: usize = 10_000;

/// The props of the text field the page changes.
#[derive(Clone, Copy)]
struct Props {
    value: RwSignal<String>,
    disabled: RwSignal<bool>,
    read_only: RwSignal<bool>,
    required: RwSignal<bool>,
    invalid: RwSignal<bool>,
}

/// What the page's controls read from the mounted field.
#[derive(Clone, Copy)]
struct Focus {
    is_focused: Signal<bool>,
    is_focus_visible: Signal<bool>,
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let props = Props {
        value: RwSignal::new("Ada".to_owned()),
        disabled: RwSignal::new(false),
        read_only: RwSignal::new(false),
        required: RwSignal::new(false),
        invalid: RwSignal::new(false),
    };
    let focus = StoredValue::new(None::<Focus>);
    let read = move |read: fn(&Focus) -> bool| {
        move || {
            focus
                .get_value()
                .map_or(0.0, |focus| f64::from(u8::from(read(&focus))))
        }
    };
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("disable", move || props.disabled.set(true)),
        Control::action("enable", move || props.disabled.set(false)),
        Control::action("set-read-only", move || props.read_only.set(true)),
        Control::action("unset-read-only", move || props.read_only.set(false)),
        Control::action("require", move || props.required.set(true)),
        Control::action("unrequire", move || props.required.set(false)),
        Control::action("invalidate", move || props.invalid.set(true)),
        Control::action("uninvalidate", move || props.invalid.set(false)),
        Control::action("set-value", move || props.value.set("Grace".to_owned())),
        Control::action("set-long-value", move || {
            props.value.set("a".repeat(LONG_VALUE_LENGTH));
        }),
        Control::read("focused", read(|focus| focus.is_focused.get_untracked())),
        Control::read(
            "focus-visible",
            read(|focus| focus.is_focus_visible.get_untracked()),
        ),
    ];
    view! {
        <h1>"use_text_field performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <Field props focus />
        </Show>
        <button id="perf-after">"After"</button>
    }
}

/// The text field.
#[component]
fn Field(props: Props, focus: StoredValue<Option<Focus>>) -> impl IntoView {
    let state = use_text_field_state(UseTextFieldStateInput {
        default_value: String::new(),
        value: Some(ValueBinding::from(props.value)),
        on_change: None,
    });
    let field = use_text_field(UseTextFieldInput {
        state,
        id: Some("perf-input".to_owned()),
        element: Signal::stored(TextFieldElement::Input),
        input_type: Signal::stored(InputType::Text),
        is_disabled: props.disabled.into(),
        is_read_only: props.read_only.into(),
        is_required: props.required.into(),
        is_invalid: props.invalid.into(),
        validate: None,
        validation_behavior: ValidationBehavior::Aria,
        validation: None,
        name: Some("name".to_owned()),
        form: None,
        placeholder: MaybeProp::default(),
        pattern: None,
        min_length: None,
        max_length: None,
        auto_complete: None,
        auto_capitalize: None,
        auto_correct: None,
        spell_check: None,
        input_mode: Signal::stored(None),
        enter_key_hint: None,
        auto_focus: false,
        exclude_from_tab_order: false,
        label_id: None,
        has_label: true.into(),
        aria_label: MaybeProp::default(),
        aria_labelledby: Signal::default(),
        aria_describedby: Signal::default(),
        aria_errormessage: Signal::default(),
        aria_activedescendant: Signal::stored(None),
        aria_autocomplete: None,
        aria_haspopup: None,
        aria_controls: Signal::stored(None),
        on_focus: None,
        on_blur: None,
        on_focus_change: None,
        on_key_down: None,
        on_key_up: None,
        shortcuts: None,
    });
    focus.set_value(Some(Focus {
        is_focused: field.is_focused,
        is_focus_visible: field.is_focus_visible,
    }));
    on_cleanup(move || {
        focus.try_update_value(|focus| *focus = None);
    });
    let is_invalid = field.is_invalid;
    let error_message_props = field.error_message_props;
    view! {
        <label {..field.label_props.into_attrs()}>"Name"</label>
        <input {..field.input_props.into_attrs()} />
        <p {..field.description_props.into_attrs()}>"Shown to others."</p>
        <div id="perf-error-slot">
            <Show when=move || is_invalid.get()>
                <p {..error_message_props.clone().into_attrs()}>"Invalid name."</p>
            </Show>
        </div>
    }
}
