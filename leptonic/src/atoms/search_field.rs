// Upstream: react-aria-components/src/SearchField.tsx @ 99e6102368
use leptos::prelude::*;

use super::{form::use_validation_behavior, text_field::provide_text_field_contexts};
use crate::{
    hooks::{
        AutoCapitalize, EnterKeyHint, InputMode, InputType, IntoAttrs, TextFieldState,
        UseButtonInput, UseSearchFieldInput, UseSearchFieldReturn, UseTextFieldInput,
        UseTextFieldStateInput, ValidateFn, ValidationBehavior, use_button, use_search_field,
        use_text_field_state,
    },
    utils::data_attributes::flag,
    utils::scoped_context::scoped_view,
    utils::{classes::Classes, styles::Styles},
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - As [`TextField`](super::text_field::TextField): no controlled `value`, the input's attributes
//   are props of the field.
// - The clear button is the `SearchFieldClearButton` part (react-aria-components: any `Button`,
//   configured through `ButtonContext`). Reason: Leptos contexts are typed; a dedicated part
//   needs no generic button context.
// - Render props become `data-*` attributes plus plain children.
//
// =============================================================================

/// Context from [`SearchField`] to its [`SearchFieldClearButton`].
#[derive(Clone)]
struct SearchFieldCtx {
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
#[allow(clippy::too_many_arguments, clippy::needless_pass_by_value)]
#[component]
pub fn SearchField(
    #[prop(into, optional)] default_value: String,
    /// Called when the value changes.
    #[prop(into, optional)]
    on_change: Option<Callback<String>>,
    /// External value state, replacing `default_value`. Bind a signal with
    /// `state=TextFieldState::from(rw_signal)`.
    #[prop(into, optional)]
    state: Option<TextFieldState>,
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
    let state = if let Some(state) = state {
        on_change.map_or(state, |on_change| state.with_on_change(on_change))
    } else {
        use_text_field_state(UseTextFieldStateInput {
            default_value,
            on_change,
        })
    };
    // As in react-aria-components: a visible label is expected unless an ARIA label is given.
    let has_label = aria_label.get_untracked().is_none() && aria_labelledby.is_none();
    let UseSearchFieldReturn {
        text_field,
        clear_button,
    } = use_search_field(UseSearchFieldInput {
        on_submit,
        on_clear,
        ..UseSearchFieldInput::new(UseTextFieldInput {
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
            input_mode,
            enter_key_hint,
            auto_focus,
            has_label,
            aria_label,
            aria_labelledby,
            aria_describedby,
            on_focus_change,
            ..UseTextFieldInput::new(state)
        })
    });
    let is_invalid = text_field.is_invalid;
    let is_empty = Signal::derive(move || state.value.with(String::is_empty));

    // Contexts for the children only, with the `<div>` as the root (getting attributes set on the
    // component).
    scoped_view(
        move || {
            provide_text_field_contexts(text_field);
            provide_context(SearchFieldCtx {
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
/// Data attributes: `data-pressed`, `data-hovered`, `data-disabled`.
#[component]
pub fn SearchFieldClearButton(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let ctx = expect_context::<SearchFieldCtx>();
    let input = ctx.clear_button.get_value();
    let is_disabled = input.is_disabled;
    let button = use_button(input);
    let hover = crate::hooks::use_hover(crate::hooks::UseHoverInput {
        is_disabled,
        ..crate::hooks::UseHoverInput::default()
    });
    let (attrs, button_styles) = button.props.into_parts();

    view! {
        <button
            {..attrs}
            {..hover.props.into_attrs()}
            class=classes
            style=button_styles.merge(styles)
            data-pressed=flag(button.is_pressed)
            data-hovered=flag(hover.is_hovered)
            data-disabled=flag(is_disabled)
        >
            {children()}
        </button>
    }
}
