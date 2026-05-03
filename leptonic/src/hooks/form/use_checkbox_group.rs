use std::{collections::HashSet, hash::Hash};

use leptos::{attr, attr::Attr, prelude::*};
use uuid::Uuid;

use super::use_form_validation_state::{
    UseFormValidationStateInput, ValidateFn, ValidationBehavior, ValidityStateSnapshot,
    use_form_validation_state,
};
use crate::{
    hooks::IntoAttrs,
    utils::aria::{AriaDisabled, AriaInvalid, AriaOrientation, AriaRequired, AriaRole},
};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/checkbox/src/useCheckboxGroup.ts

// No intentional deviations from the react-aria implementation.

/// Input parameters for the `use_checkbox_group` hook.
#[derive(Clone)]
pub struct UseCheckboxGroupInput<T>
where
    T: Hash + Eq + Clone + Send + Sync + 'static,
{
    /// The current selected values (controlled).
    pub value: Signal<HashSet<T>>,

    /// Callback when the selection changes.
    pub on_change: Option<Callback<HashSet<T>>>,

    /// Whether the group is disabled.
    pub is_disabled: Signal<bool>,

    /// Whether the group is read-only.
    pub is_read_only: Signal<bool>,

    /// Whether the group is required.
    pub is_required: bool,

    /// Whether the group is explicitly marked as invalid (controlled validation).
    ///
    /// - `None` — not controlled; validation comes from `validate`, server errors,
    ///   or native constraint validation.
    /// - `Some(signal)` — controlled; the signal value determines valid/invalid
    ///   and overrides all other validation sources.
    pub is_invalid: Option<Signal<bool>>,

    /// Custom client-side validation function.
    ///
    /// Returns `Ok(())` for valid, `Err(messages)` for invalid.
    pub validate: Option<ValidateFn<HashSet<T>>>,

    /// Validation behavior mode.
    pub validation_behavior: ValidationBehavior,

    /// The name attribute for form submission, used to match server errors.
    pub name: Option<&'static str>,

    /// The label for the group.
    pub label: Option<String>,

    /// A description for the group.
    pub description: Option<String>,

    /// The orientation of the group.
    pub orientation: Orientation,
}

impl<T> Default for UseCheckboxGroupInput<T>
where
    T: Hash + Eq + Clone + Send + Sync + 'static,
{
    fn default() -> Self {
        Self {
            value: Signal::derive(HashSet::new),
            on_change: None,
            is_disabled: Signal::derive(|| false),
            is_read_only: Signal::derive(|| false),
            is_required: false,
            is_invalid: None,
            validate: None,
            validation_behavior: ValidationBehavior::default(),
            name: None,
            label: None,
            description: None,
            orientation: Orientation::default(),
        }
    }
}

/// The orientation of a group.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Orientation {
    /// Horizontal layout.
    #[default]
    Horizontal,
    /// Vertical layout.
    Vertical,
}

impl From<Orientation> for AriaOrientation {
    fn from(value: Orientation) -> Self {
        match value {
            Orientation::Horizontal => Self::Horizontal,
            Orientation::Vertical => Self::Vertical,
        }
    }
}

/// The return value of the `use_checkbox_group` hook.
pub struct UseCheckboxGroupReturn<T>
where
    T: Hash + Eq + Clone + Send + Sync + 'static,
{
    /// Props for the group container element.
    pub group_props: UseCheckboxGroupProps,

    /// Props for the label element.
    pub label_props: UseCheckboxGroupLabelProps,

    /// The group state for use by individual checkboxes.
    pub state: UseCheckboxGroupState<T>,

    /// Whether the displayed validation is invalid.
    pub is_invalid: Signal<bool>,

    /// The displayed validation error messages.
    pub validation_errors: Signal<Vec<String>>,

    /// Detailed validity state (mirrors native `ValidityState`).
    pub validation_details: Signal<ValidityStateSnapshot>,
}

