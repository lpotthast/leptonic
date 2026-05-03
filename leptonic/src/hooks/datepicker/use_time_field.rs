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
    incomplete_date::IncompleteDate,
    incomplete_time::IncompleteTime,
    use_date_segment::{DateSegment, DateSegmentType},
};
use crate::{
    hooks::IntoAttrs,
    utils::{
        EventHandler,
        aria::{AriaDisabled, AriaInvalid, AriaRequired, AriaRole},
    },
};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/datepicker/src/useTimeField.ts

//
// DIFFERENT BEHAVIOR
// - Hook-owned state: Uses `IncompleteTime` editing buffer owned internally.
//   Callers get read-only signals and `DateSegmentType`-based mutation callbacks
//   instead of focused-segment-index-based ones.
// - Segment mutation callbacks take `DateSegmentType` instead of operating on
//   the focused segment implicitly.
//

/// A time value with hours, minutes, and optional seconds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TimeValue {
    /// Hours (0-23).
    pub hour: u8,
    /// Minutes (0-59).
    pub minute: u8,
    /// Seconds (0-59).
    pub second: u8,
}

impl TimeValue {
    /// Creates a new time value.
    pub fn new(hour: u8, minute: u8, second: u8) -> Self {
        Self {
            hour: hour.min(23),
            minute: minute.min(59),
            second: second.min(59),
        }
    }

    /// Creates a time from hours and minutes.
    pub fn hm(hour: u8, minute: u8) -> Self {
        Self::new(hour, minute, 0)
    }

    /// Formats as HH:MM.
    pub fn format_hm(&self) -> String {
        format!("{:02}:{:02}", self.hour, self.minute)
    }

    /// Formats as HH:MM:SS.
    pub fn format_hms(&self) -> String {
        format!("{:02}:{:02}:{:02}", self.hour, self.minute, self.second)
    }

    /// Converts to 12-hour format.
    pub fn to_12_hour(&self) -> (u8, bool) {
        let is_pm = self.hour >= 12;
        let hour_12 = match self.hour {
            0 => 12,
            1..=12 => self.hour,
            _ => self.hour - 12,
        };
        (hour_12, is_pm)
    }
}

/// Input parameters for the `use_time_field` hook.
#[derive(Debug, Clone)]
pub struct UseTimeFieldInput {
    /// The current time value.
    pub value: Signal<Option<TimeValue>>,

    /// The minimum allowed time.
    pub min: Option<TimeValue>,

    /// The maximum allowed time.
    pub max: Option<TimeValue>,

    /// Whether the field is disabled.
    pub is_disabled: Signal<bool>,

    /// Whether the field is read-only.
    pub is_read_only: Signal<bool>,

    /// Whether the field is required.
    pub is_required: bool,

    /// Whether to show seconds.
    pub show_seconds: bool,

    /// Whether to use 24-hour format.
    pub hour_cycle_24: bool,

    /// The label for the field.
    pub label: Option<String>,

    /// The description for the field.
    pub description: Option<String>,

    /// Error message to display.
    pub error_message: Option<String>,

    /// Callback when the value changes.
    pub on_change: Option<Callback<Option<TimeValue>>>,
}

impl Default for UseTimeFieldInput {
    fn default() -> Self {
        Self {
            value: Signal::derive(|| None),
            min: None,
            max: None,
            is_disabled: Signal::derive(|| false),
            is_read_only: Signal::derive(|| false),
            is_required: false,
            show_seconds: false,
            hour_cycle_24: true,
            label: None,
            description: None,
            error_message: None,
            on_change: None,
        }
    }
}

/// The return value of the `use_time_field` hook.
pub struct UseTimeFieldReturn {
    /// Props for the field container element. Call `.into_attrs()` for view spreading.
    pub field_props: UseTimeFieldProps,

    /// Props for the label element.
    pub label_props: UseTimeFieldLabelProps,

    /// Props for the description element.
    pub description_props: UseTimeFieldDescriptionProps,

    /// Props for the error message element.
    pub error_props: UseTimeFieldErrorProps,

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

    /// Clear a segment.
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
}

/// Props from `use_time_field` for the field container element.
#[derive(Debug)]
pub struct UseTimeFieldProps {
    pub id: String,
    pub role: AriaRole,
    pub aria_labelledby: Option<String>,
    pub aria_describedby: Signal<Option<String>>,
    pub aria_disabled: Signal<Option<AriaDisabled>>,
    pub aria_invalid: Signal<Option<AriaInvalid>>,
    pub aria_required: Option<AriaRequired>,
    pub on_keydown: EventHandler<KeyboardEvent>,
}

