use leptos::{
    attr,
    attr::Attr,
    ev,
    ev::{On, SharedEventCallback},
    prelude::*,
};
use leptos_use::use_document;
use uuid::Uuid;
use wasm_bindgen::JsCast;
use web_sys::{KeyboardEvent, MouseEvent};

use super::use_date_picker_state::{
    UseDatePickerStateInput, UseDatePickerStateReturn, use_date_picker_state,
};
use crate::{
    hooks::{
        IntoAttrs,
        form::use_form_validation_state::{ValidateFn, ValidationBehavior},
    },
    utils::{
        EventHandler,
        aria::{AriaDisabled, AriaExpanded, AriaModal, AriaRole},
        element_capture::{CapturedElement, ElementCaptureAttr},
    },
};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/datepicker/src/useDatePicker.ts

//
// ## OMITTED FEATURES
// - `focusWithin` tracking: `on_focus`/`on_blur`/`on_focus_change` props are
//   not supported. React-aria: uses `useFocusWithin` (disabled when popover
//   is open to avoid spurious blur events).
// - RTL layout support for segment navigation: Not implemented.
//   React-aria: supports RTL via locale-aware arrow key navigation in
//   `useDatePickerGroup`.
// - `useField` integration: Label, description, and error message IDs are
//   managed inline. React-aria: uses `useField` for auto-generated IDs.
// - Localized strings: `aria-label` on the button is English-only ("Calendar").
//   React-aria: uses `useLocalizedStringFormatter`.
//
// ## DIFFERENT BEHAVIOR
// - Accessibility description: A hidden DOM element is created via `Effect` +
//   `on_cleanup` for the "Selected date: ..." description. React-aria: uses
//   `useDescription` which follows the same pattern internally.
// - Button trigger: Uses `on_click` + `on_keydown` instead of `onPress` from
//   react-aria's interaction system. Both only open the popover (not toggle).
// - Calendar `on_change` adapter: Converts `OffsetDateTime` from the calendar
//   to `Date` for the staged commit system, since our calendar uses
//   `OffsetDateTime` while React-aria's uses `CalendarDate`.
// - Value description not on field: The value description (`aria-describedby`)
//   is on the group and button, not the field container. React-aria includes it
//   on the field too via `mergeProps`. In Leptos, the field generates its own
//   `aria-describedby` via `use_date_field`.
//
// ## LEPTOS-SPECIFIC ADAPTATIONS
// - Field and calendar data pass-through: Data for the inner date field and
//   calendar is available on `state` (returned in `UseDatePickerReturn`).
//   Consumers configure `use_date_field` and calendar from `state` fields.
//   React-aria: passes props directly via JSX.
//

/// Input parameters for the `use_date_picker` hook.
#[allow(clippy::struct_excessive_bools)]
#[derive(Clone)]
pub struct UseDatePickerInput {
    /// The initial date value.
    pub default_value: Option<time::OffsetDateTime>,

    /// The minimum allowed date.
    pub min: Option<time::OffsetDateTime>,

    /// The maximum allowed date.
    pub max: Option<time::OffsetDateTime>,

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

    /// Whether the picker includes a time portion.
    pub show_time: bool,

    /// Whether to use 24-hour format.
    pub hour_cycle_24: bool,

    /// Whether to auto-close the popover when a date is selected.
    pub should_close_on_select: bool,

    /// Callback to check if a specific date is unavailable.
    pub is_date_unavailable: Option<Callback<time::Date, bool>>,

    /// Whether the field is explicitly invalid (controlled).
    pub is_invalid: Option<Signal<bool>>,

    /// Custom validation function.
    pub validate: Option<ValidateFn<Option<time::OffsetDateTime>>>,

    /// Validation behavior mode.
    pub validation_behavior: ValidationBehavior,

    /// The field's name for form validation context matching.
    pub name: Option<String>,
}

impl Default for UseDatePickerInput {
    fn default() -> Self {
        Self {
            default_value: None,
            min: None,
            max: None,
            is_disabled: Signal::derive(|| false),
            is_read_only: Signal::derive(|| false),
            is_required: false,
            label: None,
            description: None,
            error_message: None,
            on_change: None,
            on_open_change: None,
            show_time: false,
            hour_cycle_24: true,
            should_close_on_select: true,
            is_date_unavailable: None,
            is_invalid: None,
            validate: None,
            validation_behavior: ValidationBehavior::default(),
            name: None,
        }
    }
}

