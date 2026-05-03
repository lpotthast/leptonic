use leptos::{
    attr,
    attr::Attr,
    ev,
    ev::{On, SharedEventCallback},
    prelude::*,
};
use web_sys::{FocusEvent, KeyboardEvent};

use super::incomplete_date::IncompleteDate;
use crate::{
    hooks::IntoAttrs,
    utils::{
        EventHandler,
        aria::{AriaDisabled, AriaInvalid, AriaReadonly, AriaRole},
    },
};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/datepicker/src/useDateSegment.ts

//
// DIFFERENT BEHAVIOR
// - Digit accumulation: Implemented directly here rather than in a separate
//   utility. Typed digits accumulate in an internal buffer; auto-advance
//   triggers when the parsed value * 10 would exceed max or max digits reached.
// - Keyboard callbacks take `DateSegmentType` instead of index: the parent
//   `use_date_field` state hook operates on types, not indices.
//
// LEPTOS-SPECIFIC ADAPTATIONS
// - ARIA value attributes are reactive `Signal`s rather than static strings,
//   derived from the segment data.
// - Uses `StoredValue<String>` for the digit accumulation buffer.
//

/// The type of date segment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DateSegmentType {
    /// Year segment (e.g., "2024")
    Year,
    /// Month segment (e.g., "01" or "January")
    Month,
    /// Day segment (e.g., "15")
    Day,
    /// Hour segment (e.g., "14")
    Hour,
    /// Minute segment (e.g., "30")
    Minute,
    /// Second segment (e.g., "45")
    Second,
    /// AM/PM segment
    DayPeriod,
    /// Literal segment (e.g., "/" or ":")
    Literal,
}

impl DateSegmentType {
    /// Returns a human-readable label for screen readers.
    pub fn aria_label(self) -> &'static str {
        match self {
            Self::Year => "year",
            Self::Month => "month",
            Self::Day => "day",
            Self::Hour => "hour",
            Self::Minute => "minute",
            Self::Second => "second",
            Self::DayPeriod => "AM/PM",
            Self::Literal => "",
        }
    }
}

/// A segment of a date/time value.
#[derive(Debug, Clone)]
pub struct DateSegment {
    /// The type of this segment.
    pub segment_type: DateSegmentType,

    /// The text representation of this segment.
    pub text: String,

    /// The numeric value (if applicable).
    pub value: Option<i32>,

    /// The minimum valid value.
    pub min_value: Option<i32>,

    /// The maximum valid value.
    pub max_value: Option<i32>,

    /// Whether this segment is editable.
    pub is_editable: bool,

    /// Whether this segment is a placeholder.
    pub is_placeholder: bool,
}

impl DateSegment {
    /// Creates a literal segment (non-editable separator).
    pub fn literal(text: &str) -> Self {
        Self {
            segment_type: DateSegmentType::Literal,
            text: text.to_string(),
            value: None,
            min_value: None,
            max_value: None,
            is_editable: false,
            is_placeholder: false,
        }
    }

    /// Creates a year segment.
    pub fn year(value: Option<i32>) -> Self {
        Self {
            segment_type: DateSegmentType::Year,
            text: value.map_or_else(|| "yyyy".to_string(), |v| format!("{v:04}")),
            value,
            min_value: Some(1),
            max_value: Some(9999),
            is_editable: true,
            is_placeholder: value.is_none(),
        }
    }

    /// Creates a month segment.
    pub fn month(value: Option<u8>) -> Self {
        Self {
            segment_type: DateSegmentType::Month,
            text: value.map_or_else(|| "mm".to_string(), |v| format!("{v:02}")),
            value: value.map(i32::from),
            min_value: Some(1),
            max_value: Some(12),
            is_editable: true,
            is_placeholder: value.is_none(),
        }
    }

    /// Creates a day segment.
    pub fn day(value: Option<u8>, max: u8) -> Self {
        Self {
            segment_type: DateSegmentType::Day,
            text: value.map_or_else(|| "dd".to_string(), |v| format!("{v:02}")),
            value: value.map(i32::from),
            min_value: Some(1),
            max_value: Some(i32::from(max)),
            is_editable: true,
            is_placeholder: value.is_none(),
        }
    }

