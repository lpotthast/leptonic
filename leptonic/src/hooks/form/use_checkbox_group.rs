use leptos::attr;
use leptos::attr::Attr;
use leptos::prelude::*;
use std::collections::HashSet;
use std::hash::Hash;
use uuid::Uuid;

use crate::utils::aria::{AriaDisabled, AriaInvalid, AriaOrientation, AriaRequired};
use crate::hooks::IntoAttrs;

use super::use_field::ValidationState;

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/checkbox/src/useCheckboxGroup.ts

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// No intentional deviations from the react-aria implementation.
//
// =============================================================================

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

    /// The validation state of the group.
    pub validation_state: ValidationState,

    /// The label for the group.
    pub label: Option<String>,

    /// A description for the group.
    pub description: Option<String>,

    /// An error message for the group.
    pub error_message: Option<String>,

    /// The orientation of the group.
    pub orientation: Orientation,
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
}

/// Props from `use_checkbox_group` for the checkbox group container.
#[derive(Debug)]
pub struct UseCheckboxGroupProps {
    /// The role attribute.
    pub role: &'static str,

    /// The aria-labelledby attribute.
    pub aria_labelledby: Option<String>,

    /// The aria-describedby attribute.
    pub aria_describedby: Option<String>,

    /// The aria-invalid attribute.
    pub aria_invalid: Option<AriaInvalid>,

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
    Attr<attr::Role, &'static str>,
    Attr<attr::AriaLabelledby, Option<String>>,
    Attr<attr::AriaDescribedby, Option<String>>,
    Attr<attr::AriaInvalid, Option<AriaInvalid>>,
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
#[allow(clippy::needless_pass_by_value)]
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
        validation_state,
        label,
        description,
        error_message,
        orientation,
    } = input;

    let base_id = Uuid::new_v4();
    let label_id = format!("checkbox-group-label-{base_id}");
    let description_id = format!("checkbox-group-description-{base_id}");
    let error_id = format!("checkbox-group-error-{base_id}");

    // Build aria-describedby
    let mut describedby_parts = Vec::new();
    if description.is_some() {
        describedby_parts.push(description_id.clone());
    }
    if validation_state == ValidationState::Invalid && error_message.is_some() {
        describedby_parts.push(error_id.clone());
    }

    let aria_describedby = if describedby_parts.is_empty() {
        None
    } else {
        Some(describedby_parts.join(" "))
    };

    // Build aria-labelledby
    let aria_labelledby = if label.is_some() {
        Some(label_id.clone())
    } else {
        None
    };

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
            role: "group",
            aria_labelledby,
            aria_describedby,
            aria_invalid: (validation_state == ValidationState::Invalid)
                .then_some(AriaInvalid::True),
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
    }
}
