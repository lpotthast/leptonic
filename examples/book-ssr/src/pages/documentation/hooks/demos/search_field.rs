use leptonic::{
    IntoAttrs,
    atoms::checkbox::{CheckboxButton, CheckboxField},
    hooks::{
        button::use_button,
        form::{
            InputType, TextFieldElement, UseSearchFieldInput, UseSearchFieldReturn,
            UseTextFieldInput, UseTextFieldStateInput, ValidationBehavior, use_search_field,
            use_text_field_state,
        },
    },
};
use leptos::prelude::*;

#[component]
pub fn SearchFieldDemo() -> impl IntoView {
    let query = use_text_field_state(UseTextFieldStateInput::default());
    let (submitted, set_submitted) = signal(None::<String>);
    let disabled = RwSignal::new(false);

    let UseSearchFieldReturn {
        text_field,
        clear_button,
    } = use_search_field(UseSearchFieldInput {
        on_submit: Some(Callback::new(move |value| set_submitted.set(Some(value)))),
        text_field: UseTextFieldInput {
            input_type: Signal::stored(InputType::Search),
            ..UseTextFieldInput {
                has_label: true.into(),
                is_disabled: disabled.into(),
                placeholder: "Search\u{2026}".into(),
                state: query,
                id: None,
                element: TextFieldElement::Input,
                input_type: Signal::stored(InputType::Text),
                is_read_only: Signal::stored(false),
                is_required: Signal::stored(false),
                is_invalid: Signal::stored(false),
                validate: None,
                validation_behavior: ValidationBehavior::default(),
                validation: None,
                name: None,
                form: None,
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
                aria_label: MaybeProp::default(),
                aria_labelledby: None,
                aria_describedby: None,
                aria_errormessage: None,
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
            }
        },
        on_clear: None,
    });
    // The clear button's configuration goes to `use_button`. It is labelled "Clear search".
    let (clear_attrs, clear_styles) = use_button(clear_button).props.into_parts();
    let is_empty = move || query.value.with(String::is_empty);

    view! {
        <div class="demo-field">
            <label class="demo-field-label" {..text_field.label_props.into_attrs()}>"Search"</label>
            <div class="demo-input-row">
                <input class="demo-input demo-text-input" {..text_field.input_props.into_attrs()}/>
                // Nothing to clear while the field is empty.
                <button {..clear_attrs} style=clear_styles class="demo-btn" hidden=is_empty>
                    <span aria-hidden="true">"\u{2715}"</span>
                </button>
            </div>
        </div>

        <p class="demo-status">
            {move || match submitted.get() {
                Some(query) => format!("Submitted: \u{201c}{query}\u{201d}"),
                None => "Nothing submitted yet.".to_owned(),
            }}
        </p>

        <div class="demo-controls">
            <CheckboxField is_selected=disabled set_selected=disabled>
                <CheckboxButton classes="demo-check">
                    <span class="demo-check-box" aria-hidden="true"></span>
                    "Disabled"
                </CheckboxButton>
            </CheckboxField>
        </div>
    }
}
