// Upstream: react-aria/src/checkbox/useCheckboxGroup.ts @ 99e6102368
// Upstream: react-aria/src/checkbox/useCheckboxGroupItem.ts @ 99e6102368
use leptos::{
    attr,
    attr::Attr,
    ev,
    ev::{On, SharedEventCallback},
    prelude::*,
};
use web_sys::FocusEvent;

use super::{
    use_checkbox::{UseCheckboxInput, UseCheckboxReturn, use_checkbox_with},
    use_checkbox_group_state::CheckboxGroupState,
    use_field::{UseFieldInput, UseFieldReturn, use_field},
    use_form_validation_state::{
        DEFAULT_VALIDATION_RESULT, UseFormValidationStateInput, UseFormValidationStateReturn,
        ValidateFn, ValidationBehavior, ValidationResult, ValidityStateSnapshot,
        use_form_validation_state,
    },
    use_label::{LabelElementType, UseLabelProps},
    use_toggle::ToggleOptions,
    use_toggle_state::ToggleState,
};
use crate::{
    hooks::{IntoAttrs, PropsWithStyles, UseFocusWithinInput, collections::Key, use_focus_within},
    utils::{
        EventHandler, SlotProps,
        aria::{AriaDisabled, AriaRole},
        join_slot_ids,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - The group hands its items a `CheckboxGroupData` (react-aria: a `WeakMap` keyed by the state).
// - Items take the group's validation behavior (react-aria: an item may override it).
// - An item is required when it or its group is (react-aria: the item's `isRequired` replaces
//   the group's when given).
//
// =============================================================================

/// Input of [`use_checkbox_group`].
#[derive(Debug, Clone)]
pub struct UseCheckboxGroupInput {
    pub state: CheckboxGroupState,
    /// The group element's id. Generated when `None`.
    pub id: Option<String>,
    /// Whether a visible label is rendered (with `label_props`).
    pub has_label: Signal<bool>,
    pub aria_label: MaybeProp<String>,
    pub aria_labelledby: Option<String>,
    pub aria_describedby: Option<String>,
    /// The id of the form the checkboxes belong to, when not their ancestor.
    pub form: Option<String>,
    pub on_focus: Option<Callback<FocusEvent>>,
    pub on_blur: Option<Callback<FocusEvent>>,
    pub on_focus_change: Option<Callback<bool>>,
}

/// What the checkboxes of a group need from it.
#[derive(Debug, Clone)]
pub struct CheckboxGroupData {
    pub state: CheckboxGroupState,
    form: Option<String>,
    description_id: Signal<Option<String>>,
    error_message_id: Signal<Option<String>>,
}

/// Output of [`use_checkbox_group`].
#[derive(Debug)]
pub struct UseCheckboxGroupReturn {
    /// Props for the group element.
    pub props: UseCheckboxGroupProps,
    /// Props for the group's label (a `<span>`).
    pub label_props: UseLabelProps,
    pub description_props: SlotProps,
    pub error_message_props: SlotProps,
    /// For [`use_checkbox_group_item`].
    pub data: CheckboxGroupData,
    pub is_invalid: Signal<bool>,
    pub validation_errors: Signal<Vec<String>>,
    pub validation_details: Signal<ValidityStateSnapshot>,
}

/// Props for the checkbox group element.
#[derive(Debug)]
pub struct UseCheckboxGroupProps {
    pub id: String,
    pub aria_disabled: Signal<Option<AriaDisabled>>,
    pub aria_label: MaybeProp<String>,
    pub aria_labelledby: Signal<Option<String>>,
    pub aria_describedby: Signal<Option<String>>,
    pub on_focusin: EventHandler<FocusEvent>,
    pub on_focusout: EventHandler<FocusEvent>,
}

pub type UseCheckboxGroupAttrs = (
    Attr<attr::Role, AriaRole>,
    Attr<attr::Id, String>,
    Attr<attr::AriaDisabled, Signal<Option<AriaDisabled>>>,
    Attr<attr::AriaLabel, MaybeProp<String>>,
    Attr<attr::AriaLabelledby, Signal<Option<String>>>,
    Attr<attr::AriaDescribedby, Signal<Option<String>>>,
    On<ev::focusin, SharedEventCallback<FocusEvent>>,
    On<ev::focusout, SharedEventCallback<FocusEvent>>,
);

impl IntoAttrs for UseCheckboxGroupProps {
    type Attrs = UseCheckboxGroupAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Role, AriaRole::Group),
            Attr(attr::Id, self.id),
            Attr(attr::AriaDisabled, self.aria_disabled),
            Attr(attr::AriaLabel, self.aria_label),
            Attr(attr::AriaLabelledby, self.aria_labelledby),
            Attr(attr::AriaDescribedby, self.aria_describedby),
            self.on_focusin.into_on(ev::focusin),
            self.on_focusout.into_on(ev::focusout),
        )
    }
}