/// Props from `use_checkbox_group` for the checkbox group container.
#[derive(Debug)]
pub struct UseCheckboxGroupProps {
    /// The role attribute.
    pub role: AriaRole,

    /// The aria-labelledby attribute.
    pub aria_labelledby: Option<String>,

    /// The aria-describedby attribute.
    pub aria_describedby: Signal<Option<String>>,

    /// The aria-invalid attribute.
    pub aria_invalid: Signal<Option<AriaInvalid>>,

    /// The aria-required attribute.
    pub aria_required: Option<AriaRequired>,

    /// The aria-disabled attribute.
    pub aria_disabled: Signal<Option<AriaDisabled>>,

    /// The aria-orientation attribute.
    pub aria_orientation: AriaOrientation,
}

impl IntoAttrs for UseCheckboxGroupProps {
    type Attrs = UseCheckboxGroupAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Role, self.role),
            Attr(attr::AriaLabelledby, self.aria_labelledby),
            Attr(attr::AriaDescribedby, self.aria_describedby),
            Attr(attr::AriaInvalid, self.aria_invalid),
            Attr(attr::AriaRequired, self.aria_required),
            Attr(attr::AriaDisabled, self.aria_disabled),
            Attr(attr::AriaOrientation, self.aria_orientation),
        )
    }
}

/// Attributes for the checkbox group container.
/// Spread onto the group element using `<fieldset {..group_props.into_attrs()}>`.
pub type UseCheckboxGroupAttrs = (
    Attr<attr::Role, AriaRole>,
    Attr<attr::AriaLabelledby, Option<String>>,
    Attr<attr::AriaDescribedby, Signal<Option<String>>>,
    Attr<attr::AriaInvalid, Signal<Option<AriaInvalid>>>,
    Attr<attr::AriaRequired, Option<AriaRequired>>,
    Attr<attr::AriaDisabled, Signal<Option<AriaDisabled>>>,
    Attr<attr::AriaOrientation, AriaOrientation>,
);

/// Props for the group label.
#[derive(Debug)]
pub struct UseCheckboxGroupLabelProps {
    /// The id of the label element.
    pub id: String,
}

impl IntoAttrs for UseCheckboxGroupLabelProps {
    type Attrs = UseCheckboxGroupLabelAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (Attr(attr::Id, self.id),)
    }
}

/// Attributes for the group label element.
/// Spread onto the label element using `<legend {..label_props.into_attrs()}>`.
pub type UseCheckboxGroupLabelAttrs = (Attr<attr::Id, String>,);

/// State for a checkbox group.
#[derive(Clone, Copy)]
pub struct UseCheckboxGroupState<T>
where
    T: Hash + Eq + Clone + Send + Sync + 'static,
{
    /// The current selected values.
    pub value: Signal<HashSet<T>>,

    /// Whether the group is disabled.
    pub is_disabled: Signal<bool>,

    /// Whether the group is read-only.
    pub is_read_only: Signal<bool>,

    /// Check if a value is selected.
    pub is_selected: Callback<T, bool>,

    /// Add a value to the selection.
    pub add_value: Callback<T>,

    /// Remove a value from the selection.
    pub remove_value: Callback<T>,

    /// Toggle a value's selection.
    pub toggle_value: Callback<T>,
}