    /// Creates an hour segment.
    pub fn hour(value: Option<u8>, is_24_hour: bool) -> Self {
        let (min, max) = if is_24_hour { (0, 23) } else { (1, 12) };
        Self {
            segment_type: DateSegmentType::Hour,
            text: value.map_or_else(|| "--".to_string(), |v| format!("{v:02}")),
            value: value.map(i32::from),
            min_value: Some(min),
            max_value: Some(max),
            is_editable: true,
            is_placeholder: value.is_none(),
        }
    }

    /// Creates a minute segment.
    pub fn minute(value: Option<u8>) -> Self {
        Self {
            segment_type: DateSegmentType::Minute,
            text: value.map_or_else(|| "--".to_string(), |v| format!("{v:02}")),
            value: value.map(i32::from),
            min_value: Some(0),
            max_value: Some(59),
            is_editable: true,
            is_placeholder: value.is_none(),
        }
    }

    /// Creates a second segment.
    pub fn second(value: Option<u8>) -> Self {
        Self {
            segment_type: DateSegmentType::Second,
            text: value.map_or_else(|| "--".to_string(), |v| format!("{v:02}")),
            value: value.map(i32::from),
            min_value: Some(0),
            max_value: Some(59),
            is_editable: true,
            is_placeholder: value.is_none(),
        }
    }

    /// Creates an AM/PM segment.
    pub fn day_period(is_pm: Option<bool>) -> Self {
        Self {
            segment_type: DateSegmentType::DayPeriod,
            text: match is_pm {
                Some(true) => "PM".to_string(),
                Some(false) | None => "AM".to_string(),
            },
            value: is_pm.map(i32::from),
            min_value: Some(0),
            max_value: Some(1),
            is_editable: true,
            is_placeholder: is_pm.is_none(),
        }
    }
}

/// Input parameters for the `use_date_segment` hook.
#[derive(Debug, Clone)]
pub struct UseDateSegmentInput {
    /// The segment data.
    pub segment: DateSegment,

    /// Whether the segment is focused.
    pub is_focused: Signal<bool>,

    /// Whether the field is disabled.
    pub is_disabled: Signal<bool>,

    /// Whether the field is read-only.
    pub is_read_only: Signal<bool>,

    /// Callback when the segment value changes (typed digit or AM/PM).
    pub on_change: Option<Callback<i32>>,

    /// Callback when increment is requested (`ArrowUp`).
    pub on_increment: Option<Callback<()>>,

    /// Callback when decrement is requested (`ArrowDown`).
    pub on_decrement: Option<Callback<()>>,

    /// Callback to focus the next segment.
    pub on_focus_next: Option<Callback<()>>,

    /// Callback to focus the previous segment.
    pub on_focus_previous: Option<Callback<()>>,

    /// Callback to clear the segment (Backspace/Delete).
    pub on_clear: Option<Callback<()>>,

    /// Callback for page-up increment.
    pub on_increment_page: Option<Callback<()>>,

    /// Callback for page-down decrement.
    pub on_decrement_page: Option<Callback<()>>,

    /// Callback to set segment to max (End key).
    pub on_increment_to_max: Option<Callback<()>>,

    /// Callback to set segment to min (Home key).
    pub on_decrement_to_min: Option<Callback<()>>,

    /// Callback when segment loses focus.
    pub on_blur: Option<Callback<()>>,

    /// Whether the field is invalid (for aria-invalid on the segment).
    pub is_invalid: Signal<bool>,
}

/// The return value of the `use_date_segment` hook.
#[derive(Debug)]
pub struct UseDateSegmentReturn {
    /// Props for the segment element. Call `.into_attrs()` for view spreading.
    pub segment_props: UseDateSegmentProps,

    /// The segment data.
    pub segment: DateSegment,
}

