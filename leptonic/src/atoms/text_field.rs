// Upstream: react-aria-components/src/TextField.tsx @ 99e6102368
// Upstream: react-aria-components/test/TextField.test.js @ 99e6102368
use leptos::prelude::*;
use leptos_classes::Classes;

use super::{
    field::{FieldContext, LabelContext},
    form::use_validation_behavior,
    input::{InputContext, InputState},
};
use crate::{
    Out,
    atoms::field::LabelPresence,
    hooks::form::{
        AutoCapitalize, EnterKeyHint, InputMode, InputType, TextFieldElement, UseTextFieldInput,
        UseTextFieldReturn, UseTextFieldStateInput, ValidateFn, ValidationBehavior, use_text_field,
        use_text_field_state,
    },
    utils::{
        data_attributes::flag, default_class::with_default_class, scoped_context::scoped_view,
        styles::Styles,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Value (C4): `default_value` + `on_change`, or `value` + `set_value` (react-aria: `value` +
//   `onChange`).
// - The input's attributes (`placeholder`, `pattern`, `input_type`, ...) are props of the field,
//   not of its `Input`. Reason: the field's hook computes all of the input's attributes; the
//   `Input` part only renders them.
// - Render props become `data-*` attributes plus plain children.
//
// =============================================================================

/// A headless text field. Compose it from a [`Label`](super::field::Label), an
/// [`Input`](super::input::Input) or [`TextArea`](super::input::TextArea), a
/// [`Description`](super::field::Description) and a [`FieldError`](super::field::FieldError).
///
/// Data attributes: `data-disabled`, `data-invalid`, `data-readonly`, `data-required`.
///
/// Default class: `leptonic-TextField`.
#[allow(clippy::too_many_arguments, clippy::needless_pass_by_value)]
#[component]
pub fn TextField(
    #[prop(into, optional)] default_value: String,
    /// Called when the value changes.
    #[prop(into, optional)]
    on_change: Option<Callback<String>>,
    /// The value (controlled): a value or any signal.
    #[prop(into, optional)]
    value: Option<Signal<String>>,
    /// Receives the new value: an `RwSignal`, `WriteSignal`, closure, `Callback`, ...
    #[prop(into, optional)]
    set_value: Option<Out<String>>,
    /// The `<input>`'s type (ignored by a `TextArea`).
    #[prop(into, optional)]
    input_type: Signal<InputType>,
    #[prop(into, optional)] is_disabled: Signal<bool>,
    #[prop(into, optional)] is_read_only: Signal<bool>,
    #[prop(into, optional)] is_required: Signal<bool>,
    #[prop(into, optional)] is_invalid: Signal<bool>,
    #[prop(optional)] validate: Option<ValidateFn<String>>,
    /// Default: the surrounding [`Form`](super::form::Form)'s, else `Native`.
    #[prop(optional)]
    validation_behavior: Option<ValidationBehavior>,
    #[prop(into, optional)] name: Option<String>,
    #[prop(into, optional)] form: Option<String>,
    #[prop(into, optional)] placeholder: MaybeProp<String>,
    /// A validation pattern (ignored by a `TextArea`).
    #[prop(into, optional)]
    pattern: Option<String>,
    #[prop(into, optional)] min_length: Option<u32>,
    #[prop(into, optional)] max_length: Option<u32>,
    /// The `autocomplete` hint, e.g. `"email"` or `"off"`.
    #[prop(into, optional)]
    auto_complete: Option<String>,
    #[prop(into, optional)] auto_capitalize: Option<AutoCapitalize>,
    #[prop(into, optional)] auto_correct: Option<bool>,
    #[prop(into, optional)] spell_check: Option<bool>,
    #[prop(into, optional)] input_mode: Option<InputMode>,
    #[prop(into, optional)] enter_key_hint: Option<EnterKeyHint>,
    #[prop(optional)] auto_focus: bool,
    /// The input's id.
    #[prop(into, optional)]
    id: Option<String>,
    /// Labels the field when there is no [`Label`](super::field::Label).
    #[prop(into, optional)]
    aria_label: MaybeProp<String>,
    #[prop(into, optional)] aria_labelledby: Option<String>,
    #[prop(into, optional)] aria_describedby: Option<String>,
    #[prop(into, optional)] on_focus_change: Option<Callback<bool>>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let classes = with_default_class("leptonic-TextField", classes);
    let (value, on_change) = crate::ValueBinding::from_state_props(value, set_value, on_change);
    let state = use_text_field_state(UseTextFieldStateInput {
        default_value,
        value,
        on_change,
    });
    // As in react-aria-components: a visible label is expected unless an ARIA label is given.
    let label_presence = LabelPresence::new(aria_label, aria_labelledby.as_ref());
    let has_label = label_presence.has_label;
    let field = use_text_field(UseTextFieldInput {
        id,
        input_type,
        is_disabled,
        is_read_only,
        is_required,
        is_invalid,
        validate,
        validation_behavior: use_validation_behavior(validation_behavior),
        name,
        form,
        placeholder,
        pattern,
        min_length,
        max_length,
        auto_complete,
        auto_capitalize,
        auto_correct,
        spell_check,
        input_mode: Signal::stored(input_mode),
        enter_key_hint,
        auto_focus,
        has_label,
        aria_label,
        aria_labelledby,
        aria_describedby,
        on_focus_change,
        state,
        element: TextFieldElement::Input,
        validation: None,
        exclude_from_tab_order: false,
        label_id: None,
        aria_errormessage: None,
        aria_activedescendant: Signal::stored(None),
        aria_autocomplete: None,
        aria_haspopup: None,
        aria_controls: Signal::stored(None),
        on_focus: None,
        on_blur: None,
        on_key_down: None,
        on_key_up: None,
        shortcuts: None,
    });
    let is_invalid = field.is_invalid;

    // Contexts for the children only, with the `<div>` as the root (getting attributes set on the
    // component).
    scoped_view(
        move || provide_text_field_contexts(field, label_presence),
        move || {
            view! {
                <div
                    class=classes
                    style=styles
                    data-disabled=flag(is_disabled)
                    data-invalid=flag(is_invalid)
                    data-readonly=flag(is_read_only)
                    data-required=flag(is_required)
                >
                    {children()}
                </div>
            }
        },
    )
}

/// Provides the contexts of a text-like field's parts ([`Label`](super::field::Label),
/// [`Input`](super::input::Input), ...).
pub(crate) fn provide_text_field_contexts(
    field: UseTextFieldReturn,
    label_presence: LabelPresence,
) {
    let UseTextFieldReturn {
        label_props,
        input_props,
        description_props,
        error_message_props,
        is_focused,
        is_focus_visible,
        is_invalid,
        validation_errors,
        validation_details,
        ..
    } = field;
    let is_disabled = input_props.disabled;
    provide_context(InputContext::text_field(
        input_props,
        InputState {
            is_disabled,
            is_invalid,
            is_focused,
            is_focus_visible,
        },
    ));
    provide_context(LabelContext::label(label_props).with_presence(label_presence));
    provide_context(FieldContext {
        description: description_props,
        error_message: error_message_props,
        is_invalid,
        validation_errors,
        validation_details,
    });
}
