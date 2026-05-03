use leptos::{
    attr,
    attr::Attr,
    ev,
    ev::{On, SharedEventCallback},
    prelude::*,
};
use uuid::Uuid;
use web_sys::KeyboardEvent;

use super::{
    use_date_field_state::{UseDateFieldStateInput, UseDateFieldStateReturn, use_date_field_state},
    use_date_segment::{DateSegment, DateSegmentType},
};
use crate::{
    hooks::{
        IntoAttrs,
        form::use_form_validation_state::{ValidateFn, ValidationBehavior},
    },
    utils::{
        EventHandler,
        aria::{AriaDisabled, AriaInvalid, AriaRequired, AriaRole},
    },
};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/datepicker/src/useDateField.ts

//
// DIFFERENT BEHAVIOR
// - Hook-owned state: Uses `use_date_field_state` which owns the editing buffer
//   internally. Callers get read-only signals and semantic mutation callbacks.
// - Segment mutation callbacks take `DateSegmentType` instead of segment index,
//   because the state hook operates on field types, not indices.
//
// LEPTOS-SPECIFIC ADAPTATIONS
// - aria-describedby is a reactive `Signal<Option<String>>` that dynamically
//   includes/excludes the error ID based on validation state (following
//   `use_text_field` pattern).
//

/// Input parameters for the `use_date_field` hook.
#[allow(clippy::struct_excessive_bools)]
#[derive(Clone)]
pub struct UseDateFieldInput {
    /// The current date value.
    pub value: Signal<Option<time::OffsetDateTime>>,

    /// The default value to restore on form reset.
    pub default_value: Option<time::OffsetDateTime>,

    /// The minimum allowed date.
    pub min: Option<time::OffsetDateTime>,

    /// The maximum allowed date.
    pub max: Option<time::OffsetDateTime>,

    /// Whether the field is disabled.
    pub is_disabled: Signal<bool>,

    /// Whether the field is read-only.
    pub is_read_only: Signal<bool>,

    /// Whether the field is required.
    pub is_required: bool,

    /// Whether the field is explicitly marked as invalid (controlled validation).
    pub is_invalid: Option<Signal<bool>>,

    /// Custom client-side validation function.
    pub validate: Option<ValidateFn<Option<time::OffsetDateTime>>>,

    /// Validation behavior mode.
    pub validation_behavior: ValidationBehavior,

    /// The label for the field.
    pub label: Option<String>,

    /// The description for the field.
    pub description: Option<String>,

    /// Error message to display.
    pub error_message: Option<String>,

    /// Callback when the value changes.
    pub on_change: Option<Callback<Option<time::OffsetDateTime>>>,

    /// Whether to show the time portion.
    pub show_time: bool,

    /// Whether to use 24-hour format.
    pub hour_cycle_24: bool,

    /// Whether this field is inside a date picker (changes role to Presentation).
    pub is_date_picker: bool,

    /// The field's name for form validation context matching.
    pub name: Option<String>,
}

impl Default for UseDateFieldInput {
    fn default() -> Self {
        Self {
            value: Signal::derive(|| None),
            default_value: None,
            min: None,
            max: None,
            is_disabled: Signal::derive(|| false),
            is_read_only: Signal::derive(|| false),
            is_required: false,
            is_invalid: None,
            validate: None,
            validation_behavior: ValidationBehavior::default(),
            label: None,
            description: None,
            error_message: None,
            on_change: None,
            show_time: false,
            hour_cycle_24: true,
            is_date_picker: false,
            name: None,
        }
    }
}

/// The return value of the `use_date_field` hook.
pub struct UseDateFieldReturn {
    /// Props for the field container element. Call `.into_attrs()` for view spreading.
    pub field_props: UseDateFieldProps,

    /// Props for the label element.
    pub label_props: UseDateFieldLabelProps,

    /// Props for the description element.
    pub description_props: UseDateFieldDescriptionProps,

    /// Props for the error message element.
    pub error_props: UseDateFieldErrorProps,

    /// The segments to render.
    pub segments: Signal<Vec<DateSegment>>,

    /// The currently focused segment index.
    pub focused_segment: Signal<Option<usize>>,

    /// The ID of the field.
    pub field_id: String,

    /// Focus a specific segment.
    pub focus_segment: Callback<usize>,

    /// Focus the next segment.
    pub focus_next: Callback<()>,

    /// Focus the previous segment.
    pub focus_previous: Callback<()>,

    /// Increment a segment by type.
    pub increment: Callback<DateSegmentType>,

    /// Decrement a segment by type.
    pub decrement: Callback<DateSegmentType>,

