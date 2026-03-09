use leptos::{
    attr,
    attr::Attr,
    ev,
    ev::{On, SharedEventCallback},
    prelude::*,
};
use web_sys::{FocusEvent, KeyboardEvent};

use crate::{
    hooks::IntoAttrs,
    utils::{
        aria::{AriaDisabled, AriaReadonly, AriaRole},
        EventHandler,
    },
};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/datepicker/src/useDateSegment.ts

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// No intentional deviations from the react-aria implementation.
//
// =============================================================================

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

    /// Callback when the segment value changes.
    pub on_change: Option<Callback<i32>>,

    /// Callback when increment is requested.
    pub on_increment: Option<Callback<()>>,

    /// Callback when decrement is requested.
    pub on_decrement: Option<Callback<()>>,

    /// Callback to focus the next segment.
    pub on_focus_next: Option<Callback<()>>,

    /// Callback to focus the previous segment.
    pub on_focus_previous: Option<Callback<()>>,
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
    pub aria_valuenow: Option<String>,
    pub aria_valuemin: Option<String>,
    pub aria_valuemax: Option<String>,
    pub aria_valuetext: String,
    pub aria_readonly: Signal<Option<AriaReadonly>>,
    pub aria_disabled: Signal<Option<AriaDisabled>>,
    pub on_keydown: EventHandler<KeyboardEvent>,
    pub on_focus: EventHandler<FocusEvent>,
}

impl IntoAttrs for UseDateSegmentProps {
    type Attrs = UseDateSegmentAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Role, self.role),
            Attr(attr::Tabindex, self.tabindex),
            Attr(attr::AriaValuenow, self.aria_valuenow),
            Attr(attr::AriaValuemin, self.aria_valuemin),
            Attr(attr::AriaValuemax, self.aria_valuemax),
            Attr(attr::AriaValuetext, self.aria_valuetext),
            Attr(attr::AriaReadonly, self.aria_readonly),
            Attr(attr::AriaDisabled, self.aria_disabled),
            self.on_keydown.into_on(ev::keydown),
            self.on_focus.into_on(ev::focus),
        )
    }
}

/// Attributes for the date segment element.
pub type UseDateSegmentAttrs = (
    Attr<attr::Role, AriaRole>,
    Attr<attr::Tabindex, Signal<&'static str>>,
    Attr<attr::AriaValuenow, Option<String>>,
    Attr<attr::AriaValuemin, Option<String>>,
    Attr<attr::AriaValuemax, Option<String>>,
    Attr<attr::AriaValuetext, String>,
    Attr<attr::AriaReadonly, Signal<Option<AriaReadonly>>>,
    Attr<attr::AriaDisabled, Signal<Option<AriaDisabled>>>,
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
    On<ev::focus, SharedEventCallback<FocusEvent>>,
);

/// Provides the behavior and accessibility for a date segment.
///
/// A date segment is a single editable part of a date field (year, month, day, etc.).
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
///
/// # Panics
///
/// Panics if a digit character cannot be parsed as an integer (should not happen
/// since the input is validated to be an ASCII digit).
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
    } = input;
    let segment = segment.clone();
    let is_editable = segment.is_editable;

    // Compute tabindex
    let tabindex = Signal::derive(move || {
        if is_focused.get() && is_editable {
            "0"
        } else {
            "-1"
        }
    });

    // Compute aria-readonly
    let aria_readonly =
        Signal::derive(move || (is_read_only.get() || !is_editable).then_some(AriaReadonly::True));

    // Compute aria-disabled
    let aria_disabled = Signal::derive(move || disabled.get().then_some(AriaDisabled::True));

    // ARIA value attributes
    let aria_valuenow = segment.value.map(|v| v.to_string());
    let aria_valuemin = segment.min_value.map(|v| v.to_string());
    let aria_valuemax = segment.max_value.map(|v| v.to_string());
    let aria_valuetext = segment.text.clone();

    // Handle keyboard input
    let segment_type = segment.segment_type;

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
                let digit_val: i32 = digit.parse().unwrap();
                if let Some(cb) = on_change {
                    cb.run(digit_val);
                }
            }
            _ => {}
        }
    };

    // Handle focus
    let handle_focus = move |_e: FocusEvent| {
        // Focus handling is managed by parent
    };

    // Role depends on whether segment is editable
    let role = if is_editable {
        AriaRole::Spinbutton
    } else {
        AriaRole::Presentation
    };

    UseDateSegmentReturn {
        segment_props: UseDateSegmentProps {
            role,
            tabindex,
            aria_valuenow,
            aria_valuemin,
            aria_valuemax,
            aria_valuetext,
            aria_readonly,
            aria_disabled,
            on_keydown: EventHandler::new(handle_keydown),
            on_focus: EventHandler::new(handle_focus),
        },
        segment,
    }
}
