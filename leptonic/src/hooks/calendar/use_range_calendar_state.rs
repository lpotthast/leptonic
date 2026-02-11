use leptos::prelude::*;

use super::use_range_calendar::DateRange;

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// No intentional deviations from the react-aria implementation.
//
// =============================================================================

/// State for managing range calendar selection.
#[derive(Clone, Copy)]
pub struct UseRangeCalendarStateReturn {
    /// The current selected range.
    pub value: Signal<DateRange>,

    /// Set the selected range.
    pub set_value: Callback<DateRange>,

    /// Clear the selection.
    pub clear: Callback<()>,
}

/// Creates internal state for a range calendar.
pub fn use_range_calendar_state(default_value: Option<DateRange>) -> UseRangeCalendarStateReturn {
    let (value, set_value) = signal(default_value.unwrap_or_default());

    UseRangeCalendarStateReturn {
        value: value.into(),
        set_value: Callback::new(move |range| set_value.set(range)),
        clear: Callback::new(move |_| set_value.set(DateRange::empty())),
    }
}
