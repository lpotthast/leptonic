// Upstream: react-aria-components/src/Checkbox.tsx @ 99e6102368
// Upstream: react-aria-components/test/Checkbox.test.js @ 99e6102368
// Upstream: react-aria-components/test/CheckboxGroup.test.js @ 99e6102368
use std::collections::HashSet;

use leptos::prelude::*;
use leptos_classes::Classes;

use super::{
    field::{FieldContext, LabelContext, LabelPresence},
    form::use_validation_behavior,
    typed_values::{KeyedStateProps, keyed_state_props},
};
use crate::{
    IntoAttrs, Out, Part, PropsWithStyles, ValueBinding,
    hooks::{
        collections::{Key, SelectionValue},
        form::{
            CheckboxGroupData, ToggleOptions, UseCheckboxGroupInput, UseCheckboxGroupItemInput,
            UseCheckboxGroupReturn, UseCheckboxGroupStateInput, UseCheckboxInput,
            UseToggleInputAttrs, UseToggleInputProps, UseToggleLabelAttrs, UseToggleLabelProps,
            UseToggleReturn, UseToggleStateInput, ValidateFn, ValidationBehavior, use_checkbox,
            use_checkbox_group, use_checkbox_group_item, use_checkbox_group_state,
            use_toggle_state,
        },
        interactions::{UseHoverInput, UseHoverProps, UseHoverReturn, use_hover},
    },
    sharing_one_effect,
    utils::{
        data_attributes::data_flags, default_class::with_default_class, dev_warn, probe::either,
        scoped_context::scoped_view, styles::Styles, visually_hidden::visually_hidden_styles,
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
pub struct CheckboxGroupContext {
    pub data: CheckboxGroupData,
    /// What a checkbox's own `FieldError` shows in the group (nothing: the group validates), one
    /// pair of constants for every checkbox.
    no_field_errors: (Signal<bool>, Signal<Vec<String>>),
}

/// A headless checkbox with a description and an error message of its own: a `<div>` around a
/// [`CheckboxButton`] (the clickable `<label>` with the box), a
/// [`Description`](super::field::Description) and a [`FieldError`](super::field::FieldError). In a
/// [`CheckboxGroup`], the group validates, so a `FieldError` shows nothing.
///
/// Data attributes: `data-selected`, `data-indeterminate`, `data-disabled`, `data-readonly`,
/// `data-invalid`, `data-required`.
///
/// Inside a [`CheckboxGroup`], `value` is required (without one, the field warns in debug builds
/// and renders nothing) and the group holds the selection (`default_selected`, `is_selected` and
/// `set_selected` don't apply).
///
/// Default class: `leptonic-CheckboxField`.
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
    id: Option<crate::Id>,
    #[prop(into, optional)] aria_label: MaybeProp<String>,
    #[prop(into, optional)] aria_labelledby: MaybeProp<String>,
    #[prop(into, optional)] aria_describedby: MaybeProp<String>,
    #[prop(optional)] auto_focus: bool,
    #[prop(into, optional)] on_focus_change: Option<Callback<bool>>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    crate::metrics::subject!();
    let classes = with_default_class("leptonic-CheckboxField", classes);
    let group = use_context::<CheckboxGroupContext>();
    let in_group = group.is_some();
    if in_group && value.is_none() {
        dev_warn!("A <CheckboxField> in a <CheckboxGroup> needs a `value`.");
        return None;
    }
    // The checkbox and its button's hover end their state while disabled in one effect
    // (`performance.md` R6).
    let no_field_errors = group.as_ref().map(|group| group.no_field_errors);
    let (checkbox, hover) = sharing_one_effect(|| {
        let checkbox = use_checkbox_atom(CheckboxSetup {
            group,
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
        let hover = toggle_hover(&checkbox);
        (checkbox, hover)
    });
    let (is_selected, is_disabled, is_read_only, is_invalid) = (
        checkbox.is_selected,
        checkbox.is_disabled,
        checkbox.is_read_only,
        checkbox.is_invalid,
    );
    // In a group, the group's validation shows (react-aria-components: no `FieldErrorContext`).
    let (field_is_invalid, validation_errors) =
        no_field_errors.unwrap_or((is_invalid, checkbox.validation_errors));
    let field = FieldContext {
        description: checkbox.description.clone(),
        error_message: checkbox.error_message.clone(),
        is_invalid: field_is_invalid,
        validation_errors,
        validation_details: checkbox.validation_details,
    };
    let button = CheckboxButtonContext {
        button: ToggleButtonParts::of(checkbox, hover),
        is_indeterminate,
        is_required,
    };

    // Both contexts in one scope (one owner, no `<Provider>` per context: `performance.md` R11).
    Some(scoped_view(
        move || {
            provide_context(button);
            provide_context(field);
        },
        move || {
            view! {
                <div
                    class=classes
                    style={styles}
                    {..data_flags([
                        ("data-selected", is_selected),
                        ("data-indeterminate", is_indeterminate),
                        ("data-disabled", is_disabled),
                    ])}
                    {..data_flags([
                        ("data-readonly", is_read_only),
                        ("data-invalid", is_invalid),
                        ("data-required", is_required),
                    ])}
                >
                    {children()}
                </div>
            }
        },
    ))
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
    crate::metrics::subject!();
    let classes = with_default_class("leptonic-CheckboxButton", classes);
    let Some(ctx) = use_context::<CheckboxButtonContext>() else {
        dev_warn!("a <CheckboxButton> belongs in a <CheckboxField>");
        return None;
    };
    // One button per field (`Part`: a second one renders nothing).
    let parts = ctx.button.props()?;
    Some(checkbox_button(
        parts,
        ctx.is_indeterminate,
        ctx.is_required,
        classes,
        styles,
        children,
    ))
}

/// What a [`CheckboxField`] hands its [`CheckboxButton`].
#[derive(Clone)]
struct CheckboxButtonContext {
    /// The checkbox, for the button rendered.
    button: ToggleButtonParts,
    is_indeterminate: Signal<bool>,
    is_required: Signal<bool>,
}

/// The settings of a [`CheckboxField`].
struct CheckboxSetup {
    /// The surrounding group's context, if any.
    group: Option<CheckboxGroupContext>,
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
    id: Option<crate::Id>,
    aria_label: MaybeProp<String>,
    aria_labelledby: MaybeProp<String>,
    aria_describedby: MaybeProp<String>,
    auto_focus: bool,
    on_focus_change: Option<Callback<bool>>,
}

/// The checkbox of a [`CheckboxField`]: an item of the surrounding [`CheckboxGroup`] (given a
/// `value`; the field renders nothing without one), or a checkbox with its own state.
fn use_checkbox_atom(setup: CheckboxSetup) -> UseToggleReturn {
    let CheckboxSetup {
        group,
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
    let group = group.zip(value);
    let validation_behavior = if group.is_some() {
        validation_behavior
    } else {
        Some(use_validation_behavior(validation_behavior))
    };
    let options = ToggleOptions {
        id,
        is_disabled,
        is_read_only,
        is_required,
        is_invalid,
        validate,
        validation_behavior,
        name,
        form,
        value: form_value,
        aria_label,
        aria_labelledby: crate::utils::id::id_ref_prop(aria_labelledby),
        aria_describedby: crate::utils::id::id_ref_prop(aria_describedby),
        auto_focus,
        on_focus_change,
        // Every field named: `..ToggleOptions::default()` would create a stored signal for each
        // `Signal` field, also the ones set here (`performance.md` R14).
        aria_errormessage: Signal::stored(None),
        aria_controls: Signal::stored(None),
        exclude_from_tab_order: Signal::stored(false),
        on_focus: None,
        on_blur: None,
        on_press_start: None,
        on_press_end: None,
        on_press_up: None,
        on_press: None,
        on_press_change: None,
    };
    if let Some((group, value)) = group {
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
            validation: None,
        })
    }
}

/// The hover of a toggle's button (`CheckboxButton`, `SwitchButton`), created by its field with the
/// toggle (in the same `sharing_one_effect`).
pub(super) fn toggle_hover(toggle: &UseToggleReturn) -> UseHoverReturn {
    use_hover(UseHoverInput {
        is_disabled: either(toggle.is_disabled, toggle.is_read_only),
        on_hover_start: None,
        on_hover_end: None,
        on_hover_change: None,
    })
}

/// What a toggle's field (`CheckboxField`, `SwitchField`) hands its button: the toggle's label
/// and input parts and its state, for the button rendered (again: e.g. inside a `Show`).
#[derive(Clone)]
pub(super) struct ToggleButtonParts {
    label: Part<PropsWithStyles<UseToggleLabelProps>>,
    input: Part<PropsWithStyles<UseToggleInputProps>>,
    hover: Part<UseHoverProps>,
    is_hovered: Signal<bool>,
    is_selected: Signal<bool>,
    is_pressed: Signal<bool>,
    is_focused: Signal<bool>,
    is_focus_visible: Signal<bool>,
    is_disabled: Signal<bool>,
    is_read_only: Signal<bool>,
    is_invalid: Signal<bool>,
}

/// The rendered button's attributes and state (see [`ToggleButtonParts::props`]).
pub(super) struct ToggleButton {
    pub(super) label: (UseToggleLabelAttrs, Styles),
    pub(super) input: (UseToggleInputAttrs, Styles),
    pub(super) hover: UseHoverProps,
    pub(super) is_hovered: Signal<bool>,
    pub(super) is_selected: Signal<bool>,
    pub(super) is_pressed: Signal<bool>,
    pub(super) is_focused: Signal<bool>,
    pub(super) is_focus_visible: Signal<bool>,
    pub(super) is_disabled: Signal<bool>,
    pub(super) is_read_only: Signal<bool>,
    pub(super) is_invalid: Signal<bool>,
}

impl ToggleButtonParts {
    pub(super) fn of(toggle: UseToggleReturn, hover: UseHoverReturn) -> Self {
        Self {
            label: toggle.label,
            input: toggle.input,
            hover: hover.target,
            is_hovered: hover.is_hovered,
            is_selected: toggle.is_selected,
            is_pressed: toggle.is_pressed,
            is_focused: toggle.is_focused,
            is_focus_visible: toggle.is_focus_visible,
            is_disabled: toggle.is_disabled,
            is_read_only: toggle.is_read_only,
            is_invalid: toggle.is_invalid,
        }
    }

    /// The props of the button being rendered; `None` while another button holds them.
    #[track_caller]
    pub(super) fn props(&self) -> Option<ToggleButton> {
        let label = self.label.props()?.into_parts();
        let input = self.input.props()?.into_parts();
        let hover = self.hover.props()?;
        Some(ToggleButton {
            label,
            input,
            hover,
            is_hovered: self.is_hovered,
            is_selected: self.is_selected,
            is_pressed: self.is_pressed,
            is_focused: self.is_focused,
            is_focus_visible: self.is_focus_visible,
            is_disabled: self.is_disabled,
            is_read_only: self.is_read_only,
            is_invalid: self.is_invalid,
        })
    }
}

fn checkbox_button(
    checkbox: ToggleButton,
    is_indeterminate: Signal<bool>,
    is_required: Signal<bool>,
    classes: Classes,
    styles: Styles,
    children: Option<Children>,
) -> impl IntoView {
    let (label_attrs, label_styles) = checkbox.label;
    let (input_attrs, input_styles) = checkbox.input;

    view! {
        <label
            {..label_attrs}
            {..checkbox.hover.into_attrs()}
            class=classes
            style=label_styles.merge(styles)
            {..data_flags([
                ("data-pressed", checkbox.is_pressed),
                ("data-hovered", checkbox.is_hovered),
                ("data-focused", checkbox.is_focused),
            ])}
            {..data_flags([("data-focus-visible", checkbox.is_focus_visible)])}
            {..data_flags([
                ("data-selected", checkbox.is_selected),
                ("data-indeterminate", is_indeterminate),
                ("data-disabled", checkbox.is_disabled),
            ])}
            {..data_flags([
                ("data-readonly", checkbox.is_read_only),
                ("data-invalid", checkbox.is_invalid),
                ("data-required", is_required),
            ])}
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
// All controlled and uncontrolled selection props share the hook's HashSet representation.
#[allow(clippy::implicit_hasher)]
#[component]
pub fn CheckboxGroup<V: SelectionValue>(
    /// The initially checked values. Ignored with `value`.
    #[prop(optional)]
    default_value: HashSet<V>,
    /// The checked values (controlled): a value or any signal.
    #[prop(into, optional)]
    value: Option<Signal<HashSet<V>>>,
    /// Receives the new state: an `RwSignal`, `WriteSignal`, closure, `Callback`, ...
    #[prop(into, optional)]
    set_value: Option<Out<HashSet<V>>>,
    #[prop(into, optional)] on_change: Option<Callback<HashSet<V>>>,
    #[prop(into, optional)] is_disabled: Signal<bool>,
    #[prop(into, optional)] is_read_only: Signal<bool>,
    /// Whether at least one checkbox must be checked.
    #[prop(into, optional)]
    is_required: Signal<bool>,
    #[prop(into, optional)] is_invalid: Signal<bool>,
    #[prop(optional)] validate: Option<ValidateFn<HashSet<V>>>,
    /// Default: the surrounding [`Form`](super::form::Form)'s, else `Native`.
    #[prop(optional)]
    validation_behavior: Option<ValidationBehavior>,
    /// The checkboxes' `name` (react-aria generates none).
    #[prop(into, optional)]
    name: Option<String>,
    #[prop(into, optional)] form: Option<String>,
    #[prop(into, optional)] id: Option<crate::Id>,
    #[prop(into, optional)] aria_label: MaybeProp<String>,
    #[prop(into, optional)] aria_labelledby: MaybeProp<String>,
    #[prop(into, optional)] aria_describedby: MaybeProp<String>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    crate::metrics::subject!();
    let classes = with_default_class("leptonic-CheckboxGroup", classes);
    let validation_behavior = use_validation_behavior(validation_behavior);
    let KeyedStateProps {
        default_value,
        value,
        set_value,
        on_change,
        validate,
    } = keyed_state_props(Some(default_value), value, set_value, on_change, validate);
    let (value, on_change) = ValueBinding::from_state_props(value, set_value, on_change);
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
    let label_presence = LabelPresence::new(aria_label, aria_labelledby);
    let has_label = label_presence.has_label;
    let UseCheckboxGroupReturn {
        group: props,
        label: label_props,
        description: description_props,
        error_message: error_message_props,
        data,
        is_invalid,
        validation_errors,
        validation_details,
        ..
    } = use_checkbox_group(UseCheckboxGroupInput {
        id,
        has_label,
        aria_label,
        aria_labelledby: crate::utils::id::id_ref_prop(aria_labelledby),
        aria_describedby: crate::utils::id::id_ref_prop(aria_describedby),
        form,
        state,
        on_focus: None,
        on_blur: None,
        on_focus_change: None,
    });
    let ctx = CheckboxGroupContext {
        data,
        no_field_errors: (Signal::stored(false), Signal::stored(Vec::new())),
    };
    let label = LabelContext::span(label_props).with_presence(label_presence);
    let field = FieldContext {
        description: description_props,
        error_message: error_message_props,
        is_invalid,
        validation_errors,
        validation_details,
    };

    scoped_view(
        move || {
            provide_context(ctx);
            provide_context(label);
            provide_context(field);
        },
        move || {
            view! {
                <div
                    {..props.attrs()}
                    class=classes
                    style={styles}
                    {..data_flags([
                        ("data-disabled", state.is_disabled),
                        ("data-readonly", state.is_read_only),
                        ("data-required", is_required),
                    ])}
                    {..data_flags([("data-invalid", is_invalid)])}
                >
                    {children()}
                </div>
            }
        },
    )
}