/// The return value of the `use_date_picker` hook.
pub struct UseDatePickerReturn {
    /// Props for the date picker group container. Call `.into_attrs()` for view spreading.
    pub group_props: UseDatePickerGroupProps,

    /// Props for the label element. Call `.into_attrs()` for view spreading.
    /// Includes an `on_click` handler that focuses the first segment.
    pub label_props: UseDatePickerLabelProps,

    /// Props for the field container. Call `.into_attrs()` for view spreading.
    /// These are the picker-level ARIA additions. When using with `use_date_field`,
    /// spread the date field's attrs first, then these (to override `role`, etc.).
    pub field_props: UseDatePickerFieldProps,

    /// Props for the calendar trigger button. Call `.into_attrs()` for view spreading.
    pub button_props: UseDatePickerButtonProps,

    /// Props for the description element.
    pub description_props: UseDatePickerDescriptionProps,

    /// Props for the error message element.
    pub error_props: UseDatePickerErrorProps,

    /// Props for the calendar dialog/popover.
    pub dialog_props: UseDatePickerDialogProps,

    /// Configuration for the calendar inside the popover (data, not ARIA attributes).
    pub calendar_props: UseDatePickerCalendarProps,

    /// The date picker state. Use this to configure the inner date field and
    /// calendar, and to read/write the value, open state, etc.
    pub state: UseDatePickerStateReturn,

    /// Whether the picker is open. Shorthand for `state.is_open`.
    pub is_open: Signal<bool>,

    /// The current committed value. Shorthand for `state.value`.
    pub value: Signal<Option<time::OffsetDateTime>>,

    /// Open the picker. Shorthand for `state.open`.
    pub open: Callback<()>,

    /// Close the picker. Shorthand for `state.close`.
    pub close: Callback<()>,

    /// Set the open state. Shorthand for `state.set_open`.
    pub set_open: Callback<bool>,

    /// Whether the value is invalid. Shorthand for `state.is_invalid`.
    pub is_invalid: Signal<bool>,

    /// Validation error messages. Shorthand for `state.validation_errors`.
    pub validation_errors: Signal<Vec<String>>,

    /// The accessible description of the selected value
    /// (e.g., "Selected date: March 15, 2024").
    /// A hidden DOM element with this text is created automatically.
    pub value_description: Signal<String>,

    /// The ID of the hidden value description element.
    pub value_description_id: String,

    /// The generated picker ID.
    pub picker_id: String,
}

/// Props from `use_date_picker` for the group container.
#[derive(Debug)]
pub struct UseDatePickerGroupProps {
    pub id: String,
    pub role: AriaRole,
    pub aria_labelledby: Option<String>,
    pub aria_describedby: Signal<Option<String>>,
    pub aria_disabled: Signal<Option<AriaDisabled>>,
    pub element_capture: ElementCaptureAttr,
    pub on_keydown: EventHandler<KeyboardEvent>,
}

impl IntoAttrs for UseDatePickerGroupProps {
    type Attrs = UseDatePickerGroupAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Id, self.id),
            Attr(attr::Role, self.role),
            Attr(attr::AriaLabelledby, self.aria_labelledby),
            Attr(attr::AriaDescribedby, self.aria_describedby),
            Attr(attr::AriaDisabled, self.aria_disabled),
            self.element_capture,
            self.on_keydown.into_on(ev::keydown),
        )
    }
}

/// Attributes for the date picker group container.
pub type UseDatePickerGroupAttrs = (
    Attr<attr::Id, String>,
    Attr<attr::Role, AriaRole>,
    Attr<attr::AriaLabelledby, Option<String>>,
    Attr<attr::AriaDescribedby, Signal<Option<String>>>,
    Attr<attr::AriaDisabled, Signal<Option<AriaDisabled>>>,
    ElementCaptureAttr,
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
);

/// Props for the label element. Call `.into_attrs()` for view spreading.
#[derive(Debug)]
pub struct UseDatePickerLabelProps {
    /// The id of the label element.
    pub id: String,
    /// Click handler that focuses the first segment in the field.
    pub on_click: EventHandler<MouseEvent>,
}

