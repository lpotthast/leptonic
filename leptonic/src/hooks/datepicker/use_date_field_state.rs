use leptos::prelude::*;

use super::{
    incomplete_date::IncompleteDate,
    use_date_segment::{DateSegment, DateSegmentType},
};
use crate::{
    hooks::form::use_form_validation_state::{
        UseFormValidationStateInput, UseFormValidationStateReturn, ValidateFn, ValidationBehavior,
        use_form_validation_state,
    },
    utils::time::whole_days_in,
};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-stately/datepicker/src/useDateFieldState.ts

//
// DIFFERENT BEHAVIOR
// - Hook-owned state: The `IncompleteDate` editing buffer is created and owned
//   internally. Callers get read-only `Signal`s and semantic mutation callbacks
//   (`set_segment`, `increment`, etc.). This replaces React-aria's mutable
//   state object returned from `useDateFieldState`.
//
// LEPTOS-SPECIFIC ADAPTATIONS
// - Uses `StoredValue<IncompleteDate>` + `Trigger` for the editing buffer
//   instead of React's `useState`.
// - Validation via `use_form_validation_state` following the leptonic pattern.
//

/// Input parameters for [`use_date_field_state`].
pub struct UseDateFieldStateInput {
    /// The current date value (controlled).
    pub value: Signal<Option<time::OffsetDateTime>>,

    /// The default value to restore on form reset.
    pub default_value: Option<time::OffsetDateTime>,

    /// The minimum allowed date.
    pub min: Option<time::OffsetDateTime>,

    /// The maximum allowed date.
    pub max: Option<time::OffsetDateTime>,

    /// Callback when the value changes.
    pub on_change: Option<Callback<Option<time::OffsetDateTime>>>,

    /// Whether to show the time portion.
    pub show_time: bool,

    /// Whether to use 24-hour format.
    pub hour_cycle_24: bool,

    /// Whether the field is disabled.
    pub is_disabled: Signal<bool>,

    /// Whether the field is read-only.
    pub is_read_only: Signal<bool>,

    /// Whether the field is required.
    pub is_required: bool,

    /// Custom client-side validation function.
    pub validate: Option<ValidateFn<Option<time::OffsetDateTime>>>,

    /// Whether the field is explicitly marked as invalid (controlled validation).
    pub is_invalid: Option<Signal<bool>>,

    /// Validation behavior mode.
    pub validation_behavior: ValidationBehavior,

    /// The field's name for form validation context matching.
    pub name: Option<String>,
}

/// Return value of [`use_date_field_state`].
#[derive(Clone, Copy)]
pub struct UseDateFieldStateReturn {
    /// The current date value.
    pub value: Signal<Option<time::OffsetDateTime>>,

    /// The segments to render.
    pub segments: Signal<Vec<DateSegment>>,

    /// Form validation state.
    pub validation: UseFormValidationStateReturn,

    /// Set a segment to a specific value.
    pub set_segment: Callback<(DateSegmentType, i32)>,

    /// Clear a segment (Backspace/Delete).
    pub clear_segment: Callback<DateSegmentType>,

    /// Increment a segment by 1.
    pub increment: Callback<DateSegmentType>,

    /// Decrement a segment by 1.
    pub decrement: Callback<DateSegmentType>,

    /// Increment a segment by page step.
    pub increment_page: Callback<DateSegmentType>,

    /// Decrement a segment by page step.
    pub decrement_page: Callback<DateSegmentType>,

    /// Set a segment to its maximum value.
    pub increment_to_max: Callback<DateSegmentType>,

    /// Set a segment to its minimum value.
    pub decrement_to_min: Callback<DateSegmentType>,

    /// Called on blur: if complete, emit the value; if all cleared, emit None.
    pub confirm_placeholder: Callback<()>,
}

