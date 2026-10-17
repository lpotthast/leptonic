// Upstream: react-aria/src/checkbox/useCheckboxGroup.ts @ 99e6102368
// Upstream: react-aria/src/checkbox/useCheckboxGroupItem.ts @ 99e6102368
// Upstream: react-aria/test/checkbox/useCheckboxGroup.test.tsx @ 99e6102368
// Upstream: react-aria-components/test/CheckboxGroup.test.js @ 99e6102368
use leptos::{ev, prelude::*};
use web_sys::FocusEvent;

use super::{
    use_checkbox::{UseCheckboxInput, UseCheckboxReturn, use_checkbox},
    use_checkbox_group_state::CheckboxGroupState,
    use_field::{UseFieldInput, UseFieldReturn, use_field},
    use_form_validation_state::{
        DEFAULT_VALIDATION_RESULT, FormValidationState, UseFormValidationStateInput,
        ValidationBehavior, ValidationResult, ValidityStateSnapshot, use_form_validation_state,
    },
    use_label::{LabelElementType, UseLabelProps},
    use_toggle::ToggleOptions,
    use_toggle_state::{UseToggleStateInput, use_toggle_state},
};
use crate::{
    EventHandler, Id, IdRefs, IntoAttrs, OnEvent, Part, SlotProps, ValueBinding,
    hooks::{
        collections::Key,
        focus::{FocusWithinEvent, UseFocusWithinInput, use_focus_within},
    },
    utils::{
        aria::{AriaDisabled, AriaRole, aria_true},
        part::Remount,
        probe::either,
        signal_attr::{FixedAttr, SignalAttr, fixed_attr, maybe_attr, signal_attr},
        spread::Spread,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - The group hands its items a `CheckboxGroupData` (react-aria: a `WeakMap` keyed by the state).
// - An item is required when it or its group is (react-aria: the item's `isRequired` replaces
//   the group's when given).
//
// =============================================================================

/// Input of [`use_checkbox_group`].
#[derive(Debug, Clone)]
pub struct UseCheckboxGroupInput {
    pub state: CheckboxGroupState,
    /// The group element's id. Generated when `None`.
    pub id: Option<Id>,
    /// Whether a visible label is rendered (with `label_props`).
    pub has_label: Signal<bool>,
    pub aria_label: MaybeProp<String>,
    pub aria_labelledby: Signal<Option<Id>>,
    pub aria_describedby: Signal<Option<Id>>,
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
    description_id: Signal<Option<Id>>,
    error_message_id: Signal<Option<Id>>,
}

/// The return value of [`use_checkbox_group`].
#[derive(Debug)]
pub struct UseCheckboxGroupReturn {
    /// The group element.
    pub group: Part<UseCheckboxGroupProps>,
    /// The group's label element.
    pub label: Part<UseLabelProps>,
    pub description: Part<SlotProps>,
    pub error_message: Part<SlotProps>,
    /// For [`use_checkbox_group_item`].
    pub data: CheckboxGroupData,
    pub is_invalid: Signal<bool>,
    pub validation_errors: Signal<Vec<String>>,
    pub validation_details: Signal<ValidityStateSnapshot>,
}

/// Props for the checkbox group element.
#[derive(Debug)]
pub struct UseCheckboxGroupProps {
    pub id: Id,
    pub aria_disabled: Signal<Option<AriaDisabled>>,
    pub aria_label: MaybeProp<String>,
    pub aria_labelledby: Signal<Option<Id>>,
    pub aria_describedby: Signal<Option<Id>>,
    pub on_focusin: EventHandler<FocusEvent>,
    pub on_focusout: EventHandler<FocusEvent>,
}

impl Remount for UseCheckboxGroupProps {
    fn remount(&self) -> Self {
        Self {
            id: self.id.clone(),
            aria_disabled: self.aria_disabled,
            aria_label: self.aria_label,
            aria_labelledby: self.aria_labelledby,
            aria_describedby: self.aria_describedby,
            on_focusin: self.on_focusin.clone(),
            on_focusout: self.on_focusout.clone(),
        }
    }
}

pub type UseCheckboxGroupAttrs = Spread<(
    FixedAttr,
    FixedAttr,
    SignalAttr<Option<AriaDisabled>>,
    SignalAttr<Option<String>>,
    SignalAttr<Option<Id>>,
    SignalAttr<Option<Id>>,
    OnEvent<ev::focusin>,
    OnEvent<ev::focusout>,
)>;

impl IntoAttrs for UseCheckboxGroupProps {
    type Attrs = UseCheckboxGroupAttrs;

    fn into_attrs(self) -> Self::Attrs {
        Spread((
            fixed_attr("role", AriaRole::Group),
            fixed_attr("id", self.id),
            signal_attr("aria-disabled", self.aria_disabled),
            maybe_attr("aria-label", self.aria_label),
            signal_attr("aria-labelledby", self.aria_labelledby),
            signal_attr("aria-describedby", self.aria_describedby),
            self.on_focusin.into_on(ev::focusin),
            self.on_focusout.into_on(ev::focusout),
        ))
    }
}

/// Provides the behavior and accessibility of a group of checkboxes (`role="group"` with a
/// label, description and error message). Render its checkboxes with
/// [`use_checkbox_group_item`].
pub fn use_checkbox_group(input: UseCheckboxGroupInput) -> UseCheckboxGroupReturn {
    crate::metrics::subject!();
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
        label: label_props,
        field: field_props,
        description: description_props,
        error_message: error_message_props,
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
    let field_props = field_props.into_props();
    let focus_within = use_focus_within(UseFocusWithinInput {
        on_focus_within: on_focus
            .map(|cb| Callback::new(move |e: FocusWithinEvent| cb.run(e.event))),
        on_blur_within: on_blur.map(|cb| Callback::new(move |e: FocusWithinEvent| cb.run(e.event))),
        on_focus_within_change: on_focus_change,
        ..UseFocusWithinInput::default()
    });
    let focus_within_props = focus_within.target.into_props();
    let is_disabled = state.is_disabled;
    let validation = state.validation;
    UseCheckboxGroupReturn {
        group: Part::stored(UseCheckboxGroupProps {
            id: field_props.id,
            aria_disabled: aria_true(is_disabled),
            aria_label: field_props.aria_label,
            aria_labelledby: field_props.aria_labelledby,
            aria_describedby: field_props.aria_describedby,
            on_focusin: focus_within_props.on_focusin,
            on_focusout: focus_within_props.on_focusout,
        }),
        label: label_props,
        description: description_props,
        error_message: error_message_props,
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
    /// Further settings. `name`, `form` and `validation_behavior` default to the group's; the
    /// input's `value` is `value`. `validate` and `is_invalid` validate the checkbox on its own:
    /// its errors join the group's.
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
pub fn use_checkbox_group_item(input: UseCheckboxGroupItemInput) -> UseCheckboxReturn {
    crate::metrics::subject!();
    let UseCheckboxGroupItemInput {
        group,
        value,
        is_indeterminate,
        on_change,
        mut options,
    } = input;
    let state = group.state;

    options.is_read_only = either(options.is_read_only, state.is_read_only);
    options.is_disabled = either(options.is_disabled, state.is_disabled);
    options.is_required = either(options.is_required, state.is_required);
    options.name = options.name.or_else(|| state.name());
    options.form = options.form.or_else(|| group.form.clone());
    // Submitted with the form as the group's value.
    options.value = Some(value.to_string());
    // The item's own behavior, else the group's: a native item reports its native validity
    // (e.g. a missing required value) to a group that shows it in realtime.
    let validation_behavior = options
        .validation_behavior
        .unwrap_or(state.validation_behavior);
    options.validation_behavior = Some(validation_behavior);

    let toggled_value = value.clone();
    let toggle_state = use_toggle_state(UseToggleStateInput {
        default_selected: false,
        value: Some(ValueBinding::new(
            // Only this checkbox's value's changes (`performance.md` R1).
            {
                let value = value.clone();
                Signal::derive(move || state.is_selected(&value))
            },
            Callback::new(move |is_selected: bool| {
                if is_selected {
                    state.add_value(toggled_value.clone());
                } else {
                    state.remove_value(&toggled_value);
                }
            }),
        )),
        on_change,
        // The item's or the group's (react-aria).
        is_read_only: options.is_read_only,
    });

    // The checkbox's own validation, merged into the group's (server errors are the group's).
    let realtime_validation = use_form_validation_state(UseFormValidationStateInput {
        builtin_validation: Signal::default(),
        is_invalid: options.is_invalid,
        value: toggle_state.is_selected,
        validate: options.validate.take(),
        validation_behavior: ValidationBehavior::Aria,
        names: Vec::new(),
    })
    .realtime_validation;
    // Shown through the group's validation.
    options.is_invalid = Signal::stored(false);
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
    // A memo: the displayed validation and the input's custom validity both read it.
    let combined_realtime: Signal<ValidationResult> = Memo::new(move |_| {
        let group = group_validation.realtime_validation.get();
        if group.is_invalid {
            group
        } else {
            realtime_validation.get()
        }
    })
    .into();
    let display_validation = if validation_behavior == ValidationBehavior::Native {
        group_validation.display_validation
    } else {
        combined_realtime
    };
    // The group's commit and reset (its commit reads every checkbox's native validity).
    let item_validation = FormValidationState::from_parts(
        combined_realtime,
        display_validation,
        Callback::new(move |validation: ValidationResult| {
            native_validation.set_value(validation);
            update_validation();
        }),
        group_validation,
    );

    // Also described by the group's description and (while invalid) its error message.
    let group_error = Signal::derive(move || {
        if state.is_invalid.get() {
            group.error_message_id.get()
        } else {
            None
        }
    });
    options.aria_describedby =
        IdRefs::derive([options.aria_describedby, group_error, group.description_id]);
    use_checkbox(UseCheckboxInput {
        state: toggle_state,
        is_indeterminate,
        validation: Some(item_validation),
        options,
    })
}