impl IntoAttrs for UseDatePickerLabelProps {
    type Attrs = UseDatePickerLabelAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (Attr(attr::Id, self.id), self.on_click.into_on(ev::click))
    }
}

/// Attributes for the label element.
pub type UseDatePickerLabelAttrs = (
    Attr<attr::Id, String>,
    On<ev::click, SharedEventCallback<MouseEvent>>,
);

/// Props for the field container.
#[derive(Debug)]
pub struct UseDatePickerFieldProps {
    /// The id of the field element.
    pub id: String,
    /// The role attribute (`Presentation` when inside a date picker).
    pub role: AriaRole,
    /// `aria-haspopup` attribute.
    pub aria_haspopup: &'static str,
    /// `aria-expanded` attribute reflecting open state.
    pub aria_expanded: Signal<Option<AriaExpanded>>,
}

impl IntoAttrs for UseDatePickerFieldProps {
    type Attrs = UseDatePickerFieldAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Id, self.id),
            Attr(attr::Role, self.role),
            Attr(attr::AriaHaspopup, self.aria_haspopup),
            Attr(attr::AriaExpanded, self.aria_expanded),
        )
    }
}

/// Attributes for the field container.
pub type UseDatePickerFieldAttrs = (
    Attr<attr::Id, String>,
    Attr<attr::Role, AriaRole>,
    Attr<attr::AriaHaspopup, &'static str>,
    Attr<attr::AriaExpanded, Signal<Option<AriaExpanded>>>,
);

/// Props from `use_date_picker` for the calendar trigger button.
#[derive(Debug)]
pub struct UseDatePickerButtonProps {
    pub id: String,
    pub aria_label: &'static str,
    pub aria_labelledby: String,
    pub aria_haspopup: &'static str,
    pub aria_expanded: Signal<Option<AriaExpanded>>,
    pub aria_disabled: Signal<Option<AriaDisabled>>,
    pub aria_describedby: Signal<Option<String>>,
    pub tabindex: &'static str,
    pub on_click: EventHandler<MouseEvent>,
    pub on_keydown: EventHandler<KeyboardEvent>,
}

impl IntoAttrs for UseDatePickerButtonProps {
    type Attrs = UseDatePickerButtonAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Id, self.id),
            Attr(attr::AriaLabel, self.aria_label),
            Attr(attr::AriaLabelledby, self.aria_labelledby),
            Attr(attr::AriaHaspopup, self.aria_haspopup),
            Attr(attr::AriaExpanded, self.aria_expanded),
            Attr(attr::AriaDisabled, self.aria_disabled),
            Attr(attr::AriaDescribedby, self.aria_describedby),
            Attr(attr::Tabindex, self.tabindex),
            self.on_click.into_on(ev::click),
            self.on_keydown.into_on(ev::keydown),
        )
    }
}

/// Attributes for the calendar trigger button.
pub type UseDatePickerButtonAttrs = (
    Attr<attr::Id, String>,
    Attr<attr::AriaLabel, &'static str>,
    Attr<attr::AriaLabelledby, String>,
    Attr<attr::AriaHaspopup, &'static str>,
    Attr<attr::AriaExpanded, Signal<Option<AriaExpanded>>>,
    Attr<attr::AriaDisabled, Signal<Option<AriaDisabled>>>,
    Attr<attr::AriaDescribedby, Signal<Option<String>>>,
    Attr<attr::Tabindex, &'static str>,
    On<ev::click, SharedEventCallback<MouseEvent>>,
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
);

/// Props for the description element.
#[derive(Debug)]
pub struct UseDatePickerDescriptionProps {
    /// The id of the description element.
    pub id: String,
}

/// Props for the error message element.
#[derive(Debug)]
pub struct UseDatePickerErrorProps {
    /// The id of the error message element.
    pub id: String,
    /// The role for the error message.
    pub role: AriaRole,
    /// The `aria-live` attribute.
    pub aria_live: &'static str,
}

/// Props for the calendar dialog/popover.
#[derive(Debug)]
pub struct UseDatePickerDialogProps {
    /// The id of the dialog element.
    pub id: String,
    /// The role attribute.
    pub role: AriaRole,
    /// `aria-modal` attribute.
    pub aria_modal: AriaModal,
    /// `aria-labelledby` — references both the button and the label.
    pub aria_labelledby: String,
}