/// Provides state management for a date field.
///
/// Separates state logic from ARIA/DOM concerns. Manages the `IncompleteDate`
/// editing buffer, segment generation, increment/decrement, and form validation.
#[allow(clippy::too_many_lines)]
pub fn use_date_field_state(input: UseDateFieldStateInput) -> UseDateFieldStateReturn {
    let UseDateFieldStateInput {
        value,
        default_value,
        min,
        max,
        on_change,
        show_time,
        hour_cycle_24,
        is_disabled,
        is_read_only,
        is_required: _,
        validate,
        is_invalid,
        validation_behavior,
        name,
    } = input;

    // ---- Internal editing buffer ----
    let initial_incomplete = value.get_untracked().map_or_else(
        || IncompleteDate::empty(hour_cycle_24),
        |d| IncompleteDate::from_date(&d, hour_cycle_24),
    );
    let display_value = StoredValue::new(initial_incomplete);

    // Default placeholder is now_utc or default_value.
    let placeholder = StoredValue::new(default_value.unwrap_or_else(time::OffsetDateTime::now_utc));

    // Trigger to notify segment derivation when display_value changes.
    let display_trigger = Trigger::new();

    // Sync display_value when the external controlled value changes.
    Effect::new(move |_| {
        let ext = value.get();
        match ext {
            Some(date) => {
                display_value.update_value(|dv| dv.sync_from_date(&date));
            }
            None => {
                display_value.set_value(IncompleteDate::empty(hour_cycle_24));
            }
        }
        display_trigger.notify();
    });

    // ---- Form validation ----
    let validation = use_form_validation_state(UseFormValidationStateInput {
        is_invalid,
        value,
        validate,
        validation_behavior,
        name,
    });

    // ---- Emit helper ----
    let emit = move |new_value: Option<time::OffsetDateTime>| {
        if let Some(on_change) = on_change {
            on_change.run(new_value);
        }
    };

    let try_emit_if_complete = move || {
        let dv = display_value.get_value();
        if dv.is_complete(show_time) {
            let ph = placeholder.get_value();
            if let Some(date) = dv.to_date(&ph) {
                let date = clamp_date(date, min, max);
                emit(Some(date));
            }
        } else if dv.is_cleared(show_time) {
            emit(None);
        }
    };

    // ---- Mutation callbacks ----
    let set_segment = Callback::new(move |(seg_type, val): (DateSegmentType, i32)| {
        if is_disabled.get_untracked() || is_read_only.get_untracked() {
            return;
        }
        display_value.update_value(|dv| dv.set(seg_type, val));
        display_trigger.notify();
        try_emit_if_complete();
    });

    let clear_segment = Callback::new(move |seg_type: DateSegmentType| {
        if is_disabled.get_untracked() || is_read_only.get_untracked() {
            return;
        }
        display_value.update_value(|dv| dv.clear(seg_type));
        display_trigger.notify();
        // If all fields are now cleared, emit None.
        if display_value.get_value().is_cleared(show_time) {
            emit(None);
        }
    });

    let increment = Callback::new(move |seg_type: DateSegmentType| {
        if is_disabled.get_untracked() || is_read_only.get_untracked() {
            return;
        }
        let ph = placeholder.get_value();
        display_value.update_value(|dv| dv.cycle(seg_type, 1, &ph));
        display_trigger.notify();
        try_emit_if_complete();
    });

    let decrement = Callback::new(move |seg_type: DateSegmentType| {
        if is_disabled.get_untracked() || is_read_only.get_untracked() {
            return;
        }
        let ph = placeholder.get_value();
        display_value.update_value(|dv| dv.cycle(seg_type, -1, &ph));
        display_trigger.notify();
        try_emit_if_complete();
    });

    let increment_page = Callback::new(move |seg_type: DateSegmentType| {
        if is_disabled.get_untracked() || is_read_only.get_untracked() {
            return;
        }
        let ph = placeholder.get_value();
        let step = IncompleteDate::page_step(seg_type);
        display_value.update_value(|dv| dv.cycle(seg_type, step, &ph));
        display_trigger.notify();
        try_emit_if_complete();
    });

    let decrement_page = Callback::new(move |seg_type: DateSegmentType| {
        if is_disabled.get_untracked() || is_read_only.get_untracked() {
            return;
        }
        let ph = placeholder.get_value();
        let step = IncompleteDate::page_step(seg_type);
        display_value.update_value(|dv| dv.cycle(seg_type, -step, &ph));
        display_trigger.notify();
        try_emit_if_complete();
    });

    let increment_to_max = Callback::new(move |seg_type: DateSegmentType| {
        if is_disabled.get_untracked() || is_read_only.get_untracked() {
            return;
        }
        let ph = placeholder.get_value();
        display_value.update_value(|dv| dv.set_to_max(seg_type, &ph));
        display_trigger.notify();
        try_emit_if_complete();
    });

    let decrement_to_min = Callback::new(move |seg_type: DateSegmentType| {
        if is_disabled.get_untracked() || is_read_only.get_untracked() {
            return;
        }
        let ph = placeholder.get_value();
        display_value.update_value(|dv| dv.set_to_min(seg_type, &ph));
        display_trigger.notify();
        try_emit_if_complete();
    });

    let confirm_placeholder = Callback::new(move |()| {
        let dv = display_value.get_value();
        if dv.is_complete(show_time) {
            let ph = placeholder.get_value();
            if let Some(date) = dv.to_date(&ph) {
                let date = clamp_date(date, min, max);
                emit(Some(date));
            }
        } else if dv.is_cleared(show_time) {
            emit(None);
        }
        // If partially filled, leave as-is (react-aria behavior).
    });

    // ---- Segment generation ----
    let segments = Signal::derive(move || {
        display_trigger.track();
        let dv = display_value.get_value();
        let ph = placeholder.get_value();
        build_segments(&dv, &ph, show_time, hour_cycle_24)
    });

    UseDateFieldStateReturn {
        value,
        segments,
        validation,
        set_segment,
        clear_segment,
        increment,
        decrement,
        increment_page,
        decrement_page,
        increment_to_max,
        decrement_to_min,
        confirm_placeholder,
    }
}