impl IntoAttrs for UseTimeFieldProps {
    type Attrs = UseTimeFieldAttrs;

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

/// Attributes for the time field container element.
pub type UseTimeFieldAttrs = (
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
pub struct UseTimeFieldLabelProps {
    /// The id of the label element.
    pub id: String,
}

/// Props for the description element.
#[derive(Debug)]
pub struct UseTimeFieldDescriptionProps {
    /// The id of the description element.
    pub id: String,
}

/// Props for the error message element.
#[derive(Debug)]
pub struct UseTimeFieldErrorProps {
    /// The id of the error message element.
    pub id: String,
    /// The role for the error message.
    pub role: AriaRole,
    /// The aria-live attribute.
    pub aria_live: &'static str,
}

/// Provides the behavior and accessibility for a time field.
///
/// A time field allows users to enter a time using editable segments.
/// Uses an `IncompleteTime` editing buffer for partial state support.
///
/// # Example
///
/// ```ignore
/// let (value, set_value) = signal(None);
///
/// let field = use_time_field(UseTimeFieldInput {
///     value: value.into(),
///     label: Some("Time".to_string()),
///     on_change: Some(Callback::new(move |v| set_value.set(v))),
///     ..Default::default()
/// });
///
/// view! {
///     <div>
///         <label id=field.label_props.id>"Time"</label>
///         <div {..field.field_props.into_attrs()}>
///             // Render segments...
///         </div>
///     </div>
/// }
/// ```
#[allow(clippy::too_many_lines, clippy::needless_pass_by_value)]
pub fn use_time_field(input: UseTimeFieldInput) -> UseTimeFieldReturn {
    let UseTimeFieldInput {
        value,
        min: _,
        max: _,
        is_disabled: disabled,
        is_read_only,
        is_required,
        show_seconds,
        hour_cycle_24,
        label,
        description,
        error_message,
        on_change,
    } = input;

    // ---- Internal editing buffer ----
    let initial_incomplete = value.get_untracked().map_or_else(
        || IncompleteTime::empty(hour_cycle_24),
        |t| IncompleteTime::from_time(&t, hour_cycle_24),
    );
    let display_value = StoredValue::new(initial_incomplete);
    let placeholder = StoredValue::new(TimeValue::default());
    let display_trigger = Trigger::new();

    // Sync display_value when external value changes.
    Effect::new(move |_| {
        let ext = value.get();
        match ext {
            Some(time) => {
                display_value.update_value(|dv| dv.sync_from_time(&time));
            }
            None => {
                display_value.set_value(IncompleteTime::empty(hour_cycle_24));
            }
        }
        display_trigger.notify();
    });

    // ---- Emit helpers ----
    let emit = move |new_value: Option<TimeValue>| {
        if let Some(on_change) = on_change {
            on_change.run(new_value);
        }
    };

    let try_emit_if_complete = move || {
        let dv = display_value.get_value();
        if dv.is_complete(show_seconds) {
            let ph = placeholder.get_value();
            emit(Some(dv.to_time(&ph)));
        } else if dv.is_cleared(show_seconds) {
            emit(None);
        }
    };

    // ---- IDs ----
    let base_id = Uuid::new_v4();
    let field_id = format!("time-field-{base_id}");
    let label_id = format!("time-field-label-{base_id}");
    let description_id = format!("time-field-desc-{base_id}");
    let error_id = format!("time-field-error-{base_id}");

    // ---- Track focused segment ----
    let (focused_segment, set_focused_segment) = signal::<Option<usize>>(None);

    // ---- Segment generation ----
    let segments = Signal::derive(move || {
        display_trigger.track();
        let dv = display_value.get_value();
        build_time_segments(&dv, show_seconds, hour_cycle_24)
    });

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

    // ---- Mutation callbacks ----
    let set_segment = Callback::new(move |(seg_type, val): (DateSegmentType, i32)| {
        if disabled.get_untracked() || is_read_only.get_untracked() {
            return;
        }
        display_value.update_value(|dv| dv.set(seg_type, val));
        display_trigger.notify();
        try_emit_if_complete();
    });

    let clear_segment = Callback::new(move |seg_type: DateSegmentType| {
        if disabled.get_untracked() || is_read_only.get_untracked() {
            return;
        }
        display_value.update_value(|dv| dv.clear(seg_type));
        display_trigger.notify();
        if display_value.get_value().is_cleared(show_seconds) {
            emit(None);
        }
    });

    let increment = Callback::new(move |seg_type: DateSegmentType| {
        if disabled.get_untracked() || is_read_only.get_untracked() {
            return;
        }
        let ph = placeholder.get_value();
        display_value.update_value(|dv| dv.cycle(seg_type, 1, &ph));
        display_trigger.notify();
        try_emit_if_complete();
    });

    let decrement = Callback::new(move |seg_type: DateSegmentType| {
        if disabled.get_untracked() || is_read_only.get_untracked() {
            return;
        }
        let ph = placeholder.get_value();
        display_value.update_value(|dv| dv.cycle(seg_type, -1, &ph));
        display_trigger.notify();
        try_emit_if_complete();
    });

    let increment_page = Callback::new(move |seg_type: DateSegmentType| {
        if disabled.get_untracked() || is_read_only.get_untracked() {
            return;
        }
        let ph = placeholder.get_value();
        let step = IncompleteDate::page_step(seg_type);
        display_value.update_value(|dv| dv.cycle(seg_type, step, &ph));
        display_trigger.notify();
        try_emit_if_complete();
    });

    let decrement_page = Callback::new(move |seg_type: DateSegmentType| {
        if disabled.get_untracked() || is_read_only.get_untracked() {
            return;
        }
        let ph = placeholder.get_value();
        let step = IncompleteDate::page_step(seg_type);
        display_value.update_value(|dv| dv.cycle(seg_type, -step, &ph));
        display_trigger.notify();
        try_emit_if_complete();
    });

    let increment_to_max = Callback::new(move |seg_type: DateSegmentType| {
        if disabled.get_untracked() || is_read_only.get_untracked() {
            return;
        }
        display_value.update_value(|dv| dv.set_to_max(seg_type));
        display_trigger.notify();
        try_emit_if_complete();
    });

    let decrement_to_min = Callback::new(move |seg_type: DateSegmentType| {
        if disabled.get_untracked() || is_read_only.get_untracked() {
            return;
        }
        display_value.update_value(|dv| dv.set_to_min(seg_type));
        display_trigger.notify();
        try_emit_if_complete();
    });

    let confirm_placeholder = Callback::new(move |()| {
        let dv = display_value.get_value();
        if dv.is_complete(show_seconds) {
            let ph = placeholder.get_value();
            emit(Some(dv.to_time(&ph)));
        } else if dv.is_cleared(show_seconds) {
            emit(None);
        }
    });

    // ---- Reactive ARIA attributes ----
    let has_description = description.is_some();
    let has_error = error_message.is_some();
    let has_label = label.is_some();

    let description_id_for_signal = description_id.clone();
    let error_id_for_signal = error_id.clone();
    let aria_describedby = Signal::derive(move || {
        let mut parts = Vec::new();
        if has_description {
            parts.push(description_id_for_signal.clone());
        }
        if has_error {
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
    // Time field doesn't have validation yet; always valid.
    let aria_invalid = Signal::derive(|| None::<AriaInvalid>);
    let aria_required = is_required.then_some(AriaRequired::True);

    // Keyboard handler (field-level).
    let handle_keydown = move |_e: KeyboardEvent| {
        // Navigation is handled by individual segments.
        // Disabled/read-only check is a no-op at field level.
        let _ = disabled.get_untracked();
    };

    UseTimeFieldReturn {
        field_props: UseTimeFieldProps {
            id: field_id.clone(),
            role: AriaRole::Group,
            aria_labelledby,
            aria_describedby,
            aria_disabled,
            aria_invalid,
            aria_required,
            on_keydown: EventHandler::new(handle_keydown),
        },
        label_props: UseTimeFieldLabelProps { id: label_id },
        description_props: UseTimeFieldDescriptionProps { id: description_id },
        error_props: UseTimeFieldErrorProps {
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
        increment,
        decrement,
        set_segment,
        clear_segment,
        increment_page,
        decrement_page,
        increment_to_max,
        decrement_to_min,
        confirm_placeholder,
    }
}

/// Build time segments from the incomplete time state.
fn build_time_segments(
    dv: &IncompleteTime,
    show_seconds: bool,
    hour_cycle_24: bool,
) -> Vec<DateSegment> {
    let mut segs = vec![
        DateSegment::hour(dv.hour, hour_cycle_24),
        DateSegment::literal(":"),
        DateSegment::minute(dv.minute),
    ];

    if show_seconds {
        segs.push(DateSegment::literal(":"));
        segs.push(DateSegment::second(dv.second));
    }

    if !hour_cycle_24 {
        segs.push(DateSegment::literal(" "));
        segs.push(DateSegment::day_period(dv.day_period));
    }

    segs
}