/// Props from `use_date_segment` for the segment element.
#[derive(Debug)]
pub struct UseDateSegmentProps {
    pub role: AriaRole,
    pub tabindex: Signal<&'static str>,
    pub aria_label: &'static str,
    pub aria_valuenow: Signal<Option<String>>,
    pub aria_valuemin: Signal<Option<String>>,
    pub aria_valuemax: Signal<Option<String>>,
    pub aria_valuetext: Signal<String>,
    pub aria_readonly: Signal<Option<AriaReadonly>>,
    pub aria_disabled: Signal<Option<AriaDisabled>>,
    pub aria_invalid: Signal<Option<AriaInvalid>>,
    pub content_editable: Option<&'static str>,
    pub input_mode: Option<&'static str>,
    pub data_placeholder: Signal<Option<&'static str>>,
    pub on_keydown: EventHandler<KeyboardEvent>,
    pub on_focus: EventHandler<FocusEvent>,
    pub on_blur: EventHandler<FocusEvent>,
}

impl IntoAttrs for UseDateSegmentProps {
    type Attrs = UseDateSegmentAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Role, self.role),
            Attr(attr::Tabindex, self.tabindex),
            Attr(attr::AriaLabel, self.aria_label),
            Attr(attr::AriaValuenow, self.aria_valuenow),
            Attr(attr::AriaValuemin, self.aria_valuemin),
            Attr(attr::AriaValuemax, self.aria_valuemax),
            Attr(attr::AriaValuetext, self.aria_valuetext),
            Attr(attr::AriaReadonly, self.aria_readonly),
            Attr(attr::AriaDisabled, self.aria_disabled),
            Attr(attr::AriaInvalid, self.aria_invalid),
            self.on_keydown.into_on(ev::keydown),
            self.on_focus.into_on(ev::focus),
            self.on_blur.into_on(ev::blur),
        )
    }
}

/// Attributes for the date segment element.
pub type UseDateSegmentAttrs = (
    Attr<attr::Role, AriaRole>,
    Attr<attr::Tabindex, Signal<&'static str>>,
    Attr<attr::AriaLabel, &'static str>,
    Attr<attr::AriaValuenow, Signal<Option<String>>>,
    Attr<attr::AriaValuemin, Signal<Option<String>>>,
    Attr<attr::AriaValuemax, Signal<Option<String>>>,
    Attr<attr::AriaValuetext, Signal<String>>,
    Attr<attr::AriaReadonly, Signal<Option<AriaReadonly>>>,
    Attr<attr::AriaDisabled, Signal<Option<AriaDisabled>>>,
    Attr<attr::AriaInvalid, Signal<Option<AriaInvalid>>>,
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
    On<ev::focus, SharedEventCallback<FocusEvent>>,
    On<ev::blur, SharedEventCallback<FocusEvent>>,
);