/// Build the segment list from the incomplete date and placeholder.
fn build_segments(
    dv: &IncompleteDate,
    placeholder: &time::OffsetDateTime,
    show_time: bool,
    hour_cycle_24: bool,
) -> Vec<DateSegment> {
    let year = dv.year.unwrap_or_else(|| placeholder.year());
    let month_num = dv.month.unwrap_or_else(|| placeholder.month() as u8);
    let max_day = month_from_u8(month_num).map_or(31, |m| whole_days_in(year, m));

    let mut segs = vec![
        DateSegment::year(dv.year),
        DateSegment::literal("-"),
        DateSegment::month(dv.month),
        DateSegment::literal("-"),
        DateSegment::day(dv.day, max_day),
    ];

    if show_time {
        segs.push(DateSegment::literal(" "));

        // Hour
        segs.push(DateSegment::hour(dv.hour, hour_cycle_24));
        segs.push(DateSegment::literal(":"));

        // Minute
        segs.push(DateSegment::minute(dv.minute));

        if !hour_cycle_24 {
            segs.push(DateSegment::literal(" "));
            segs.push(DateSegment::day_period(dv.day_period));
        }
    }

    segs
}

/// Clamp a date to the optional min/max range.
fn clamp_date(
    date: time::OffsetDateTime,
    min: Option<time::OffsetDateTime>,
    max: Option<time::OffsetDateTime>,
) -> time::OffsetDateTime {
    let mut result = date;
    if let Some(min) = min {
        if result < min {
            result = min;
        }
    }
    if let Some(max) = max {
        if result > max {
            result = max;
        }
    }
    result
}

/// Convert a 1-12 number to `time::Month`.
fn month_from_u8(m: u8) -> Option<time::Month> {
    match m {
        1 => Some(time::Month::January),
        2 => Some(time::Month::February),
        3 => Some(time::Month::March),
        4 => Some(time::Month::April),
        5 => Some(time::Month::May),
        6 => Some(time::Month::June),
        7 => Some(time::Month::July),
        8 => Some(time::Month::August),
        9 => Some(time::Month::September),
        10 => Some(time::Month::October),
        11 => Some(time::Month::November),
        12 => Some(time::Month::December),
        _ => None,
    }
}