    /// Set a segment value by type.
    pub set_segment: Callback<(DateSegmentType, i32)>,

    /// Clear a segment (Backspace/Delete).
    pub clear_segment: Callback<DateSegmentType>,

    /// Increment by page step.
    pub increment_page: Callback<DateSegmentType>,

    /// Decrement by page step.
    pub decrement_page: Callback<DateSegmentType>,

    /// Set segment to max value.
    pub increment_to_max: Callback<DateSegmentType>,

    /// Set segment to min value.
    pub decrement_to_min: Callback<DateSegmentType>,

    /// Confirm placeholder on blur.
    pub confirm_placeholder: Callback<()>,

    /// Whether the displayed validation is invalid.
    pub is_invalid: Signal<bool>,

    /// The displayed validation error messages.
    pub validation_errors: Signal<Vec<String>>,

    /// The underlying state hook return, for advanced usage.
    pub state: UseDateFieldStateReturn,
}

/// Props from `use_date_field` for the field container element.
#[derive(Debug)]
pub struct UseDateFieldProps {
    pub id: String,
    pub role: AriaRole,
    pub aria_labelledby: Option<String>,
    pub aria_describedby: Signal<Option<String>>,
    pub aria_disabled: Signal<Option<AriaDisabled>>,
    pub aria_invalid: Signal<Option<AriaInvalid>>,
    pub aria_required: Option<AriaRequired>,
    pub on_keydown: EventHandler<KeyboardEvent>,
}

impl IntoAttrs for UseDateFieldProps {
    type Attrs = UseDateFieldAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Id, self.id),
            Attr(attr::Role, self.role),
            Attr(attr::AriaLabelledby, self.aria_labelledby),
            Attr(attr::AriaDescribedby, self.aria_describedby),
            Attr(attr::AriaDisabled, self.aria_disabled),
            Attr(attr::AriaInvalid, self.aria_invalid),
            Attr(attr::AriaRequired, self.aria_required),
            self.on_keydown.into_on(ev::keydown),
        )
    }
}

/// Attributes for the date field container element.
pub type UseDateFieldAttrs = (
    Attr<attr::Id, String>,
    Attr<attr::Role, AriaRole>,
    Attr<attr::AriaLabelledby, Option<String>>,
    Attr<attr::AriaDescribedby, Signal<Option<String>>>,
    Attr<attr::AriaDisabled, Signal<Option<AriaDisabled>>>,
    Attr<attr::AriaInvalid, Signal<Option<AriaInvalid>>>,
    Attr<attr::AriaRequired, Option<AriaRequired>>,
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
);

/// Props for the label element.
#[derive(Debug)]
pub struct UseDateFieldLabelProps {
    /// The id of the label element.
    pub id: String,
}

/// Props for the description element.
#[derive(Debug)]
pub struct UseDateFieldDescriptionProps {
    /// The id of the description element.
    pub id: String,
}

/// Props for the error message element.
#[derive(Debug)]
pub struct UseDateFieldErrorProps {
    /// The id of the error message element.
    pub id: String,
    /// The role for the error message.
    pub role: AriaRole,
    /// The aria-live attribute.
    pub aria_live: &'static str,
}