/// Provides the behavior and accessibility for a date segment.
///
/// A date segment is a single editable part of a date field (year, month, day, etc.).
/// Implements digit accumulation, keyboard navigation, and ARIA spinbutton semantics.
///
/// # Example
///
/// ```ignore
/// let segment = use_date_segment(UseDateSegmentInput {
///     segment: DateSegment::month(Some(3)),
///     is_focused: Signal::derive(|| false),
///     is_disabled: Signal::derive(|| false),
///     is_read_only: Signal::derive(|| false),
///     on_increment: Some(Callback::new(|_| { /* increment month */ })),
///     on_decrement: Some(Callback::new(|_| { /* decrement month */ })),
///     ..Default::default()
/// });
///
/// view! {
///     <span {..segment.segment_props.into_attrs()}>
///         {segment.segment.text.clone()}
///     </span>
/// }
/// ```
#[allow(clippy::too_many_lines)]
pub fn use_date_segment(input: UseDateSegmentInput) -> UseDateSegmentReturn {
    let UseDateSegmentInput {
        segment,
        is_focused,
        is_disabled: disabled,
        is_read_only,
        on_change,
        on_increment,
        on_decrement,
        on_focus_next,
        on_focus_previous,
        on_clear,
        on_increment_page,
        on_decrement_page,
        on_increment_to_max,
        on_decrement_to_min,
        on_blur,
        is_invalid,
    } = input;
    let segment = segment.clone();
    let is_editable = segment.is_editable;
    let segment_type = segment.segment_type;
    let max_digits = IncompleteDate::max_digits(segment_type);

    // Digit accumulation buffer.
    let entered_keys = StoredValue::new(String::new());

    // Segment value/text stored for reactive ARIA attributes.
    let seg_value = segment.value;
    let seg_text = segment.text.clone();
    let seg_min = segment.min_value;
    let seg_max = segment.max_value;
    let seg_is_placeholder = segment.is_placeholder;

    // Compute tabindex: 0 if focused and editable, -1 otherwise.
    let tabindex = Signal::derive(move || {
        if is_focused.get() && is_editable {
            "0"
        } else {
            "-1"
        }
    });

    // Reactive ARIA attributes.
    let aria_readonly =
        Signal::derive(move || (is_read_only.get() || !is_editable).then_some(AriaReadonly::True));
    let aria_disabled = Signal::derive(move || disabled.get().then_some(AriaDisabled::True));
    let aria_invalid_signal = Signal::derive(move || is_invalid.get().then_some(AriaInvalid::True));

    let aria_valuenow = Signal::derive(move || seg_value.map(|v| v.to_string()));
    let aria_valuemin = Signal::derive(move || seg_min.map(|v| v.to_string()));
    let aria_valuemax = Signal::derive(move || seg_max.map(|v| v.to_string()));
    let seg_text_clone = seg_text.clone();
    let aria_valuetext = Signal::derive(move || seg_text_clone.clone());

    let data_placeholder = Signal::derive(move || {
        if seg_is_placeholder {
            Some("true")
        } else {
            None
        }
    });

    let aria_label = segment_type.aria_label();

    // Content editable and input mode for editable segments.
    let content_editable = if is_editable { Some("true") } else { None };
    let input_mode = if !is_editable {
        None
    } else if segment_type == DateSegmentType::DayPeriod {
        Some("text")
    } else {
        Some("numeric")
    };

    // Role depends on whether segment is editable.
    let role = if is_editable {
        AriaRole::Spinbutton
    } else {
        AriaRole::Presentation
    };

    // ---- Keyboard handler ----
    let handle_keydown = move |e: KeyboardEvent| {
        if disabled.get_untracked() || is_read_only.get_untracked() || !is_editable {
            return;
        }

        let key = e.key();

        match key.as_str() {
            "ArrowUp" => {
                e.prevent_default();
                if let Some(cb) = on_increment {
                    cb.run(());
                }
            }
            "ArrowDown" => {
                e.prevent_default();
                if let Some(cb) = on_decrement {
                    cb.run(());
                }
            }
            "ArrowRight" => {
                e.prevent_default();
                if let Some(cb) = on_focus_next {
                    cb.run(());
                }
            }
            "ArrowLeft" => {
                e.prevent_default();
                if let Some(cb) = on_focus_previous {
                    cb.run(());
                }
            }
            "PageUp" => {
                e.prevent_default();
                if let Some(cb) = on_increment_page {
                    cb.run(());
                }
            }
            "PageDown" => {
                e.prevent_default();
                if let Some(cb) = on_decrement_page {
                    cb.run(());
                }
            }
            "Home" => {
                e.prevent_default();
                if let Some(cb) = on_decrement_to_min {
                    cb.run(());
                }
            }
            "End" => {
                e.prevent_default();
                if let Some(cb) = on_increment_to_max {
                    cb.run(());
                }
            }
            "Backspace" => {
                e.prevent_default();
                let keys = entered_keys.get_value();
                if keys.is_empty() {
                    // No accumulated digits: clear the segment and focus previous.
                    if seg_is_placeholder {
                        // Already placeholder, just focus previous.
                        if let Some(cb) = on_focus_previous {
                            cb.run(());
                        }
                    } else if let Some(cb) = on_clear {
                        cb.run(());
                    }
                } else {
                    // Remove last accumulated digit.
                    let mut new_keys = keys;
                    new_keys.pop();
                    if new_keys.is_empty() {
                        entered_keys.set_value(String::new());
                        if let Some(cb) = on_clear {
                            cb.run(());
                        }
                    } else if let Ok(val) = new_keys.parse::<i32>() {
                        entered_keys.set_value(new_keys);
                        if let Some(cb) = on_change {
                            cb.run(val);
                        }
                    }
                }
            }
            "Delete" => {
                e.prevent_default();
                entered_keys.set_value(String::new());
                if let Some(cb) = on_clear {
                    cb.run(());
                }
            }
            "a" | "A" if segment_type == DateSegmentType::DayPeriod => {
                e.prevent_default();
                if let Some(cb) = on_change {
                    cb.run(0); // AM
                }
            }
            "p" | "P" if segment_type == DateSegmentType::DayPeriod => {
                e.prevent_default();
                if let Some(cb) = on_change {
                    cb.run(1); // PM
                }
            }
            digit
                if digit.len() == 1 && digit.chars().next().is_some_and(|c| c.is_ascii_digit()) =>
            {
                e.prevent_default();
                handle_digit_input(
                    digit,
                    &entered_keys,
                    max_digits,
                    seg_max.unwrap_or(i32::MAX),
                    on_change,
                    on_focus_next,
                );
            }
            _ => {}
        }
    };

    // ---- Focus handler ----
    let handle_focus = move |_e: FocusEvent| {
        // Clear digit accumulation on focus.
        entered_keys.set_value(String::new());
    };

    // ---- Blur handler ----
    let handle_blur = move |_e: FocusEvent| {
        entered_keys.set_value(String::new());
        if let Some(cb) = on_blur {
            cb.run(());
        }
    };

    UseDateSegmentReturn {
        segment_props: UseDateSegmentProps {
            role,
            tabindex,
            aria_label,
            aria_valuenow,
            aria_valuemin,
            aria_valuemax,
            aria_valuetext,
            aria_readonly,
            aria_disabled,
            aria_invalid: aria_invalid_signal,
            content_editable,
            input_mode,
            data_placeholder,
            on_keydown: EventHandler::new(handle_keydown),
            on_focus: EventHandler::new(handle_focus),
            on_blur: EventHandler::new(handle_blur),
        },
        segment,
    }
}