/// Provides the behavior and accessibility of a group of checkboxes (`role="group"` with a
/// label, description and error message). Render its checkboxes with
/// [`use_checkbox_group_item`].
pub fn use_checkbox_group(input: UseCheckboxGroupInput) -> UseCheckboxGroupReturn {
    let UseCheckboxGroupInput {
        state,
        id,
        has_label,
        aria_label,
        aria_labelledby,
        aria_describedby,
        form,
        on_focus,
        on_blur,
        on_focus_change,
    } = input;
    let UseFieldReturn {
        label_props,
        field_props,
        description_props,
        error_message_props,
        description_id,
        error_message_id,
    } = use_field(UseFieldInput {
        id,
        has_label,
        label_element_type: LabelElementType::Span,
        aria_label,
        aria_labelledby,
        aria_describedby,
        ..UseFieldInput::default()
    });
    let focus_within = use_focus_within(UseFocusWithinInput {
        on_focus_within: on_focus
            .map(|cb| Callback::new(move |e: crate::hooks::FocusWithinEvent| cb.run(e.event))),
        on_blur_within: on_blur
            .map(|cb| Callback::new(move |e: crate::hooks::FocusWithinEvent| cb.run(e.event))),
        on_focus_within_change: on_focus_change,
        ..UseFocusWithinInput::default()
    });
    let is_disabled = state.is_disabled;
    let validation = state.validation;
    UseCheckboxGroupReturn {
        props: UseCheckboxGroupProps {
            id: field_props.id,
            aria_disabled: Signal::derive(move || is_disabled.get().then_some(AriaDisabled::True)),
            aria_label: field_props.aria_label,
            aria_labelledby: field_props.aria_labelledby,
            aria_describedby: field_props.aria_describedby,
            on_focusin: focus_within.props.on_focusin,
            on_focusout: focus_within.props.on_focusout,
        },
        label_props,
        description_props,
        error_message_props,
        data: CheckboxGroupData {
            state,
            form,
            description_id,
            error_message_id,
        },
        is_invalid: state.is_invalid,
        validation_errors: validation.validation_errors,
        validation_details: Signal::derive(move || {
            validation.display_validation.get().validation_details
        }),
    }
}

/// Input of [`use_checkbox_group_item`].
#[derive(Clone)]
pub struct UseCheckboxGroupItemInput {
    pub group: CheckboxGroupData,
    /// The checkbox's value in the group.
    pub value: Key,
    /// Shows the checkbox as partially checked, regardless of its selection.
    pub is_indeterminate: Signal<bool>,
    /// Called when the checkbox is checked or unchecked.
    pub on_change: Option<Callback<bool>>,
    /// Validates the checkbox on its own (its errors join the group's).
    pub validate: Option<ValidateFn<bool>>,
    /// Further settings. `name` and `form` default to the group's; the input's `value` is
    /// `value`; the group's validation behavior applies.
    pub options: ToggleOptions,
}

impl std::fmt::Debug for UseCheckboxGroupItemInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UseCheckboxGroupItemInput")
            .field("value", &self.value)
            .finish_non_exhaustive()
    }
}

