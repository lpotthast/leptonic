// Upstream: react-aria-components/src/Checkbox.tsx @ 99e6102368
use leptos::{context::Provider, prelude::*};

use super::{
    field::{FieldContext, LabelContext},
    form::use_validation_behavior,
};
use crate::{
    Out,
    atoms::field::LabelPresence,
    hooks::{
        CheckboxGroupData, IntoAttrs, ToggleOptions, UseCheckboxGroupInput,
        UseCheckboxGroupItemInput, UseCheckboxGroupReturn, UseCheckboxGroupStateInput,
        UseCheckboxInput, UseCheckboxReturn, UseHoverInput, UseToggleStateInput, ValidateFn,
        ValidationBehavior, collections::Key, use_checkbox, use_checkbox_group,
        use_checkbox_group_item, use_checkbox_group_state, use_hover, use_toggle_state,
    },
    utils::{
        ValueBinding, classes::Classes, data_attributes::flag, styles::Styles,
        visually_hidden::visually_hidden_styles,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - State (C4): `default_selected` + `on_change`, or `is_selected` + `set_selected`; the group's
//   `default_value` + `on_change`, or `value` + `set_value` (react-aria: `isSelected`/`value`
//   + `onChange`).
// - Render props become `data-*` attributes plus plain children.
//
// =============================================================================

/// Context from [`CheckboxGroup`] to its checkboxes.
#[derive(Clone)]
pub struct CheckboxGroupCtx {
    pub data: CheckboxGroupData,
}

/// A headless checkbox: a `<label>` around a visually hidden `<input type="checkbox">` and the
/// children (draw the box with them, styled through the label's data attributes).
///
/// Data attributes: `data-selected`, `data-indeterminate`, `data-pressed`, `data-hovered`,
/// `data-focused`, `data-focus-visible`, `data-disabled`, `data-readonly`, `data-invalid`,
/// `data-required`.
///
/// Inside a [`CheckboxGroup`], `value` is required and the group holds the selection
/// (`default_selected`, `is_selected` and `set_selected` don't apply).
#[allow(clippy::too_many_arguments, clippy::needless_pass_by_value)]
#[component]
pub fn Checkbox(
    /// The checkbox's value in its [`CheckboxGroup`].
    #[prop(into, optional)]
    value: Option<Key>,
    #[prop(optional)] default_selected: bool,
    /// Called when the checkbox is checked or unchecked.
    #[prop(into, optional)]
    on_change: Option<Callback<bool>>,
    /// Whether the toggle is selected (controlled): a value or any signal.
    #[prop(into, optional)]
    is_selected: Option<Signal<bool>>,
    /// Receives the selection: an `RwSignal`, `WriteSignal`, closure, `Callback`, ...
    #[prop(into, optional)]
    set_selected: Option<Out<bool>>,
    /// Shows the checkbox as partially checked, regardless of its selection.
    #[prop(into, optional)]
    is_indeterminate: Signal<bool>,
    #[prop(into, optional)] is_disabled: Signal<bool>,
    #[prop(into, optional)] is_read_only: Signal<bool>,
    #[prop(into, optional)] is_required: Signal<bool>,
    #[prop(into, optional)] is_invalid: Signal<bool>,
    #[prop(optional)] validate: Option<ValidateFn<bool>>,
    /// Default: the surrounding [`Form`](super::form::Form)'s, else `Native`.
    #[prop(optional)]
    validation_behavior: Option<ValidationBehavior>,
    /// The input's `name` (in a group: the group's).
    #[prop(into, optional)]
    name: Option<String>,
    /// The input's `value` (submitted while checked; in a group: `value`).
    #[prop(into, optional)]
    form_value: Option<String>,
    #[prop(into, optional)] form: Option<String>,
    /// The input's id.
    #[prop(into, optional)]
    id: Option<String>,
    #[prop(into, optional)] aria_label: MaybeProp<String>,
    #[prop(into, optional)] aria_labelledby: Option<String>,
    #[prop(into, optional)] aria_describedby: Option<String>,
    #[prop(optional)] auto_focus: bool,
    #[prop(into, optional)] on_focus_change: Option<Callback<bool>>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let validation_behavior = use_validation_behavior(validation_behavior);
    let options = ToggleOptions {
        id,
        is_disabled,
        is_read_only,
        is_required,
        is_invalid,
        validate: None,
        validation_behavior,
        name,
        form,
        value: form_value,
        aria_label,
        aria_labelledby,
        aria_describedby,
        auto_focus,
        on_focus_change,
        ..ToggleOptions::default()
    };
    let checkbox: UseCheckboxReturn = if let Some(group) = use_context::<CheckboxGroupCtx>() {
        let value = value.expect("a <Checkbox> in a <CheckboxGroup> needs a `value`");
        use_checkbox_group_item(UseCheckboxGroupItemInput {
            is_indeterminate,
            on_change,
            validate,
            options,
            ..UseCheckboxGroupItemInput::new(group.data, value)
        })
    } else {
        let (value, on_change) =
            ValueBinding::from_state_props(is_selected, set_selected, on_change);
        let state = use_toggle_state(UseToggleStateInput {
            default_selected,
            value,
            on_change,
            is_read_only,
        });
        use_checkbox(UseCheckboxInput {
            is_indeterminate,
            options: ToggleOptions {
                validate,
                ..options
            },
            ..UseCheckboxInput::new(state)
        })
    };
    let hover = use_hover(UseHoverInput {
        is_disabled: checkbox.is_disabled,
        ..UseHoverInput::default()
    });

    let (label_attrs, label_styles) = checkbox.label_props.into_parts();
    let (input_attrs, input_styles) = checkbox.input_props.into_parts();

    view! {
        <label
            {..label_attrs}
            {..hover.props.into_attrs()}
            class=classes
            style=label_styles.merge(styles)
            data-selected=flag(checkbox.is_selected)
            data-indeterminate=flag(is_indeterminate)
            data-pressed=flag(checkbox.is_pressed)
            data-hovered=flag(hover.is_hovered)
            data-focused=flag(checkbox.is_focused)
            data-focus-visible=flag(checkbox.is_focus_visible)
            data-disabled=flag(checkbox.is_disabled)
            data-readonly=flag(checkbox.is_read_only)
            data-invalid=flag(checkbox.is_invalid)
            data-required=flag(is_required)
        >
            <input {..input_attrs} style=input_styles.merge(visually_hidden_styles()) />
            {children.map(|children| children())}
        </label>
    }
}

/// A headless group of [`Checkbox`]es selecting a set of values (`role="group"`). Label it with
/// a [`Label`](super::field::Label) (or `aria_label`), and add a
/// [`Description`](super::field::Description) and [`FieldError`](super::field::FieldError) as
/// needed.
///
/// Data attributes: `data-disabled`, `data-readonly`, `data-required`, `data-invalid`.
#[allow(clippy::too_many_arguments)]
#[component]
pub fn CheckboxGroup(
    #[prop(into, optional)] default_value: Vec<Key>,
    /// The checked values (controlled): a value or any signal.
    #[prop(into, optional)]
    value: Option<Signal<Vec<Key>>>,
    /// Receives the new state: an `RwSignal`, `WriteSignal`, closure, `Callback`, ...
    #[prop(into, optional)]
    set_value: Option<Out<Vec<Key>>>,
    #[prop(into, optional)] on_change: Option<Callback<Vec<Key>>>,
    #[prop(into, optional)] is_disabled: Signal<bool>,
    #[prop(into, optional)] is_read_only: Signal<bool>,
    /// Whether at least one checkbox must be checked.
    #[prop(into, optional)]
    is_required: Signal<bool>,
    #[prop(into, optional)] is_invalid: Signal<bool>,
    #[prop(optional)] validate: Option<ValidateFn<Vec<Key>>>,
    /// Default: the surrounding [`Form`](super::form::Form)'s, else `Native`.
    #[prop(optional)]
    validation_behavior: Option<ValidationBehavior>,
    /// The checkboxes' `name` (react-aria generates none).
    #[prop(into, optional)]
    name: Option<String>,
    #[prop(into, optional)] form: Option<String>,
    #[prop(into, optional)] id: Option<String>,
    #[prop(into, optional)] aria_label: MaybeProp<String>,
    #[prop(into, optional)] aria_labelledby: Option<String>,
    #[prop(into, optional)] aria_describedby: Option<String>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let validation_behavior = use_validation_behavior(validation_behavior);
    let (value, on_change) =
        crate::utils::ValueBinding::from_state_props(value, set_value, on_change);
    let state = use_checkbox_group_state(UseCheckboxGroupStateInput {
        default_value,
        value,
        on_change,
        is_disabled,
        is_read_only,
        is_required,
        is_invalid,
        validate,
        validation_behavior,
        name,
    });
    // As in react-aria-components: a visible label is expected unless an ARIA label is given.
    let label_presence = LabelPresence::new(aria_label, aria_labelledby.as_ref());
    let has_label = label_presence.has_label;
    let UseCheckboxGroupReturn {
        props,
        label_props,
        description_props,
        error_message_props,
        data,
        is_invalid,
        validation_errors,
        validation_details,
        ..
    } = use_checkbox_group(UseCheckboxGroupInput {
        id,
        has_label,
        aria_label,
        aria_labelledby,
        aria_describedby,
        form,
        ..UseCheckboxGroupInput::new(state)
    });
    let ctx = CheckboxGroupCtx { data };
    let label = LabelContext::span(label_props).with_presence(label_presence);
    let field = FieldContext {
        description: description_props,
        error_message: error_message_props,
        is_invalid,
        validation_errors,
        validation_details,
    };

    view! {
        <Provider value=ctx>
            <Provider value=label><Provider value=field>
                <div
                    {..props.into_attrs()}
                    class=classes
                    style=styles
                    data-disabled=flag(state.is_disabled)
                    data-readonly=flag(state.is_read_only)
                    data-required=flag(is_required)
                    data-invalid=flag(is_invalid)
                >
                    {children()}
                </div>
            </Provider></Provider>
        </Provider>
    }
}
