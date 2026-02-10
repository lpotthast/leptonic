use leptos::attr;
use leptos::attr::{Attr, Attribute};
use leptos::ev;
use leptos::ev::{on, On, SharedEventCallback};
use leptos::prelude::*;
use uuid::Uuid;
use web_sys::{KeyboardEvent, MouseEvent};

use crate::utils::aria::{AriaDisabled, AriaExpanded, AriaModal};
// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/datepicker/src/useDatePicker.ts

/// Input parameters for the `use_date_picker` hook.
#[derive(Debug, Clone)]
pub struct UseDatePickerInput {
    /// The current date value.
    pub value: Signal<Option<time::OffsetDateTime>>,

    /// The minimum allowed date.
    pub min: Option<time::OffsetDateTime>,

    /// The maximum allowed date.
    pub max: Option<time::OffsetDateTime>,

    /// Whether the picker is open.
    pub is_open: Option<Signal<bool>>,

    /// Whether the picker is disabled.
    pub is_disabled: Signal<bool>,

    /// Whether the picker is read-only.
    pub is_read_only: Signal<bool>,

    /// Whether the field is required.
    pub is_required: bool,

    /// The label for the field.
    pub label: Option<String>,

    /// The description for the field.
    pub description: Option<String>,

    /// Error message to display.
    pub error_message: Option<String>,

    /// Callback when the value changes.
    pub on_change: Option<Callback<Option<time::OffsetDateTime>>>,

    /// Callback when the open state changes.
    pub on_open_change: Option<Callback<bool>>,
}

impl Default for UseDatePickerInput {
    fn default() -> Self {
        Self {
            value: Signal::derive(|| None),
            min: None,
            max: None,
            is_open: None,
            is_disabled: Signal::derive(|| false),
            is_read_only: Signal::derive(|| false),
            is_required: false,
            label: None,
            description: None,
            error_message: None,
            on_change: None,
            on_open_change: None,
        }
    }
}

/// The return value of the `use_date_picker` hook.
#[derive(Debug, Clone)]
pub struct UseDatePickerReturn {
    /// Props for the date picker group container.
    pub group_props: UseDatePickerGroupAttrs,

    /// Props for the label element.
    pub label_props: UseDatePickerLabelProps,

    /// Props for the field container.
    pub field_props: UseDatePickerFieldProps,

    /// Props for the calendar trigger button.
    pub button_props: UseDatePickerButtonAttrs,

    /// Props for the calendar dialog/popover.
    pub dialog_props: UseDatePickerDialogProps,

    /// Props for the calendar component.
    pub calendar_props: UseDatePickerCalendarProps,

    /// Whether the picker is open.
    pub is_open: Signal<bool>,

    /// The ID of the picker.
    pub picker_id: String,

    /// Open the calendar.
    pub open: Callback<()>,

    /// Close the calendar.
    pub close: Callback<()>,

    /// Toggle the calendar.
    pub toggle: Callback<()>,
}

/// Attributes for the date picker group container.
pub type UseDatePickerGroupAttrs = (
    Attr<attr::Id, String>,
    Attr<attr::Role, &'static str>,
    Attr<attr::AriaLabelledby, Option<String>>,
    Attr<attr::AriaDescribedby, Option<String>>,
    Attr<attr::AriaDisabled, Signal<Option<AriaDisabled>>>,
);

/// Props for the label element.
// TODO: Switch to attr defining tuple.
#[derive(Debug, Clone)]
pub struct UseDatePickerLabelProps {
    /// The id of the label element.
    pub id: String,
}

/// Props for the field container.
#[derive(Debug, Clone)]
pub struct UseDatePickerFieldProps {
    /// The id of the field element.
    pub id: String,
    /// The role attribute.
    pub role: &'static str,
    /// aria-haspopup attribute.
    pub aria_haspopup: &'static str,
    /// aria-expanded attribute.
    pub aria_expanded: Signal<Option<AriaExpanded>>,
}

/// Attributes for the calendar trigger button.
pub type UseDatePickerButtonAttrs = (
    Attr<attr::Id, String>,
    Attr<attr::AriaLabel, &'static str>,
    Attr<attr::AriaHaspopup, &'static str>,
    Attr<attr::AriaExpanded, Signal<Option<AriaExpanded>>>,
    Attr<attr::AriaDisabled, Signal<Option<AriaDisabled>>>,
    Attr<attr::Tabindex, &'static str>,
    On<ev::click, SharedEventCallback<MouseEvent>>,
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
);

/// Props for the calendar dialog/popover.
// TODO: Switch to attr defining tuple.
#[derive(Debug, Clone)]
pub struct UseDatePickerDialogProps {
    /// The id of the dialog element.
    pub id: String,
    /// The role attribute.
    pub role: &'static str,
    /// aria-modal attribute.
    pub aria_modal: AriaModal,
    /// aria-labelledby attribute.
    pub aria_labelledby: String,
}

/// Props for the calendar component.
#[derive(Debug, Clone)]
pub struct UseDatePickerCalendarProps {
    /// The id of the calendar element.
    pub id: String,
}