/// Handle digit input with accumulation and auto-advance.
fn handle_digit_input(
    digit: &str,
    entered_keys: &StoredValue<String>,
    max_digits: usize,
    max_value: i32,
    on_change: Option<Callback<i32>>,
    on_focus_next: Option<Callback<()>>,
) {
    let mut keys = entered_keys.get_value();
    keys.push_str(digit);

    if let Ok(parsed) = keys.parse::<i32>() {
        if let Some(cb) = on_change {
            cb.run(parsed);
        }

        // Auto-advance: if adding another digit would exceed max, or we've
        // reached max digits, advance to next segment.
        let would_exceed = parsed.checked_mul(10).is_some_and(|v| v > max_value);
        if would_exceed || keys.len() >= max_digits {
            entered_keys.set_value(String::new());
            if let Some(cb) = on_focus_next {
                cb.run(());
            }
        } else {
            entered_keys.set_value(keys);
        }
    }
}

impl Default for UseDateSegmentInput {
    fn default() -> Self {
        Self {
            segment: DateSegment::literal(""),
            is_focused: Signal::derive(|| false),
            is_disabled: Signal::derive(|| false),
            is_read_only: Signal::derive(|| false),
            on_change: None,
            on_increment: None,
            on_decrement: None,
            on_focus_next: None,
            on_focus_previous: None,
            on_clear: None,
            on_increment_page: None,
            on_decrement_page: None,
            on_increment_to_max: None,
            on_decrement_to_min: None,
            on_blur: None,
            is_invalid: Signal::derive(|| false),
        }
    }
}
