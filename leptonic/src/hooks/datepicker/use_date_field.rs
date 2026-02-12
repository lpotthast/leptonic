use leptos::attr;
use leptos::attr::Attr;
use leptos::ev;
use leptos::ev::{On, SharedEventCallback};
use leptos::prelude::*;
use uuid::Uuid;
use web_sys::KeyboardEvent;

use super::use_date_segment::{DateSegment, DateSegmentType};
use crate::utils::aria::{AriaDisabled, AriaRequired};
use crate::utils::time::whole_days_in;
use crate::utils::EventHandler;

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/datepicker/src/useDateField.ts

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// No intentional deviations from the react-aria implementation.
//
// =============================================================================

/// Input parameters for the `use_date_field` hook.
#[derive(Debug, Clone)]
pub struct UseDateFieldInput {
    /// The current date value.
    pub value: Signal<Option<time::OffsetDateTime>>,

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
}

impl Default for UseDateFieldInput {
    fn default() -> Self {
        Self {
            value: Signal::derive(|| None),
            min: None,
            max: None,
            is_disabled: Signal::derive(|| false),
            is_read_only: Signal::derive(|| false),
            is_required: false,
            label: None,
            description: None,
            error_message: None,
            on_change: None,
            show_time: false,
            hour_cycle_24: true,
        }
    }
}

/// The return value of the `use_date_field` hook.
#[derive(Debug)]
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

    /// Increment the focused segment.
    pub increment: Callback<()>,

    /// Decrement the focused segment.
    pub decrement: Callback<()>,

    /// Set a segment value.
    pub set_segment: Callback<(usize, i32)>,
}

/// Props from `use_date_field` for the field container element.
#[derive(Debug)]
pub struct UseDateFieldProps {
    pub id: String,
    pub role: &'static str,
    pub aria_labelledby: Option<String>,
    pub aria_describedby: Option<String>,
    pub aria_disabled: Signal<Option<AriaDisabled>>,
    pub aria_required: Option<AriaRequired>,
    pub on_keydown: EventHandler<KeyboardEvent>,
}

impl UseDateFieldProps {
    /// Convert to spreadable attributes for Leptos views, consuming self.
    #[must_use]
    pub fn into_attrs(self) -> UseDateFieldAttrs {
        (
            Attr(attr::Id, self.id),
            Attr(attr::Role, self.role),
            Attr(attr::AriaLabelledby, self.aria_labelledby),
            Attr(attr::AriaDescribedby, self.aria_describedby),
            Attr(attr::AriaDisabled, self.aria_disabled),
            Attr(attr::AriaRequired, self.aria_required),
            self.on_keydown.into_on(ev::keydown),
        )
    }
}

/// Attributes for the date field container element.
pub type UseDateFieldAttrs = (
    Attr<attr::Id, String>,
    Attr<attr::Role, &'static str>,
    Attr<attr::AriaLabelledby, Option<String>>,
    Attr<attr::AriaDescribedby, Option<String>>,
    Attr<attr::AriaDisabled, Signal<Option<AriaDisabled>>>,
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
    pub role: &'static str,
    /// The aria-live attribute.
    pub aria_live: &'static str,
}

