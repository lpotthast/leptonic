//! `use_search_field` on a `<label>`, an `<input>` (`#perf-input`, value "leptos" at first) and
//! its clear button (`#perf-clear`, `use_button`), for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_search_field.rs`): mounted at first, between two plain
//! buttons (`#perf-before`, `#perf-after`). Its controls ([`Control`]) mount and unmount it, make
//! it read-only, and read how often it submitted (`submits`) and cleared (`clears`).

use leptonic::hooks::{
    button::{UseButtonInput, use_button},
    form::{
        InputType, TextFieldElement, UseSearchFieldInput, UseTextFieldInput,
        UseTextFieldStateInput, ValidationBehavior, use_search_field, use_text_field_state,
    },
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let read_only = RwSignal::new(false);
    let submits = StoredValue::new(0_u32);
    let clears = StoredValue::new(0_u32);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("set-read-only", move || read_only.set(true)),
        Control::read("submits", move || f64::from(submits.get_value())),
        Control::read("clears", move || f64::from(clears.get_value())),
    ];
    view! {
        <h1>"use_search_field performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <Search read_only submits clears />
        </Show>
        <button id="perf-after">"After"</button>
    }
}

/// The search field and its clear button.
#[component]
fn Search(
    read_only: RwSignal<bool>,
    submits: StoredValue<u32>,
    clears: StoredValue<u32>,
) -> impl IntoView {
    let state = use_text_field_state(UseTextFieldStateInput {
        default_value: "leptos".to_owned(),
        value: None,
        on_change: None,
    });
    let search = use_search_field(UseSearchFieldInput {
        on_submit: Some(Callback::new(move |_: String| {
            submits.update_value(|submits| *submits += 1);
        })),
        on_clear: Some(Callback::new(move |()| {
            clears.update_value(|clears| *clears += 1);
        })),
        text_field: UseTextFieldInput {
            state,
            id: Some("perf-input".into()),
            element: Signal::stored(TextFieldElement::Input),
            input_type: Signal::stored(InputType::Search),
            is_disabled: Signal::stored(false),
            is_read_only: read_only.into(),
            is_required: Signal::stored(false),
            is_invalid: Signal::stored(false),
            validate: None,
            validation_behavior: ValidationBehavior::Aria,
            validation: None,
            name: None,
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
        },
    });
    let (clear_attrs, clear_styles) = use_button(UseButtonInput {
        id: Some("perf-clear".into()),
        ..search.clear_button
    })
    .button
    .attrs();
    view! {
        <label {..search.text_field.label.attrs()}>"Search"</label>
        <input {..search.text_field.input.attrs()} />
        <button {..clear_attrs} style=clear_styles>
            "Clear"
        </button>
    }
}
