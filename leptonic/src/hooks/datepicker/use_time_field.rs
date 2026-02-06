use leptos::attr;
use leptos::attr::{Attr, Attribute};
use leptos::ev;
use leptos::ev::{on, On, SharedEventCallback};
use leptos::prelude::*;
use uuid::Uuid;
use web_sys::KeyboardEvent;

use super::use_date_segment::{DateSegment, DateSegmentType};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/datepicker/src/useTimeField.ts

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
#[derive(Debug, Clone)]
pub struct UseTimeFieldReturn {
    /// Props for the field container element.
    pub field_props: UseTimeFieldAttrs,

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

    /// Increment the focused segment.
    pub increment: Callback<()>,

    /// Decrement the focused segment.
    pub decrement: Callback<()>,
}

/// Attributes for the time field container element.
pub type UseTimeFieldAttrs = (
    Attr<attr::Id, String>,
    Attr<attr::Role, &'static str>,
    Attr<attr::AriaLabelledby, Option<String>>,
    Attr<attr::AriaDescribedby, Option<String>>,
    Attr<attr::AriaDisabled, Signal<&'static str>>,
    Attr<attr::AriaRequired, &'static str>,
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
);

/// Props for the label element.
#[derive(Debug, Clone)]
pub struct UseTimeFieldLabelProps {
    /// The id of the label element.
    pub id: String,
}

/// Props for the description element.
#[derive(Debug, Clone)]
pub struct UseTimeFieldDescriptionProps {
    /// The id of the description element.
    pub id: String,
}

/// Props for the error message element.
#[derive(Debug, Clone)]
pub struct UseTimeFieldErrorProps {
    /// The id of the error message element.
    pub id: String,
    /// The role for the error message.
    pub role: &'static str,
    /// The aria-live attribute.
    pub aria_live: &'static str,
}