/// Provides the behavior and accessibility of a checkbox in a [`use_checkbox_group`].
#[allow(clippy::needless_pass_by_value)]
pub fn use_checkbox_group_item(input: UseCheckboxGroupItemInput) -> UseCheckboxReturn {
    let UseCheckboxGroupItemInput {
        group,
        value,
        is_indeterminate,
        on_change,
        validate,
        mut options,
    } = input;
    let state = group.state;

    let item_read_only = options.is_read_only;
    let item_disabled = options.is_disabled;
    let item_required = options.is_required;
    options.is_read_only = Signal::derive(move || item_read_only.get() || state.is_read_only.get());
    options.is_disabled = Signal::derive(move || item_disabled.get() || state.is_disabled.get());
    options.is_required = Signal::derive(move || item_required.get() || state.is_required.get());
    options.name = options.name.or_else(|| state.name());
    options.form = options.form.or_else(|| group.form.clone());
    // Submitted with the form as the group's value.
    options.value = Some(value.to_string());
    options.validation_behavior = state.validation_behavior;

    let selected_value = value.clone();
    let toggled_value = value.clone();
    let toggle_state = ToggleState::new(
        Signal::derive(move || state.is_selected(&selected_value)),
        state.default_value().contains(&value),
        Callback::new(move |is_selected: bool| {
            if item_read_only.get_untracked() {
                return;
            }
            if is_selected {
                state.add_value(toggled_value.clone());
            } else {
                state.remove_value(&toggled_value);
            }
            if let Some(on_change) = on_change {
                on_change.run(is_selected);
            }
        }),
    );

    // The checkbox's own validation, merged into the group's.
    let realtime_validation = use_form_validation_state(UseFormValidationStateInput {
        builtin_validation: Signal::default(),
        is_invalid: Signal::stored(false),
        value: toggle_state.is_selected,
        validate,
        validation_behavior: ValidationBehavior::Aria,
        name: None,
    })
    .realtime_validation;
    let native_validation = StoredValue::new(DEFAULT_VALIDATION_RESULT);
    let update_validation = {
        let value = value.clone();
        move || {
            let realtime = realtime_validation.get_untracked();
            let validation = if realtime.is_invalid {
                realtime
            } else {
                native_validation.get_value()
            };
            state.set_invalid(value.clone(), validation);
        }
    };
    {
        let update_validation = update_validation.clone();
        Effect::new(move || {
            realtime_validation.track();
            update_validation();
        });
    }
    let group_validation = state.validation;
    let combined_realtime = Signal::derive(move || {
        let group = group_validation.realtime_validation.get();
        if group.is_invalid {
            group
        } else {
            realtime_validation.get()
        }
    });
    let display_validation = if state.validation_behavior == ValidationBehavior::Native {
        group_validation.display_validation
    } else {
        combined_realtime
    };
    let item_validation = UseFormValidationStateReturn {
        realtime_validation: combined_realtime,
        display_validation,
        is_invalid: Signal::derive(move || display_validation.get().is_invalid),
        validation_errors: Signal::derive(move || display_validation.get().validation_errors),
        update_validation: Callback::new(move |validation: ValidationResult| {
            native_validation.set_value(validation);
            update_validation();
        }),
        reset_validation: group_validation.reset_validation,
        commit_validation: group_validation.commit_validation,
        // The group's commit reads every checkbox's native validity.
        native_validity_readers: group_validation.native_validity_readers,
    };

    let mut checkbox = use_checkbox_with(
        UseCheckboxInput {
            state: toggle_state,
            is_indeterminate,
            options,
        },
        Some(item_validation),
    );

    // Also described by the group's description and (while invalid) its error message.
    let (mut input_props, input_styles) = checkbox.input_props.into_inner();
    let own = input_props.aria_describedby;
    let group_error = Signal::derive(move || {
        if state.is_invalid.get() {
            group.error_message_id.get()
        } else {
            None
        }
    });
    input_props.aria_describedby = join_slot_ids(&[own, group_error, group.description_id]);
    checkbox.input_props = PropsWithStyles::new(input_props, input_styles);
    checkbox
}
