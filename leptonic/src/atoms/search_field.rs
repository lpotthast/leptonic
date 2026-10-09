// Upstream: react-aria-components/src/SearchField.tsx @ 99e6102368
// Upstream: react-aria-components/test/SearchField.test.js @ 99e6102368
use leptos::prelude::*;
use leptos_classes::Classes;

use super::{form::use_validation_behavior, text_field::provide_text_field_contexts};
use crate::{
    Out,
    atoms::field::LabelPresence,
    hooks::{
        button::{UseButtonInput, use_button},
        form::{
            AutoCapitalize, EnterKeyHint, InputMode, InputType, TextFieldElement,
            UseSearchFieldInput, UseSearchFieldReturn, UseTextFieldInput, UseTextFieldStateInput,
            ValidateFn, ValidationBehavior, use_search_field, use_text_field_state,
        },
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
// - As [`TextField`](super::text_field::TextField): `value` + `set_value` (C4), the input's
//   attributes are props of the field.
// - The clear button is the `SearchFieldClearButton` part (react-aria-components: any `Button`,
//   configured through `ButtonContext`). Reason: Leptos contexts are typed; a dedicated part
//   needs no generic button context.
// - Render props become `data-*` attributes plus plain children.
//
// =============================================================================

/// Context from [`SearchField`] to its [`SearchFieldClearButton`].
#[derive(Clone)]
struct SearchFieldContext {
    clear_button: StoredValue<UseButtonInput>,
}

/// A headless search field: a text field where Enter submits the search (`on_submit`) and Escape
/// or the [`SearchFieldClearButton`] empty it. Compose it from a
/// [`Label`](super::field::Label), an [`Input`](super::input::Input), a
/// `SearchFieldClearButton`, a [`Description`](super::field::Description) and a
/// [`FieldError`](super::field::FieldError).
///
/// Data attributes: `data-empty`, `data-disabled`, `data-invalid`, `data-readonly`,
/// `data-required`.
///
/// Default class: `leptonic-SearchField`.
#[allow(clippy::too_many_arguments, clippy::needless_pass_by_value)]
#[component]
pub fn SearchField(
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
    /// Called with the value when Enter is pressed. Without it, Enter submits the form.
    #[prop(into, optional)]
    on_submit: Option<Callback<String>>,
    /// Called when Escape or the clear button empties the field.
    #[prop(into, optional)]
    on_clear: Option<Callback<()>>,
    #[prop(into, default = Signal::stored(InputType::Search))] input_type: Signal<InputType>,
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
    #[prop(into, optional)] pattern: Option<String>,
    #[prop(into, optional)] min_length: Option<u32>,
    #[prop(into, optional)] max_length: Option<u32>,
    /// The `autocomplete` hint, e.g. `"off"`.
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
    let classes = with_default_class("leptonic-SearchField", classes);
    let (value, on_change) = crate::ValueBinding::from_state_props(value, set_value, on_change);
    let state = use_text_field_state(UseTextFieldStateInput {
        default_value,
        value,
        on_change,
    });
    // As in react-aria-components: a visible label is expected unless an ARIA label is given.
    let label_presence = LabelPresence::new(aria_label, aria_labelledby.as_ref());
    let has_label = label_presence.has_label;
    let UseSearchFieldReturn {
        text_field,
        clear_button,
    } = use_search_field(UseSearchFieldInput {
        on_submit,
        on_clear,
        text_field: UseTextFieldInput {
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
        },
    });
    let is_invalid = text_field.is_invalid;
    let is_empty = Signal::derive(move || state.value.with(String::is_empty));

    // Contexts for the children only, with the `<div>` as the root (getting attributes set on the
    // component).
    scoped_view(
        move || {
            provide_text_field_contexts(text_field, label_presence);
            provide_context(SearchFieldContext {
                clear_button: StoredValue::new(clear_button),
            });
        },
        move || {
            view! {
                <div
                    class=classes
                    style=styles
                    data-empty=flag(is_empty)
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

/// The button emptying the [`SearchField`] around it. It is not in the tab order (Escape clears
/// from the keyboard) and keeps focus in the input. Hide it while the field is empty with the
/// field's `data-empty`.
///
/// Data attributes: `data-pressed`, `data-hovered`, `data-focused`, `data-focus-visible`,
/// `data-disabled`.
///
/// Default class: `leptonic-SearchFieldClearButton`.
#[component]
pub fn SearchFieldClearButton(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let classes = with_default_class("leptonic-SearchFieldClearButton", classes);
    let ctx = expect_context::<SearchFieldContext>();
    let input = ctx.clear_button.get_value();
    let button = use_button(input);
    let (attrs, button_styles) = button.props.into_parts();

    view! {
        <button
            {..attrs}
            class=classes
            style=button_styles.merge(styles)
            data-pressed=flag(button.is_pressed)
            data-hovered=flag(button.is_hovered)
            data-focused=flag(button.is_focused)
            data-disabled=flag(button.is_disabled)
        >
            {children()}
        </button>
    }
}