/// Provides the behavior and accessibility implementation for a checkbox group.
///
/// Checkbox groups allow users to select multiple items from a set.
///
/// # Example
///
/// ```ignore
/// let (selected, set_selected) = signal(HashSet::new());
///
/// let group = use_checkbox_group(UseCheckboxGroupInput {
///     value: selected.into(),
///     on_change: Some(Callback::new(move |values| {
///         set_selected.set(values);
///     })),
///     label: Some("Favorite fruits".to_string()),
///     ..Default::default()
/// });
///
/// view! {
///     <fieldset
///         role=group.group_props.role
///         aria-labelledby=group.group_props.aria_labelledby
///     >
///         <legend id=group.label_props.id>"Favorite fruits"</legend>
///         // Individual checkboxes here using group.state
///     </fieldset>
/// }
/// ```
#[allow(clippy::needless_pass_by_value, clippy::too_many_lines)]
pub fn use_checkbox_group<T>(input: UseCheckboxGroupInput<T>) -> UseCheckboxGroupReturn<T>
where
    T: Hash + Eq + Clone + Send + Sync + 'static,
{
    let UseCheckboxGroupInput {
        value,
        on_change,
        is_disabled,
        is_read_only,
        is_required,
        is_invalid,
        validate,
        validation_behavior,
        name,
        label,
        description,
        orientation,
    } = input;

    // ---- Form validation state ----
    let validation = use_form_validation_state(UseFormValidationStateInput {
        is_invalid,
        value,
        validate,
        validation_behavior,
        name: name.map(ToString::to_string),
    });

    // ---- IDs ----
    let base_id = Uuid::new_v4();
    let label_id = format!("checkbox-group-label-{base_id}");
    let description_id = format!("checkbox-group-description-{base_id}");
    let error_id = format!("checkbox-group-error-{base_id}");

    // ---- Reactive ARIA attributes ----
    let has_description = description.is_some();
    let has_label = label.is_some();

    let description_id_for_signal = description_id.clone();
    let error_id_for_signal = error_id.clone();
    let aria_describedby = Signal::derive(move || {
        let mut parts = Vec::new();
        if has_description {
            parts.push(description_id_for_signal.clone());
        }
        if validation.is_invalid.get() {
            parts.push(error_id_for_signal.clone());
        }
        if parts.is_empty() {
            None
        } else {
            Some(parts.join(" "))
        }
    });

    // Build aria-labelledby
    let aria_labelledby = if has_label {
        Some(label_id.clone())
    } else {
        None
    };

    let aria_invalid =
        Signal::derive(move || validation.is_invalid.get().then_some(AriaInvalid::True));

    // ---- Validation details convenience signal ----
    let validation_details =
        Signal::derive(move || validation.display_validation.get().validation_details);

    // Check if a value is selected
    let is_selected = Callback::new(move |v: T| -> bool { value.get_untracked().contains(&v) });

    // Add a value
    let add_value = Callback::new(move |v: T| {
        if is_disabled.get_untracked() || is_read_only.get_untracked() {
            return;
        }

        let mut new_set = value.get_untracked();
        new_set.insert(v);

        if let Some(on_change) = on_change {
            on_change.run(new_set);
        }
    });

    // Remove a value
    let remove_value = Callback::new(move |v: T| {
        if is_disabled.get_untracked() || is_read_only.get_untracked() {
            return;
        }

        let mut new_set = value.get_untracked();
        new_set.remove(&v);

        if let Some(on_change) = on_change {
            on_change.run(new_set);
        }
    });

    // Toggle a value
    let toggle_value = Callback::new(move |v: T| {
        if is_disabled.get_untracked() || is_read_only.get_untracked() {
            return;
        }

        let mut new_set = value.get_untracked();
        if new_set.contains(&v) {
            new_set.remove(&v);
        } else {
            new_set.insert(v);
        }

        if let Some(on_change) = on_change {
            on_change.run(new_set);
        }
    });

    UseCheckboxGroupReturn {
        group_props: UseCheckboxGroupProps {
            role: AriaRole::Group,
            aria_labelledby,
            aria_describedby,
            aria_invalid,
            aria_required: is_required.then_some(AriaRequired::True),
            aria_disabled: Signal::derive(move || is_disabled.get().then_some(AriaDisabled::True)),
            aria_orientation: AriaOrientation::from(orientation),
        },
        label_props: UseCheckboxGroupLabelProps { id: label_id },
        state: UseCheckboxGroupState {
            value,
            is_disabled,
            is_read_only,
            is_selected,
            add_value,
            remove_value,
            toggle_value,
        },
        is_invalid: validation.is_invalid,
        validation_errors: validation.validation_errors,
        validation_details,
    }
}