/// Provides the behavior and accessibility for a date field.
///
/// A date field allows users to enter a date using editable segments.
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
///
/// # Panics
///
/// Panics if the `on` event handler cannot be converted to a cloneable callback.
#[allow(clippy::too_many_lines, clippy::needless_pass_by_value)]
pub fn use_date_field(input: UseDateFieldInput) -> UseDateFieldReturn {
    let UseDateFieldInput {
        value,
        min,
        max,
        is_disabled: disabled,
        is_read_only,
        is_required,
        label,
        description,
        error_message,
        on_change,
        show_time,
        hour_cycle_24,
    } = input;

    let base_id = Uuid::new_v4();
    let field_id = format!("date-field-{base_id}");
    let label_id = format!("date-field-label-{base_id}");
    let description_id = format!("date-field-desc-{base_id}");
    let error_id = format!("date-field-error-{base_id}");

    // Track focused segment
    let (focused_segment, set_focused_segment) = signal::<Option<usize>>(None);

    // Generate segments from the current value
    let segments = Signal::derive(move || {
        let date_opt = value.get();
        let mut segs = Vec::new();

        if let Some(date) = date_opt {
            // Year
            segs.push(DateSegment::year(Some(date.year())));
            segs.push(DateSegment::literal("-"));
            // Month
            segs.push(DateSegment::month(Some(date.month() as u8)));
            segs.push(DateSegment::literal("-"));
            // Day
            let max_day = whole_days_in(date.year(), date.month());
            segs.push(DateSegment::day(Some(date.day()), max_day));

            if show_time {
                segs.push(DateSegment::literal(" "));
                // Hour
                let hour = date.hour();
                if hour_cycle_24 {
                    segs.push(DateSegment::hour(Some(hour), true));
                } else {
                    let display_hour = if hour == 0 {
                        12
                    } else if hour > 12 {
                        hour - 12
                    } else {
                        hour
                    };
                    segs.push(DateSegment::hour(Some(display_hour), false));
                }
                segs.push(DateSegment::literal(":"));
                // Minute
                segs.push(DateSegment::minute(Some(date.minute())));

                if !hour_cycle_24 {
                    segs.push(DateSegment::literal(" "));
                    segs.push(DateSegment::day_period(Some(hour >= 12)));
                }
            }
        } else {
            // Placeholder segments
            segs.push(DateSegment::year(None));
            segs.push(DateSegment::literal("-"));
            segs.push(DateSegment::month(None));
            segs.push(DateSegment::literal("-"));
            segs.push(DateSegment::day(None, 31));

            if show_time {
                segs.push(DateSegment::literal(" "));
                segs.push(DateSegment::hour(None, hour_cycle_24));
                segs.push(DateSegment::literal(":"));
                segs.push(DateSegment::minute(None));

                if !hour_cycle_24 {
                    segs.push(DateSegment::literal(" "));
                    segs.push(DateSegment::day_period(None));
                }
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
        if current_value.is_none() {
            return;
        }

        let date = current_value.unwrap();
        let segment_idx = focused.unwrap();

        segments.with_untracked(|segs| {
            if let Some(seg) = segs.get(segment_idx) {
                let new_date = match seg.segment_type {
                    DateSegmentType::Year => date.replace_year(date.year() + 1).ok(),
                    DateSegmentType::Month => {
                        let next_month = date.month().next();
                        if next_month == time::Month::January {
                            date.replace_year(date.year() + 1)
                                .and_then(|d| d.replace_month(next_month))
                                .ok()
                        } else {
                            date.replace_month(next_month).ok()
                        }
                    }
                    DateSegmentType::Day => {
                        let max_day = whole_days_in(date.year(), date.month());
                        let new_day = if date.day() >= max_day {
                            1
                        } else {
                            date.day() + 1
                        };
                        date.replace_day(new_day).ok()
                    }
                    DateSegmentType::Hour => {
                        let new_hour = (date.hour() + 1) % 24;
                        date.replace_hour(new_hour).ok()
                    }
                    DateSegmentType::Minute => {
                        let new_minute = (date.minute() + 1) % 60;
                        date.replace_minute(new_minute).ok()
                    }
                    _ => None,
                };

                if let Some(new_date) = new_date {
                    if let Some(on_change) = on_change {
                        on_change.run(Some(new_date));
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
        if current_value.is_none() {
            return;
        }

        let date = current_value.unwrap();
        let segment_idx = focused.unwrap();

        segments.with_untracked(|segs| {
            if let Some(seg) = segs.get(segment_idx) {
                let new_date = match seg.segment_type {
                    DateSegmentType::Year => date.replace_year(date.year() - 1).ok(),
                    DateSegmentType::Month => {
                        let prev_month = date.month().previous();
                        if prev_month == time::Month::December {
                            date.replace_year(date.year() - 1)
                                .and_then(|d| d.replace_month(prev_month))
                                .ok()
                        } else {
                            date.replace_month(prev_month).ok()
                        }
                    }
                    DateSegmentType::Day => {
                        let max_day = whole_days_in(date.year(), date.month());
                        let new_day = if date.day() <= 1 {
                            max_day
                        } else {
                            date.day() - 1
                        };
                        date.replace_day(new_day).ok()
                    }
                    DateSegmentType::Hour => {
                        let new_hour = if date.hour() == 0 {
                            23
                        } else {
                            date.hour() - 1
                        };
                        date.replace_hour(new_hour).ok()
                    }
                    DateSegmentType::Minute => {
                        let new_minute = if date.minute() == 0 {
                            59
                        } else {
                            date.minute() - 1
                        };
                        date.replace_minute(new_minute).ok()
                    }
                    _ => None,
                };

                if let Some(new_date) = new_date {
                    if let Some(on_change) = on_change {
                        on_change.run(Some(new_date));
                    }
                }
            }
        });
    });

    // Set a specific segment value
    let set_segment = Callback::new(move |(_index, _value): (usize, i32)| {
        // This would need more complex logic to handle typed input
        // For now, just a placeholder
    });

    // Build aria-labelledby
    let aria_labelledby = if label.is_some() {
        Some(label_id.clone())
    } else {
        None
    };

    // Build aria-describedby
    let aria_describedby = if description.is_some() || error_message.is_some() {
        let mut ids = Vec::new();
        if description.is_some() {
            ids.push(description_id.clone());
        }
        if error_message.is_some() {
            ids.push(error_id.clone());
        }
        Some(ids.join(" "))
    } else {
        None
    };

    // Compute aria-disabled
    let aria_disabled = Signal::derive(move || disabled.get().then_some(AriaDisabled::True));

    let aria_required = is_required.then_some(AriaRequired::True);

    // Handle keyboard navigation
    let handle_keydown = move |e: KeyboardEvent| {
        if disabled.get_untracked() || is_read_only.get_untracked() {
            return;
        }

        let key = e.key();
        match key.as_str() {
            "ArrowRight" | "Tab" if !e.shift_key() => {
                // Focus next segment handled by individual segments
            }
            "ArrowLeft" | "Tab" if e.shift_key() => {
                // Focus previous segment handled by individual segments
            }
            _ => {}
        }
    };

    UseDateFieldReturn {
        field_props: UseDateFieldProps {
            id: field_id.clone(),
            role: "group",
            aria_labelledby,
            aria_describedby,
            aria_disabled,
            aria_required,
            on_keydown: EventHandler::new(handle_keydown),
        },
        label_props: UseDateFieldLabelProps { id: label_id },
        description_props: UseDateFieldDescriptionProps { id: description_id },
        error_props: UseDateFieldErrorProps {
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
        set_segment,
    }
}