/// Configuration for the calendar inside the date picker popover.
///
/// These are data props (not ARIA attributes) that the consumer passes to
/// the calendar component. The `on_change` callback is pre-wired to feed
/// into the date picker's staged commit system.
#[derive(Debug)]
pub struct UseDatePickerCalendarProps {
    /// The id of the calendar element.
    pub id: String,
    /// Whether the calendar should auto-focus when the popover opens.
    pub auto_focus: bool,
    /// The current date value for the calendar (staged or committed, as `OffsetDateTime`).
    pub value: Signal<Option<time::OffsetDateTime>>,
    /// Callback when the calendar selects a date. Pre-wired to `state.set_date_value`.
    pub on_change: Callback<time::OffsetDateTime>,
    /// The minimum allowed date.
    pub min: Option<time::OffsetDateTime>,
    /// The maximum allowed date.
    pub max: Option<time::OffsetDateTime>,
    /// Whether the calendar is disabled.
    pub is_disabled: Signal<bool>,
    /// Whether the calendar is read-only.
    pub is_read_only: Signal<bool>,
    /// Callback to check if a date is unavailable.
    pub is_date_unavailable: Option<Callback<time::Date, bool>>,
    /// Whether the value is invalid.
    pub is_invalid: Signal<bool>,
    /// Error message for the calendar.
    pub error_message: Option<String>,
    /// Default date to focus when no value is selected.
    pub default_focused_value: Option<time::OffsetDateTime>,
}