/// Provides the behavior and accessibility for a date picker.
///
/// A date picker combines a date field with a calendar popover.
///
/// # Example
///
/// ```ignore
/// let (value, set_value) = signal(None);
///
/// let picker = use_date_picker(UseDatePickerInput {
///     value: value.into(),
///     label: Some("Select Date".to_string()),
///     on_change: Some(Callback::new(move |v| set_value.set(v))),
///     ..Default::default()
/// });
///
/// view! {
///     <div {..picker.group_props}>
///         <label id=picker.label_props.id>"Select Date"</label>
///         <div id=picker.field_props.id role=picker.field_props.role>
///             // Date field segments...
///         </div>
///         <button {..picker.button_props}>"📅"</button>
///         <Show when=move || picker.is_open.get()>
///             <div id=picker.dialog_props.id role=picker.dialog_props.role>
///                 // Calendar component...
///             </div>
///         </Show>
///     </div>
/// }
/// ```
#[allow(clippy::too_many_lines, clippy::needless_pass_by_value)]
pub fn use_date_picker(input: UseDatePickerInput) -> UseDatePickerReturn {
    let UseDatePickerInput {
        value,
        min,
        max,
        is_open,
        is_disabled: disabled,
        is_read_only,
        is_required,
        label,
        description,
        error_message,
        on_change,
        on_open_change,
    } = input;

    let base_id = Uuid::new_v4();
    let picker_id = format!("date-picker-{base_id}");
    let label_id = format!("date-picker-label-{base_id}");
    let field_id = format!("date-picker-field-{base_id}");
    let button_id = format!("date-picker-button-{base_id}");
    let dialog_id = format!("date-picker-dialog-{base_id}");
    let calendar_id = format!("date-picker-calendar-{base_id}");

    // Internal open state
    let (internal_open, set_internal_open) = signal(false);
    let is_open = is_open.unwrap_or_else(|| internal_open.into());

    // Helper to update open state
    let update_open = move |new_open: bool| {
        set_internal_open.set(new_open);
        if let Some(on_change) = on_open_change {
            on_change.run(new_open);
        }
    };

    // Open/close/toggle callbacks
    let open = Callback::new(move |_| {
        if !disabled.get_untracked() && !is_read_only.get_untracked() {
            update_open(true);
        }
    });

    let close = Callback::new(move |_| {
        update_open(false);
    });

    let toggle = Callback::new(move |_| {
        if !disabled.get_untracked() && !is_read_only.get_untracked() {
            update_open(!is_open.get_untracked());
        }
    });

    // Build aria-labelledby
    let aria_labelledby = if label.is_some() {
        Some(label_id.clone())
    } else {
        None
    };

    // Build aria-describedby
    let aria_describedby = if description.is_some() {
        Some(format!("date-picker-desc-{base_id}"))
    } else {
        None
    };

    // Compute aria-disabled
    let aria_disabled = Signal::derive(move || disabled.get().then_some(AriaDisabled::True));

    // Compute aria-expanded
    let aria_expanded = Signal::derive(move || Some(AriaExpanded::from(is_open.get())));

    // Handle button click
    let handle_click = move |_e: MouseEvent| {
        if disabled.get_untracked() || is_read_only.get_untracked() {
            return;
        }
        update_open(!is_open.get_untracked());
    };

    // Handle button keyboard
    let handle_keydown = move |e: KeyboardEvent| {
        if disabled.get_untracked() || is_read_only.get_untracked() {
            return;
        }

        let key = e.key();
        match key.as_str() {
            "Enter" | " " | "ArrowDown" | "ArrowUp" => {
                e.prevent_default();
                update_open(true);
            }
            "Escape" => {
                if is_open.get_untracked() {
                    e.prevent_default();
                    update_open(false);
                }
            }
            _ => {}
        }
    };

    UseDatePickerReturn {
        group_props: (
            Attr(attr::Id, picker_id.clone()),
            Attr(attr::Role, "group"),
            Attr(attr::AriaLabelledby, aria_labelledby),
            Attr(attr::AriaDescribedby, aria_describedby),
            Attr(attr::AriaDisabled, aria_disabled),
        ),
        label_props: UseDatePickerLabelProps {
            id: label_id.clone(),
        },
        field_props: UseDatePickerFieldProps {
            id: field_id,
            role: "presentation",
            aria_haspopup: "dialog",
            aria_expanded,
        },
        button_props: (
            Attr(attr::Id, button_id),
            Attr(attr::AriaLabel, "Open calendar"),
            Attr(attr::AriaHaspopup, "dialog"),
            Attr(attr::AriaExpanded, aria_expanded),
            Attr(attr::AriaDisabled, aria_disabled),
            Attr(attr::Tabindex, "0"),
            on(ev::click, handle_click).into_cloneable(),
            on(ev::keydown, handle_keydown).into_cloneable(),
        ),
        dialog_props: UseDatePickerDialogProps {
            id: dialog_id,
            role: "dialog",
            aria_modal: AriaModal::True,
            aria_labelledby: label_id,
        },
        calendar_props: UseDatePickerCalendarProps { id: calendar_id },
        is_open,
        picker_id,
        open,
        close,
        toggle,
    }
}

/// State for managing date picker.
#[derive(Clone, Copy)]
pub struct UseDatePickerStateReturn {
    /// The current value.
    pub value: Signal<Option<time::OffsetDateTime>>,

    /// Set the value.
    pub set_value: Callback<Option<time::OffsetDateTime>>,

    /// Whether the picker is open.
    pub is_open: Signal<bool>,

    /// Open the picker.
    pub open: Callback<()>,

    /// Close the picker.
    pub close: Callback<()>,

    /// Clear the value.
    pub clear: Callback<()>,
}

/// Creates internal state for a date picker.
pub fn use_date_picker_state(
    default_value: Option<time::OffsetDateTime>,
) -> UseDatePickerStateReturn {
    let (value, set_value_signal) = signal(default_value);
    let (is_open, set_is_open) = signal(false);

    UseDatePickerStateReturn {
        value: value.into(),
        set_value: Callback::new(move |v| set_value_signal.set(v)),
        is_open: is_open.into(),
        open: Callback::new(move |_| set_is_open.set(true)),
        close: Callback::new(move |_| set_is_open.set(false)),
        clear: Callback::new(move |_| set_value_signal.set(None)),
    }
}
