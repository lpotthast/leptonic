// Upstream: react-aria-components/src/Checkbox.tsx @ 99e6102368
use leptos::{context::Provider, prelude::*};

use super::{
    field::{FieldContext, LabelContext},
    form::use_validation_behavior,
    typed_values::{KeyedStateProps, keyed_state_props},
};
use crate::{
    Out,
    atoms::field::LabelPresence,
    hooks::{
        CheckboxGroupData, IntoAttrs, ToggleOptions, UseCheckboxGroupInput,
        UseCheckboxGroupItemInput, UseCheckboxGroupReturn, UseCheckboxGroupStateInput,
        UseCheckboxInput, UseCheckboxReturn, UseHoverInput, UseToggleStateInput, ValidateFn,
        ValidationBehavior,
        collections::{Key, SelectionValue},
        use_checkbox, use_checkbox_group, use_checkbox_group_item, use_checkbox_group_state,
        use_hover, use_toggle_state,
    },
    utils::{
        ValueBinding, classes::Classes, data_attributes::flag, default_class::with_default_class,
        dev_warn, styles::Styles, visually_hidden::visually_hidden_styles,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - State (C4): `default_selected` + `on_change`, or `is_selected` + `set_selected`; the group's
//   `default_value` + `on_change`, or `value` + `set_value` (react-aria: `isSelected`/`value`
//   + `onChange`). The group's values are typed (`V: SelectionValue`); checkboxes take theirs as
//   a `Key` (`From<V> for Key`). React-aria: strings.
// - Render props become `data-*` attributes plus plain children.
// - No single `Checkbox` (react-aria-components deprecates it): a checkbox is a `CheckboxField`
//   with its `CheckboxButton`.
//
// =============================================================================

/// Context from [`CheckboxGroup`] to its checkboxes.
#[derive(Clone)]
pub struct CheckboxGroupCtx {
    pub data: CheckboxGroupData,
}

/// A headless checkbox with a description and an error message of its own: a `<div>` around a
/// [`CheckboxButton`] (the clickable `<label>` with the box), a
/// [`Description`](super::field::Description) and a [`FieldError`](super::field::FieldError). In a
/// [`CheckboxGroup`], the group validates, so a `FieldError` shows nothing.
///
/// Data attributes: `data-selected`, `data-indeterminate`, `data-disabled`, `data-readonly`,
/// `data-invalid`, `data-required`.
///
/// Inside a [`CheckboxGroup`], `value` is required and the group holds the selection
/// (`default_selected`, `is_selected` and `set_selected` don't apply).
///
/// Default class: `leptonic-CheckboxField`.
#[allow(clippy::too_many_arguments, clippy::needless_pass_by_value)]
#[component]
pub fn CheckboxField(
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
    children: Children,
) -> impl IntoView {
    let classes = with_default_class("leptonic-CheckboxField", classes);
    let in_group = use_context::<CheckboxGroupCtx>().is_some();
    let checkbox = use_checkbox_atom(CheckboxSetup {
        value,
        default_selected,
        on_change,
        is_selected,
        set_selected,
        is_indeterminate,
        is_disabled,
        is_read_only,
        is_required,
        is_invalid,
        validate,
        validation_behavior,
        name,
        form_value,
        form,
        id,
        aria_label,
        aria_labelledby,
        aria_describedby,
        auto_focus,
        on_focus_change,
    });
    let (is_selected, is_disabled, is_read_only, is_invalid) = (
        checkbox.is_selected,
        checkbox.is_disabled,
        checkbox.is_read_only,
        checkbox.is_invalid,
    );
    // In a group, the group's validation shows (react-aria-components: no `FieldErrorContext`).
    let field = FieldContext {
        description: checkbox.description_props.clone(),
        error_message: checkbox.error_message_props.clone(),
        is_invalid: if in_group {
            Signal::stored(false)
        } else {
            is_invalid
        },
        validation_errors: if in_group {
            Signal::stored(Vec::new())
        } else {
            checkbox.validation_errors
        },
        validation_details: checkbox.validation_details,
    };
    let button = CheckboxButtonCtx {
        checkbox: StoredValue::new(Some(checkbox)),
        is_indeterminate,
        is_required,
    };

    view! {
        <Provider value=button>
            <Provider value=field>
                <div
                    class=classes
                    style=styles
                    data-selected=flag(is_selected)
                    data-indeterminate=flag(is_indeterminate)
                    data-disabled=flag(is_disabled)
                    data-readonly=flag(is_read_only)
                    data-invalid=flag(is_invalid)
                    data-required=flag(is_required)
                >
                    {children()}
                </div>
            </Provider>
        </Provider>
    }
}

/// The clickable part of a [`CheckboxField`]: a `<label>` around a visually hidden
/// `<input type="checkbox">` and the children (the box and the label text).
///
/// Data attributes: `data-selected`, `data-indeterminate`, `data-pressed`, `data-hovered`,
/// `data-focused`, `data-focus-visible`, `data-disabled`, `data-readonly`, `data-invalid`,
/// `data-required`.
///
/// Default class: `leptonic-CheckboxButton`.
#[component]
pub fn CheckboxButton(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let classes = with_default_class("leptonic-CheckboxButton", classes);
    let Some(ctx) = use_context::<CheckboxButtonCtx>() else {
        dev_warn!("a <CheckboxButton> belongs in a <CheckboxField>");
        return ().into_any();
    };
    let Some(checkbox) = ctx.checkbox.try_update_value(Option::take).flatten() else {
        dev_warn!("a <CheckboxField> has one <CheckboxButton>");
        return ().into_any();
    };
    checkbox_button(
        checkbox,
        ctx.is_indeterminate,
        ctx.is_required,
        classes,
        styles,
        children,
    )
    .into_any()
}

/// What a [`CheckboxField`] hands its [`CheckboxButton`].
#[derive(Clone)]
struct CheckboxButtonCtx {
    /// The checkbox, taken by the button.
    checkbox: StoredValue<Option<UseCheckboxReturn>>,
    is_indeterminate: Signal<bool>,
    is_required: Signal<bool>,
}

/// The settings of a [`CheckboxField`].
struct CheckboxSetup {
    value: Option<Key>,
    default_selected: bool,
    on_change: Option<Callback<bool>>,
    is_selected: Option<Signal<bool>>,
    set_selected: Option<Out<bool>>,
    is_indeterminate: Signal<bool>,
    is_disabled: Signal<bool>,
    is_read_only: Signal<bool>,
    is_required: Signal<bool>,
    is_invalid: Signal<bool>,
    validate: Option<ValidateFn<bool>>,
    validation_behavior: Option<ValidationBehavior>,
    name: Option<String>,
    form_value: Option<String>,
    form: Option<String>,
    id: Option<String>,
    aria_label: MaybeProp<String>,
    aria_labelledby: Option<String>,
    aria_describedby: Option<String>,
    auto_focus: bool,
    on_focus_change: Option<Callback<bool>>,
}

/// The checkbox of a [`CheckboxField`]: an item of the surrounding
/// [`CheckboxGroup`], or a checkbox with its own state.
fn use_checkbox_atom(setup: CheckboxSetup) -> UseCheckboxReturn {
    let CheckboxSetup {
        value,
        default_selected,
        on_change,
        is_selected,
        set_selected,
        is_indeterminate,
        is_disabled,
        is_read_only,
        is_required,
        is_invalid,
        validate,
        validation_behavior,
        name,
        form_value,
        form,
        id,
        aria_label,
        aria_labelledby,
        aria_describedby,
        auto_focus,
        on_focus_change,
    } = setup;
    let options = ToggleOptions {
        id,
        is_disabled,
        is_read_only,
        is_required,
        is_invalid,
        validate,
        validation_behavior: Some(use_validation_behavior(validation_behavior)),
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
    if let Some(group) = use_context::<CheckboxGroupCtx>() {
        let value = value.expect("a checkbox in a <CheckboxGroup> needs a `value`");
        use_checkbox_group_item(UseCheckboxGroupItemInput {
            is_indeterminate,
            on_change,
            options,
            group: group.data,
            value,
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
            state,
            is_indeterminate,
            options,
        })
    }
}

/// The `<label>` of a [`CheckboxButton`].
fn checkbox_button(
    checkbox: UseCheckboxReturn,
    is_indeterminate: Signal<bool>,
    is_required: Signal<bool>,
    classes: Classes,
    styles: Styles,
    children: Option<Children>,
) -> impl IntoView {
    let hover = use_hover(UseHoverInput {
        is_disabled: Signal::derive(move || {
            checkbox.is_disabled.get() || checkbox.is_read_only.get()
        }),
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

/// A headless group of [`CheckboxField`]s selecting a set of values (`role="group"`). Label it with
/// a [`Label`](super::field::Label) (or `aria_label`), and add a
/// [`Description`](super::field::Description) and [`FieldError`](super::field::FieldError) as
/// needed.
///
/// Data attributes: `data-disabled`, `data-readonly`, `data-required`, `data-invalid`.
///
/// Default class: `leptonic-CheckboxGroup`.
#[allow(clippy::too_many_arguments)]
#[component]
pub fn CheckboxGroup<V: SelectionValue>(
    /// The initially checked values. Ignored with `value`.
    #[prop(optional)]
    default_value: Vec<V>,
    /// The checked values (controlled): a value or any signal.
    #[prop(into, optional)]
    value: Option<Signal<Vec<V>>>,
    /// Receives the new state: an `RwSignal`, `WriteSignal`, closure, `Callback`, ...
    #[prop(into, optional)]
    set_value: Option<Out<Vec<V>>>,
    #[prop(into, optional)] on_change: Option<Callback<Vec<V>>>,
    #[prop(into, optional)] is_disabled: Signal<bool>,
    #[prop(into, optional)] is_read_only: Signal<bool>,
    /// Whether at least one checkbox must be checked.
    #[prop(into, optional)]
    is_required: Signal<bool>,
    #[prop(into, optional)] is_invalid: Signal<bool>,
    #[prop(optional)] validate: Option<ValidateFn<Vec<V>>>,
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
    let classes = with_default_class("leptonic-CheckboxGroup", classes);
    let validation_behavior = use_validation_behavior(validation_behavior);
    let KeyedStateProps {
        default_value,
        value,
        set_value,
        on_change,
        validate,
    } = keyed_state_props(Some(default_value), value, set_value, on_change, validate);
    let (value, on_change) =
        crate::utils::ValueBinding::from_state_props(value, set_value, on_change);
    let state = use_checkbox_group_state(UseCheckboxGroupStateInput {
        default_value: default_value.unwrap_or_default(),
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
        state,
        on_focus: None,
        on_blur: None,
        on_focus_change: None,
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