/// Provides the behavior and accessibility for a time field.
///
/// A time field allows users to enter a time using editable segments.
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
///         <div {..field.field_props}>
///             // Render segments...
///         </div>
///     </div>
/// }
/// ```
///
/// # Panics
///
/// Panics if the `on` event handler cannot be converted to a cloneable callback.
#[allow(clippy::too_many_lines, clippy::needless_pass_by_value)]
pub fn use_time_field(input: UseTimeFieldInput) -> UseTimeFieldReturn {
    let base_id = Uuid::new_v4();
    let field_id = format!("time-field-{base_id}");
    let label_id = format!("time-field-label-{base_id}");
    let description_id = format!("time-field-desc-{base_id}");
    let error_id = format!("time-field-error-{base_id}");

    let is_disabled = input.is_disabled;
    let is_read_only = input.is_read_only;
    let value = input.value;
    let on_change = input.on_change;
    let show_seconds = input.show_seconds;
    let hour_cycle_24 = input.hour_cycle_24;

    // Track focused segment
    let (focused_segment, set_focused_segment) = signal::<Option<usize>>(None);

    // Generate segments from the current value
    let segments = Signal::derive(move || {
        let time_opt = value.get();
        let mut segs = Vec::new();

        if let Some(time) = time_opt {
            // Hour
            if hour_cycle_24 {
                segs.push(DateSegment::hour(Some(time.hour), true));
            } else {
                let (hour_12, _is_pm) = time.to_12_hour();
                segs.push(DateSegment::hour(Some(hour_12), false));
            }
            segs.push(DateSegment::literal(":"));

            // Minute
            segs.push(DateSegment::minute(Some(time.minute)));

            // Second (optional)
            if show_seconds {
                segs.push(DateSegment::literal(":"));
                segs.push(DateSegment::second(Some(time.second)));
            }

            // AM/PM (for 12-hour)
            if !hour_cycle_24 {
                segs.push(DateSegment::literal(" "));
                let (_hour_12, is_pm) = time.to_12_hour();
                segs.push(DateSegment::day_period(Some(is_pm)));
            }
        } else {
            // Placeholder segments
            segs.push(DateSegment::hour(None, hour_cycle_24));
            segs.push(DateSegment::literal(":"));
            segs.push(DateSegment::minute(None));

            if show_seconds {
                segs.push(DateSegment::literal(":"));
                segs.push(DateSegment::second(None));
            }

            if !hour_cycle_24 {
                segs.push(DateSegment::literal(" "));
                segs.push(DateSegment::day_period(None));
            }
        }

        segs
    });

    // Get editable segment indices
    let editable_indices = move || {
        segments.with(|segs| {
            segs.iter()
                .enumerate()
                .filter(|(_, s)| s.is_editable)
                .map(|(i, _)| i)
                .collect::<Vec<_>>()
        })
    };

    // Focus callbacks
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

    // Increment/decrement the focused segment
    let increment = Callback::new(move |_| {
        let focused = focused_segment.get_untracked();
        if focused.is_none() {
            return;
        }

        let current_value = value.get_untracked();
        let time = current_value.unwrap_or_default();
        let segment_idx = focused.unwrap();

        segments.with_untracked(|segs| {
            if let Some(seg) = segs.get(segment_idx) {
                let new_time = match seg.segment_type {
                    DateSegmentType::Hour => {
                        let new_hour = (time.hour + 1) % 24;
                        Some(TimeValue::new(new_hour, time.minute, time.second))
                    }
                    DateSegmentType::Minute => {
                        let new_minute = (time.minute + 1) % 60;
                        Some(TimeValue::new(time.hour, new_minute, time.second))
                    }
                    DateSegmentType::Second => {
                        let new_second = (time.second + 1) % 60;
                        Some(TimeValue::new(time.hour, time.minute, new_second))
                    }
                    DateSegmentType::DayPeriod => {
                        // Toggle AM/PM (add/subtract 12 hours)
                        let new_hour = if time.hour >= 12 {
                            time.hour - 12
                        } else {
                            time.hour + 12
                        };
                        Some(TimeValue::new(new_hour, time.minute, time.second))
                    }
                    _ => None,
                };

                if let Some(new_time) = new_time {
                    if let Some(on_change) = on_change {
                        on_change.run(Some(new_time));
                    }
                }
            }
        });
    });

    let decrement = Callback::new(move |_| {
        let focused = focused_segment.get_untracked();
        if focused.is_none() {
            return;
        }

        let current_value = value.get_untracked();
        let time = current_value.unwrap_or_default();
        let segment_idx = focused.unwrap();

        segments.with_untracked(|segs| {
            if let Some(seg) = segs.get(segment_idx) {
                let new_time = match seg.segment_type {
                    DateSegmentType::Hour => {
                        let new_hour = if time.hour == 0 { 23 } else { time.hour - 1 };
                        Some(TimeValue::new(new_hour, time.minute, time.second))
                    }
                    DateSegmentType::Minute => {
                        let new_minute = if time.minute == 0 {
                            59
                        } else {
                            time.minute - 1
                        };
                        Some(TimeValue::new(time.hour, new_minute, time.second))
                    }
                    DateSegmentType::Second => {
                        let new_second = if time.second == 0 {
                            59
                        } else {
                            time.second - 1
                        };
                        Some(TimeValue::new(time.hour, time.minute, new_second))
                    }
                    DateSegmentType::DayPeriod => {
                        // Toggle AM/PM (add/subtract 12 hours)
                        let new_hour = if time.hour >= 12 {
                            time.hour - 12
                        } else {
                            time.hour + 12
                        };
                        Some(TimeValue::new(new_hour, time.minute, time.second))
                    }
                    _ => None,
                };

                if let Some(new_time) = new_time {
                    if let Some(on_change) = on_change {
                        on_change.run(Some(new_time));
                    }
                }
            }
        });
    });

    // Build aria-labelledby
    let aria_labelledby = if input.label.is_some() {
        Some(label_id.clone())
    } else {
        None
    };

    // Build aria-describedby
    let aria_describedby = if input.description.is_some() || input.error_message.is_some() {
        let mut ids = Vec::new();
        if input.description.is_some() {
            ids.push(description_id.clone());
        }
        if input.error_message.is_some() {
            ids.push(error_id.clone());
        }
        Some(ids.join(" "))
    } else {
        None
    };

    // Compute aria-disabled
    let aria_disabled = Signal::derive(move || if is_disabled.get() { "true" } else { "false" });

    let aria_required = if input.is_required { "true" } else { "false" };

    // Handle keyboard navigation.
    // Navigation is currently handled by individual segments.
    #[allow(clippy::unused_unit)]
    let handle_keydown = move |_e: KeyboardEvent| {
        if is_disabled.get_untracked() || is_read_only.get_untracked() {
            // Early exit: disabled or read-only fields ignore keyboard events.
        }
    };

    UseTimeFieldReturn {
        field_props: (
            Attr(attr::Id, field_id.clone()),
            Attr(attr::Role, "group"),
            Attr(attr::AriaLabelledby, aria_labelledby),
            Attr(attr::AriaDescribedby, aria_describedby),
            Attr(attr::AriaDisabled, aria_disabled),
            Attr(attr::AriaRequired, aria_required),
            on(ev::keydown, handle_keydown).into_cloneable(),
        ),
        label_props: UseTimeFieldLabelProps { id: label_id },
        description_props: UseTimeFieldDescriptionProps { id: description_id },
        error_props: UseTimeFieldErrorProps {
            id: error_id,
            role: "alert",
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
    }
}

/// State for managing time field.
#[derive(Clone, Copy)]
pub struct UseTimeFieldStateReturn {
    /// The current value.
    pub value: Signal<Option<TimeValue>>,

    /// Set the value.
    pub set_value: Callback<Option<TimeValue>>,

    /// Clear the value.
    pub clear: Callback<()>,
}

/// Creates internal state for a time field.
pub fn use_time_field_state(default_value: Option<TimeValue>) -> UseTimeFieldStateReturn {
    let (value, set_value_signal) = signal(default_value);

    UseTimeFieldStateReturn {
        value: value.into(),
        set_value: Callback::new(move |v| set_value_signal.set(v)),
        clear: Callback::new(move |_| set_value_signal.set(None)),
    }
}