/// Provides the behavior and accessibility for a date field.
///
/// A date field allows users to enter a date using editable segments.
/// Delegates state management to [`use_date_field_state`] and adds ARIA
/// attributes, keyboard navigation, and form validation integration.
///
/// # Example
///
/// ```ignore
/// let (value, set_value) = signal(None);
///
/// let field = use_date_field(UseDateFieldInput {
///     value: value.into(),
///     label: Some("Date".to_string()),
///     on_change: Some(Callback::new(move |v| set_value.set(v))),
///     ..Default::default()
/// });
///
/// view! {
///     <div>
///         <label id=field.label_props.id>"Date"</label>
///         <div {..field.field_props.into_attrs()}>
///             // Render segments...
///         </div>
///     </div>
/// }
/// ```
#[allow(clippy::too_many_lines, clippy::needless_pass_by_value)]
pub fn use_date_field(input: UseDateFieldInput) -> UseDateFieldReturn {
    let UseDateFieldInput {
        value,
        default_value,
        min,
        max,
        is_disabled: disabled,
        is_read_only,
        is_required,
        is_invalid,
        validate,
        validation_behavior,
        label,
        description,
        error_message,
        on_change,
        show_time,
        hour_cycle_24,
        is_date_picker,
        name,
    } = input;

    // ---- State hook ----
    let state = use_date_field_state(UseDateFieldStateInput {
        value,
        default_value,
        min,
        max,
        on_change,
        show_time,
        hour_cycle_24,
        is_disabled: disabled,
        is_read_only,
        is_required,
        validate,
        is_invalid,
        validation_behavior,
        name,
    });

    // ---- IDs ----
    let base_id = Uuid::new_v4();
    let field_id = format!("date-field-{base_id}");
    let label_id = format!("date-field-label-{base_id}");
    let description_id = format!("date-field-desc-{base_id}");
    let error_id = format!("date-field-error-{base_id}");

    // ---- Track focused segment ----
    let (focused_segment, set_focused_segment) = signal::<Option<usize>>(None);

    let segments = state.segments;

    // Get editable segment indices.
    let editable_indices = move || {
        segments.with(|segs| {
            segs.iter()
                .enumerate()
                .filter(|(_, s)| s.is_editable)
                .map(|(i, _)| i)
                .collect::<Vec<_>>()
        })
    };

    // ---- Focus callbacks ----
    let focus_segment = Callback::new(move |index: usize| {
        set_focused_segment.set(Some(index));
    });

    let focus_next = Callback::new(move |_| {
        let indices = editable_indices();
        let current = focused_segment.get_untracked();

        if let Some(curr) = current {
            let next = indices.iter().find(|&&i| i > curr).copied();
            if let Some(next_idx) = next {
                set_focused_segment.set(Some(next_idx));
            }
        } else if let Some(&first) = indices.first() {
            set_focused_segment.set(Some(first));
        }
    });

    let focus_previous = Callback::new(move |_| {
        let indices = editable_indices();
        let current = focused_segment.get_untracked();

        if let Some(curr) = current {
            let prev = indices.iter().rev().find(|&&i| i < curr).copied();
            if let Some(prev_idx) = prev {
                set_focused_segment.set(Some(prev_idx));
            }
        } else if let Some(&last) = indices.last() {
            set_focused_segment.set(Some(last));
        }
    });

    // ---- Reactive ARIA attributes ----
    let has_description = description.is_some();
    let has_error = error_message.is_some();
    let has_label = label.is_some();
    let validation = state.validation;

    let description_id_for_signal = description_id.clone();
    let error_id_for_signal = error_id.clone();
    let aria_describedby = Signal::derive(move || {
        let mut parts = Vec::new();
        if has_description {
            parts.push(description_id_for_signal.clone());
        }
        if has_error || validation.is_invalid.get() {
            parts.push(error_id_for_signal.clone());
        }
        if parts.is_empty() {
            None
        } else {
            Some(parts.join(" "))
        }
    });

    let aria_labelledby = if has_label {
        Some(label_id.clone())
    } else {
        None
    };

    let aria_disabled = Signal::derive(move || disabled.get().then_some(AriaDisabled::True));
    let aria_invalid =
        Signal::derive(move || validation.is_invalid.get().then_some(AriaInvalid::True));
    let aria_required = is_required.then_some(AriaRequired::True);

    // Role depends on whether this is inside a date picker.
    let role = if is_date_picker {
        AriaRole::Presentation
    } else {
        AriaRole::Group
    };

    // ---- Keyboard handler (field-level, for navigation) ----
    let handle_keydown = move |e: KeyboardEvent| {
        if disabled.get_untracked() || is_read_only.get_untracked() {
            return;
        }

        let key = e.key();
        match key.as_str() {
            "ArrowRight" | "Tab" if !e.shift_key() => {
                // Navigation handled by individual segments.
            }
            "ArrowLeft" | "Tab" if e.shift_key() => {
                // Navigation handled by individual segments.
            }
            _ => {}
        }
    };

    UseDateFieldReturn {
        field_props: UseDateFieldProps {
            id: field_id.clone(),
            role,
            aria_labelledby,
            aria_describedby,
            aria_disabled,
            aria_invalid,
            aria_required,
            on_keydown: EventHandler::new(handle_keydown),
        },
        label_props: UseDateFieldLabelProps { id: label_id },
        description_props: UseDateFieldDescriptionProps { id: description_id },
        error_props: UseDateFieldErrorProps {
            id: error_id,
            role: AriaRole::Alert,
            aria_live: "polite",
        },
        segments,
        focused_segment: focused_segment.into(),
        field_id,
        focus_segment,
        focus_next,
        focus_previous,
        increment: state.increment,
        decrement: state.decrement,
        set_segment: state.set_segment,
        clear_segment: state.clear_segment,
        increment_page: state.increment_page,
        decrement_page: state.decrement_page,
        increment_to_max: state.increment_to_max,
        decrement_to_min: state.decrement_to_min,
        confirm_placeholder: state.confirm_placeholder,
        is_invalid: validation.is_invalid,
        validation_errors: validation.validation_errors,
        state,
    }
}
