use leptos::prelude::*;
use uuid::Uuid;

use super::use_checkbox_group::Orientation;
use super::use_field::ValidationState;

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/radio/src/useRadioGroup.ts

/// Input parameters for the `use_radio_group` hook.
#[derive(Clone)]
pub struct UseRadioGroupInput<T>
where
    T: Clone + Send + Sync + 'static,
{
    /// The current selected value (controlled).
    pub value: Signal<Option<T>>,

    /// Callback when the selection changes.
    pub on_change: Option<Callback<T>>,

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

impl<T: Clone + Send + Sync + 'static> Default for UseRadioGroupInput<T> {
    fn default() -> Self {
        Self {
            value: Signal::derive(|| None),
            on_change: None,
            is_disabled: Signal::derive(|| false),
            is_read_only: Signal::derive(|| false),
            is_required: false,
            validation_state: ValidationState::Valid,
            label: None,
            description: None,
            error_message: None,
            orientation: Orientation::Vertical,
        }
    }
}

/// The return value of the `use_radio_group` hook.
#[derive(Clone)]
pub struct UseRadioGroupReturn<T>
where
    T: Clone + Send + Sync + 'static,
{
    /// Props for the group container element.
    pub group_props: UseRadioGroupAttrs,

    /// Props for the label element.
    pub label_props: UseRadioGroupLabelProps,

    /// The group state for use by individual radio buttons.
    pub state: UseRadioGroupState<T>,
}

/// Attributes for the radio group container.
#[derive(Debug, Clone)]
pub struct UseRadioGroupAttrs {
    /// The role attribute.
    pub role: &'static str,

    /// The aria-labelledby attribute.
    pub aria_labelledby: Option<String>,

    /// The aria-describedby attribute.
    pub aria_describedby: Option<String>,

    /// The aria-invalid attribute.
    pub aria_invalid: Option<&'static str>,

    /// The aria-required attribute.
    pub aria_required: Option<&'static str>,

    /// The aria-disabled attribute.
    pub aria_disabled: Option<&'static str>,

    /// The aria-orientation attribute.
    pub aria_orientation: &'static str,
}

/// Props for the group label.
#[derive(Debug, Clone)]
pub struct UseRadioGroupLabelProps {
    /// The id of the label element.
    pub id: String,
}

/// State for a radio group.
#[derive(Clone, Copy)]
pub struct UseRadioGroupState<T>
where
    T: Clone + Send + Sync + 'static,
{
    /// The current selected value.
    pub selected_value: Signal<Option<T>>,

    /// Whether the group is disabled.
    pub is_disabled: Signal<bool>,

    /// Whether the group is read-only.
    pub is_read_only: Signal<bool>,

    /// Set the selected value.
    pub set_selected_value: Callback<T>,

    /// The name for the radio group (for form submission).
    pub name: &'static str,
}

/// Provides the behavior and accessibility implementation for a radio group.
///
/// Radio groups allow users to select a single option from a set.
///
/// # Example
///
/// ```ignore
/// let (selected, set_selected) = signal(None::<String>);
///
/// let group = use_radio_group(UseRadioGroupInput {
///     value: selected.into(),
///     on_change: Some(Callback::new(move |value| {
///         set_selected.set(Some(value));
///     })),
///     label: Some("Choose an option".to_string()),
///     ..Default::default()
/// });
///
/// view! {
///     <fieldset
///         role=group.group_props.role
///         aria-labelledby=group.group_props.aria_labelledby
///     >
///         <legend id=group.label_props.id>"Choose an option"</legend>
///         // Individual radio buttons here using group.state
///     </fieldset>
/// }
/// ```
#[allow(clippy::needless_pass_by_value)]
pub fn use_radio_group<T>(input: UseRadioGroupInput<T>) -> UseRadioGroupReturn<T>
where
    T: Clone + Send + Sync + 'static,
{
    let base_id = Uuid::new_v4();
    let label_id = format!("radio-group-label-{base_id}");
    let description_id = format!("radio-group-description-{base_id}");
    let error_id = format!("radio-group-error-{base_id}");
    let name: &'static str = Box::leak(format!("radio-group-{base_id}").into_boxed_str());

    let value = input.value;
    let on_change = input.on_change;
    let is_disabled = input.is_disabled;
    let is_read_only = input.is_read_only;

    // Build aria-describedby
    let mut describedby_parts = Vec::new();
    if input.description.is_some() {
        describedby_parts.push(description_id.clone());
    }
    if input.validation_state == ValidationState::Invalid && input.error_message.is_some() {
        describedby_parts.push(error_id.clone());
    }

    let aria_describedby = if describedby_parts.is_empty() {
        None
    } else {
        Some(describedby_parts.join(" "))
    };

    // Build aria-labelledby
    let aria_labelledby = if input.label.is_some() {
        Some(label_id.clone())
    } else {
        None
    };

    // Set selected value callback
    let set_selected_value = Callback::new(move |v: T| {
        if is_disabled.get_untracked() || is_read_only.get_untracked() {
            return;
        }

        if let Some(on_change) = on_change {
            on_change.run(v);
        }
    });

    UseRadioGroupReturn {
        group_props: UseRadioGroupAttrs {
            role: "radiogroup",
            aria_labelledby,
            aria_describedby,
            aria_invalid: if input.validation_state == ValidationState::Invalid {
                Some("true")
            } else {
                None
            },
            aria_required: if input.is_required {
                Some("true")
            } else {
                None
            },
            aria_disabled: if is_disabled.get_untracked() {
                Some("true")
            } else {
                None
            },
            aria_orientation: match input.orientation {
                Orientation::Horizontal => "horizontal",
                Orientation::Vertical => "vertical",
            },
        },
        label_props: UseRadioGroupLabelProps { id: label_id },
        state: UseRadioGroupState {
            selected_value: value,
            is_disabled,
            is_read_only,
            set_selected_value,
            name,
        },
    }
}

/// Creates internal state for a radio group component.
pub fn use_radio_group_state<T>(default_value: Option<T>) -> UseRadioGroupStateReturn<T>
where
    T: Clone + Send + Sync + 'static,
{
    let (selected, set_selected) = signal(default_value);

    UseRadioGroupStateReturn {
        selected_value: selected.into(),
        set_selected: Callback::new(move |value| {
            set_selected.set(Some(value));
        }),
    }
}

/// State for managing radio group state.
#[derive(Clone, Copy)]
pub struct UseRadioGroupStateReturn<T>
where
    T: Clone + Send + Sync + 'static,
{
    /// The current selected value.
    pub selected_value: Signal<Option<T>>,

    /// Set the selected value.
    pub set_selected: Callback<T>,
}