/// Provides the behavior and accessibility for a date picker.
///
/// A date picker combines a date field with a calendar popover.
/// Creates internal state via [`use_date_picker_state`] and adds ARIA
/// attributes, keyboard navigation (Alt+Arrow to open), and an accessible
/// "Selected date: ..." description.
///
/// # Example
///
/// ```ignore
/// let picker = use_date_picker(UseDatePickerInput {
///     label: Some("Select Date".to_string()),
///     on_change: Some(Callback::new(move |v| { /* handle value */ })),
///     ..Default::default()
/// });
///
/// // Create a date field wired to the picker's state:
/// let field = use_date_field(UseDateFieldInput {
///     value: picker.state.value,
///     on_change: Some(picker.state.set_value),
///     is_date_picker: true,
///     ..Default::default()
/// });
///
/// view! {
///     <div {..picker.group_props.into_attrs()}>
///         <span {..picker.label_props.into_attrs()}>"Select Date"</span>
///         <div {..field.field_props.into_attrs()} {..picker.field_props.into_attrs()}>
///             // Render segments from field...
///         </div>
///         <button {..picker.button_props.into_attrs()}>"📅"</button>
///     </div>
///     <Show when=move || picker.is_open.get()>
///         <div id=picker.dialog_props.id
///              role=picker.dialog_props.role
///              aria-modal=picker.dialog_props.aria_modal
///              aria-labelledby=picker.dialog_props.aria_labelledby>
///             // Calendar component configured with picker.calendar_props...
///         </div>
///     </Show>
/// }
/// ```
#[allow(clippy::too_many_lines, clippy::needless_pass_by_value)]
pub fn use_date_picker(input: UseDatePickerInput) -> UseDatePickerReturn {
    let UseDatePickerInput {
        default_value,
        min,
        max,
        is_disabled: disabled,
        is_read_only,
        is_required: _,
        label,
        description,
        error_message,
        on_change,
        on_open_change,
        show_time,
        hour_cycle_24: _,
        should_close_on_select,
        is_date_unavailable,
        is_invalid,
        validate,
        validation_behavior,
        name,
    } = input;

    // ---- Create state ----
    let state = use_date_picker_state(UseDatePickerStateInput {
        default_value,
        min,
        max,
        show_time,
        should_close_on_select,
        is_date_unavailable,
        on_change,
        on_open_change,
        is_disabled: disabled,
        is_read_only,
        is_invalid,
        validate,
        validation_behavior,
        name,
    });

    // ---- IDs ----
    let base_id = Uuid::new_v4();
    let picker_id = format!("date-picker-{base_id}");
    let label_id = format!("date-picker-label-{base_id}");
    let field_id = format!("date-picker-field-{base_id}");
    let button_id = format!("date-picker-button-{base_id}");
    let dialog_id = format!("date-picker-dialog-{base_id}");
    let calendar_id = format!("date-picker-calendar-{base_id}");
    let description_id = format!("date-picker-desc-{base_id}");
    let error_id = format!("date-picker-error-{base_id}");
    let value_desc_id = format!("date-picker-selected-{base_id}");

    // ---- Booleans for label/description/error presence ----
    let has_label = label.is_some();
    let has_description = description.is_some();
    let has_error = error_message.is_some();

    // ---- labelled_by: label ID if present, else field ID as fallback ----
    let labelled_by = if has_label {
        label_id.clone()
    } else {
        field_id.clone()
    };

    // ---- Group element capture (for label click → focus) ----
    let group_element = CapturedElement::new();

    // ---- Accessibility description ----
    // Hidden DOM element with "Selected date: ..." text.
    let value_description = Signal::derive({
        let formatted_value = state.formatted_value;
        move || {
            let formatted = formatted_value.get();
            if formatted.is_empty() {
                String::new()
            } else {
                format!("Selected date: {formatted}")
            }
        }
    });

    // Create/update hidden description element on the client.
    {
        let desc_id_for_effect = value_desc_id.clone();
        Effect::new(move |_| {
            let text = value_description.get();
            let doc = use_document();
            let Some(document) = doc.as_ref() else {
                return;
            };

            if let Some(el) = document.get_element_by_id(&desc_id_for_effect) {
                el.set_text_content(if text.is_empty() { None } else { Some(&text) });
            } else if !text.is_empty() {
                if let Ok(el) = document.create_element("span") {
                    el.set_id(&desc_id_for_effect);
                    let _ = el.set_attribute("hidden", "");
                    el.set_text_content(Some(&text));
                    if let Some(body) = document.body() {
                        let _ = body.append_child(&el);
                    }
                }
            }
        });

        let desc_id_for_cleanup = value_desc_id.clone();
        on_cleanup(move || {
            let doc = use_document();
            if let Some(document) = doc.as_ref() {
                if let Some(el) = document.get_element_by_id(&desc_id_for_cleanup) {
                    el.remove();
                }
            }
        });
    }

    // ---- Reactive ARIA attributes ----
    let aria_disabled = Signal::derive(move || disabled.get().then_some(AriaDisabled::True));
    let aria_expanded = Signal::derive({
        let is_open = state.is_open;
        move || Some(AriaExpanded::from(is_open.get()))
    });

    // Group aria-describedby: value description + user description + error.
    let group_aria_describedby = Signal::derive({
        let value_desc_id = value_desc_id.clone();
        let description_id = description_id.clone();
        let error_id = error_id.clone();
        let combined_is_invalid = state.is_invalid;
        move || {
            let mut parts = Vec::new();
            if !value_description.get().is_empty() {
                parts.push(value_desc_id.clone());
            }
            if has_description {
                parts.push(description_id.clone());
            }
            if has_error || combined_is_invalid.get() {
                parts.push(error_id.clone());
            }
            if parts.is_empty() {
                None
            } else {
                Some(parts.join(" "))
            }
        }
    });

    // Button aria-describedby: same as group.
    let button_aria_describedby = Signal::derive({
        let value_desc_id = value_desc_id.clone();
        let description_id = description_id.clone();
        let error_id = error_id.clone();
        let combined_is_invalid = state.is_invalid;
        move || {
            let mut parts = Vec::new();
            if !value_description.get().is_empty() {
                parts.push(value_desc_id.clone());
            }
            if has_description {
                parts.push(description_id.clone());
            }
            if has_error || combined_is_invalid.get() {
                parts.push(error_id.clone());
            }
            if parts.is_empty() {
                None
            } else {
                Some(parts.join(" "))
            }
        }
    });

    // Button aria-labelledby: "{buttonId} {labelledBy}" (references both).
    let button_aria_labelledby = format!("{button_id} {labelled_by}");

    // Button disabled when either disabled or read-only.
    let button_disabled = Signal::derive(move || {
        (disabled.get() || is_read_only.get()).then_some(AriaDisabled::True)
    });

    // ---- Group keyboard handler ----
    let state_set_open = state.set_open;
    let handle_group_keydown = move |e: KeyboardEvent| {
        // Alt+ArrowDown or Alt+ArrowUp opens the popover (standard ARIA pattern).
        if e.alt_key() && (e.key() == "ArrowDown" || e.key() == "ArrowUp") {
            e.prevent_default();
            e.stop_propagation();
            state_set_open.run(true);
        }
    };

    // ---- Label click → focus first segment ----
    let handle_label_click = move |_: MouseEvent| {
        if let Some(el) = group_element.get() {
            // Find the first tabbable segment within the group.
            if let Ok(Some(first)) =
                el.query_selector("[tabindex]:not([tabindex='-1']), [data-segment]")
            {
                if let Some(focusable) = first.dyn_ref::<web_sys::HtmlElement>() {
                    let _ = focusable.focus();
                }
            }
        }
    };

    // ---- Button click → only open (not toggle) ----
    let handle_button_click = move |_: MouseEvent| {
        if !disabled.get_untracked() && !is_read_only.get_untracked() {
            state_set_open.run(true);
        }
    };

    // ---- Button keyboard ----
    let handle_button_keydown = move |e: KeyboardEvent| {
        if disabled.get_untracked() || is_read_only.get_untracked() {
            return;
        }
        match e.key().as_str() {
            "Enter" | " " => {
                e.prevent_default();
                state_set_open.run(true);
            }
            _ => {}
        }
    };

    // ---- Calendar props ----
    let calendar_on_change = Callback::new({
        let set_date_value = state.set_date_value;
        move |odt: time::OffsetDateTime| {
            set_date_value.run(odt.date());
        }
    });

    let calendar_value = Signal::derive({
        let date_value = state.date_value;
        move || {
            date_value
                .get()
                .map(|d| d.with_time(time::Time::MIDNIGHT).assume_utc())
        }
    });

    let default_focused_value = if state.date_value.get_untracked().is_some() {
        None
    } else {
        Some(default_value.unwrap_or_else(time::OffsetDateTime::now_utc))
    };

    // ---- Dialog aria-labelledby: "{buttonId} {labelledBy}" ----
    let dialog_aria_labelledby = format!("{button_id} {labelled_by}");

    UseDatePickerReturn {
        group_props: UseDatePickerGroupProps {
            id: picker_id.clone(),
            role: AriaRole::Group,
            aria_labelledby: if has_label {
                Some(label_id.clone())
            } else {
                None
            },
            aria_describedby: group_aria_describedby,
            aria_disabled,
            element_capture: group_element.attr(),
            on_keydown: EventHandler::new(handle_group_keydown),
        },
        label_props: UseDatePickerLabelProps {
            id: label_id,
            on_click: EventHandler::new(handle_label_click),
        },
        field_props: UseDatePickerFieldProps {
            id: field_id,
            role: AriaRole::Presentation,
            aria_haspopup: "dialog",
            aria_expanded,
        },
        button_props: UseDatePickerButtonProps {
            id: button_id,
            aria_label: "Calendar",
            aria_labelledby: button_aria_labelledby,
            aria_haspopup: "dialog",
            aria_expanded,
            aria_disabled: button_disabled,
            aria_describedby: button_aria_describedby,
            tabindex: "0",
            on_click: EventHandler::new(handle_button_click),
            on_keydown: EventHandler::new(handle_button_keydown),
        },
        description_props: UseDatePickerDescriptionProps { id: description_id },
        error_props: UseDatePickerErrorProps {
            id: error_id,
            role: AriaRole::Alert,
            aria_live: "polite",
        },
        dialog_props: UseDatePickerDialogProps {
            id: dialog_id,
            role: AriaRole::Dialog,
            aria_modal: AriaModal::True,
            aria_labelledby: dialog_aria_labelledby,
        },
        calendar_props: UseDatePickerCalendarProps {
            id: calendar_id,
            auto_focus: true,
            value: calendar_value,
            on_change: calendar_on_change,
            min,
            max,
            is_disabled: disabled,
            is_read_only,
            is_date_unavailable,
            is_invalid: state.is_invalid,
            error_message,
            default_focused_value,
        },
        state,
        is_open: state.is_open,
        value: state.value,
        open: state.open,
        close: state.close,
        set_open: state.set_open,
        is_invalid: state.is_invalid,
        validation_errors: state.validation_errors,
        value_description,
        value_description_id: value_desc_id,
        picker_id,
    }
}
