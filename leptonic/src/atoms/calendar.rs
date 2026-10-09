// Upstream: react-aria-components/src/Calendar.tsx @ 99e6102368
// Upstream: react-aria-components/test/Calendar.test.js @ 99e6102368
// Upstream: react-aria-components/test/RangeCalendar.test.tsx @ 99e6102368
//! Headless calendar atoms: a calendar and a range calendar with their
//! heading, buttons, grids and cells.

use jiff::civil::{Date, Weekday};
use leptos::{context::Provider, prelude::*};
use leptos_classes::Classes;

use crate::{
    CapturedElement, IntoAttrs, Out, ValueBinding,
    hooks::{
        button::{UseButtonInput, UseButtonReturn, use_button},
        calendar::{
            CalendarData, CalendarHeadingFormat, CalendarStates, CalendarYearPickerFormat,
            CommitBehavior, DateAvailabilityQuery, PageBehavior, SelectionAlignment,
            UseCalendarCellInput, UseCalendarCellReturn, UseCalendarGridInput,
            UseCalendarGridReturn, UseCalendarHeadingInput, UseCalendarInput,
            UseCalendarMonthPickerInput, UseCalendarPickerReturn, UseCalendarReturn,
            UseCalendarStateInput, UseCalendarYearPickerInput, UseRangeCalendarInput,
            UseRangeCalendarStateInput, use_calendar, use_calendar_cell, use_calendar_grid,
            use_calendar_heading, use_calendar_month_picker, use_calendar_state,
            use_calendar_year_picker, use_range_calendar, use_range_calendar_state,
        },
        focus::{UseFocusRingInput, UseFocusRingReturn, use_focus_ring},
        interactions::{UseHoverInput, use_hover},
    },
    utils::{
        data_attributes::flag,
        date::{DateDuration, DateExt, DateRange},
        date_time_formatter::{DateTimeFormat, MonthFormat},
        default_class::with_default_class,
        slot_id::SlotProps,
        styles::Styles,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - One element per atom: the grid's parts are atoms of their own (`CalendarGridHeader`,
//   `CalendarHeaderRow`, `CalendarHeaderCell`, `CalendarGridBody`, `CalendarWeek`,
//   `CalendarCell`, `CalendarCellButton`; react-aria-components renders the rows and the cell's
//   button itself), and the previous/next buttons and the error message are atoms
//   (`CalendarPreviousButton`, `CalendarNextButton`, `CalendarErrorMessage`; react-aria-components:
//   slots of `Button` and `Text`).
// - State props per C4: `value` + `set_value`, `default_value`, `on_change`.
// - The layout props (`visible_duration`, `page_behavior`, `first_day_of_week`,
//   `selection_alignment`, `weeks_in_month`) are reactive (C11).
// - `CalendarMonthPicker`/`CalendarYearPicker` render their children from the picker's
//   `UseCalendarPickerReturn` (react-aria-components: a render function of its props).
//
// ## OMITTED FEATURES
// - The visually hidden heading before the grids and the hidden next button after them
//   (react-aria-components adds both for touch screen readers): they would be further elements.
// - A ref to the calendar's element (react-spectrum's `ref.focus()` moving the focus to the
//   focused date, "should support focusing via a ref"): atoms forward no element yet.
//
// =============================================================================

/// What a calendar's parts need from it.
#[derive(Clone)]
struct CalendarContext {
    data: CalendarData,
    // Cloned per rendering: the buttons may render again (e.g. after switching a view).
    previous: StoredValue<UseButtonInput>,
    next: StoredValue<UseButtonInput>,
    error_message: StoredValue<SlotProps>,
}

/// What a date picker gives the calendar in its popover (react-aria-components'
/// `CalendarContext`): its date, selection, limits and states replace the calendar's own.
#[derive(Clone, Copy)]
pub(crate) struct CalendarPickerContext {
    pub value: Signal<Option<Date>>,
    pub select: Callback<Date>,
    pub min_value: Signal<Option<Date>>,
    pub max_value: Signal<Option<Date>>,
    pub is_date_unavailable: Option<Callback<Date, bool>>,
    pub is_disabled: Signal<bool>,
    pub is_read_only: Signal<bool>,
    pub is_invalid: Signal<bool>,
    pub default_focused_value: Option<Date>,
}

/// What a date range picker gives the range calendar in its popover.
#[derive(Clone, Copy)]
pub(crate) struct RangeCalendarPickerContext {
    pub value: Signal<Option<DateRange>>,
    pub select: Callback<DateRange>,
    pub min_value: Signal<Option<Date>>,
    pub max_value: Signal<Option<Date>>,
    pub is_date_unavailable: Option<Callback<DateAvailabilityQuery, bool>>,
    pub allows_non_contiguous_ranges: bool,
    pub is_disabled: Signal<bool>,
    pub is_read_only: Signal<bool>,
    pub is_invalid: Signal<bool>,
    pub default_focused_value: Option<Date>,
}

/// What a grid's parts need from it.
#[derive(Clone, Copy)]
struct GridContext {
    start_date: Signal<Date>,
    week_days: Signal<Vec<String>>,
    weeks_in_month: Signal<u8>,
}

/// The state of the `Calendar` or `RangeCalendar` around, for custom parts (react-aria-components'
/// `CalendarStateContext`/`RangeCalendarStateContext`), e.g. month and year pickers
/// (`use_calendar_month_picker`).
pub fn use_calendar_states() -> Option<CalendarStates> {
    use_context::<CalendarContext>().map(|context| context.data.state)
}

fn expect_calendar(part: &str) -> Option<CalendarContext> {
    let context = use_context::<CalendarContext>();
    if context.is_none() {
        crate::utils::dev_warn!("{part}: not inside a Calendar or RangeCalendar");
    }
    context
}

/// A calendar: a month (or several) of dates to pick one from. Put a `CalendarHeading`, the
/// `CalendarPreviousButton`/`CalendarNextButton` and a `CalendarGrid` inside.
///
/// Data attributes: `data-disabled`, `data-invalid`.
///
/// Default class: `leptonic-Calendar`.
#[component]
#[allow(clippy::too_many_arguments)]
pub fn Calendar(
    /// The initially selected date.
    #[prop(optional)]
    default_value: Option<Date>,
    /// The selected date (controlled): a value or any signal.
    #[prop(into, optional)]
    value: Option<Signal<Option<Date>>>,
    /// Receives the selected date: an `RwSignal`, `WriteSignal`, closure, `Callback`, ...
    #[prop(into, optional)]
    set_value: Option<Out<Option<Date>>>,
    /// Called with the selected date.
    #[prop(into, optional)]
    on_change: Option<Callback<Option<Date>>>,
    #[prop(into, optional)] min_value: Signal<Option<Date>>,
    #[prop(into, optional)] max_value: Signal<Option<Date>>,
    /// Whether a date can't be selected (e.g. booked out).
    #[prop(into, optional)]
    is_date_unavailable: Option<Callback<Date, bool>>,
    #[prop(into, optional)] is_disabled: Signal<bool>,
    #[prop(into, optional)] is_read_only: Signal<bool>,
    #[prop(into, optional)] is_invalid: Signal<bool>,
    #[prop(optional)] auto_focus: bool,
    /// The initially focused date. Default: the value, else today.
    #[prop(optional)]
    default_focused_value: Option<Date>,
    /// The focused date (controlled): a value or any signal.
    #[prop(into, optional)]
    focused_value: Option<Signal<Date>>,
    /// Receives the focused date.
    #[prop(into, optional)]
    set_focused_value: Option<Out<Date>>,
    #[prop(into, optional)] on_focused_value_change: Option<Callback<Date>>,
    /// How much is visible at once. Default: one month. A change re-aligns the visible range.
    #[prop(into, default = Signal::stored(DateDuration::months(1)))]
    visible_duration: Signal<DateDuration>,
    /// How the previous and next buttons page. Default: by the visible duration.
    #[prop(into, optional)]
    page_behavior: Signal<PageBehavior>,
    /// The first day of the week. Default: the locale's.
    #[prop(into, optional)]
    first_day_of_week: MaybeProp<Weekday>,
    /// Where the focused date is placed when the visible range is aligned. Default: centered.
    #[prop(into, optional)]
    selection_alignment: Signal<SelectionAlignment>,
    /// A fixed number of week rows per month (e.g. 6, so the calendar keeps its height).
    #[prop(into, optional)]
    weeks_in_month: MaybeProp<u8>,
    /// The calendar's id. Generated when not given.
    #[prop(into, optional)]
    id: Option<String>,
    #[prop(into, optional)] aria_label: MaybeProp<String>,
    #[prop(into, optional)] aria_labelledby: Option<String>,
    #[prop(into, optional)] aria_describedby: Option<String>,
    #[prop(into, optional)] aria_details: Option<String>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let classes = with_default_class("leptonic-Calendar", classes);
    let (mut value, mut on_change) = ValueBinding::from_state_props(value, set_value, on_change);
    let (focused_value, on_focus_change) =
        ValueBinding::from_state_props(focused_value, set_focused_value, on_focused_value_change);
    // Inside a date picker: its date, selection, limits and states; focused on opening.
    let (
        mut min_value,
        mut max_value,
        mut is_date_unavailable,
        mut is_disabled,
        mut is_read_only,
        mut is_invalid,
        mut auto_focus,
        mut default_focused_value,
    ) = (
        min_value,
        max_value,
        is_date_unavailable,
        is_disabled,
        is_read_only,
        is_invalid,
        auto_focus,
        default_focused_value,
    );
    if let Some(picker) = use_context::<CalendarPickerContext>() {
        value = Some(ValueBinding::new(
            picker.value,
            Callback::new(move |date: Option<Date>| {
                if let Some(date) = date {
                    picker.select.run(date);
                }
            }),
        ));
        on_change = None;
        min_value = picker.min_value;
        max_value = picker.max_value;
        is_date_unavailable = picker.is_date_unavailable;
        is_disabled = picker.is_disabled;
        is_read_only = picker.is_read_only;
        is_invalid = picker.is_invalid;
        auto_focus = true;
        // The placeholder's month only without a date (else the date's).
        if picker.value.get_untracked().is_none() {
            default_focused_value = picker.default_focused_value.or(default_focused_value);
        }
    }
    let state = use_calendar_state(UseCalendarStateInput {
        default_value,
        value,
        on_change,
        min_value,
        max_value,
        is_date_unavailable,
        is_disabled,
        is_read_only,
        is_invalid,
        auto_focus,
        default_focused_value,
        focused_value,
        on_focus_change,
        visible_duration,
        page_behavior,
        first_day_of_week: Signal::derive(move || first_day_of_week.get()),
        selection_alignment,
        weeks_in_month: Signal::derive(move || weeks_in_month.get()),
    });
    let calendar = use_calendar(UseCalendarInput {
        state,
        id,
        aria_label,
        aria_labelledby,
        aria_describedby,
        aria_details,
    });
    render_calendar(calendar, is_disabled, classes, styles, children)
}

/// A range calendar: a month (or several) of dates to pick a range from, by two selections or
/// by dragging. Its parts are those of [`Calendar`].
///
/// Data attributes: `data-disabled`, `data-invalid`.
///
/// Default class: `leptonic-RangeCalendar`.
#[component]
#[allow(clippy::too_many_arguments)]
pub fn RangeCalendar(
    /// The initially selected range.
    #[prop(optional)]
    default_value: Option<DateRange>,
    /// The selected range (controlled): a value or any signal.
    #[prop(into, optional)]
    value: Option<Signal<Option<DateRange>>>,
    /// Receives the selected range.
    #[prop(into, optional)]
    set_value: Option<Out<Option<DateRange>>>,
    #[prop(into, optional)] on_change: Option<Callback<Option<DateRange>>>,
    #[prop(into, optional)] min_value: Signal<Option<Date>>,
    #[prop(into, optional)] max_value: Signal<Option<Date>>,
    /// Whether a date can't be selected, given the anchor of a range being selected.
    #[prop(into, optional)]
    is_date_unavailable: Option<Callback<DateAvailabilityQuery, bool>>,
    /// Whether a range may span unavailable dates.
    #[prop(optional)]
    allows_non_contiguous_ranges: bool,
    /// What a press outside the dates or leaving the calendar does with a range being selected.
    #[prop(optional)]
    commit_behavior: CommitBehavior,
    #[prop(into, optional)] is_disabled: Signal<bool>,
    #[prop(into, optional)] is_read_only: Signal<bool>,
    #[prop(into, optional)] is_invalid: Signal<bool>,
    #[prop(optional)] auto_focus: bool,
    #[prop(optional)] default_focused_value: Option<Date>,
    #[prop(into, optional)] focused_value: Option<Signal<Date>>,
    #[prop(into, optional)] set_focused_value: Option<Out<Date>>,
    #[prop(into, optional)] on_focused_value_change: Option<Callback<Date>>,
    /// How much is visible at once. Default: one month. A change re-aligns the visible range.
    #[prop(into, default = Signal::stored(DateDuration::months(1)))]
    visible_duration: Signal<DateDuration>,
    #[prop(into, optional)] page_behavior: Signal<PageBehavior>,
    /// The first day of the week. Default: the locale's.
    #[prop(into, optional)]
    first_day_of_week: MaybeProp<Weekday>,
    /// Default: centered, or the start if the range doesn't fit then.
    #[prop(into, optional)]
    selection_alignment: MaybeProp<SelectionAlignment>,
    #[prop(into, optional)] weeks_in_month: MaybeProp<u8>,
    #[prop(into, optional)] id: Option<String>,
    #[prop(into, optional)] aria_label: MaybeProp<String>,
    #[prop(into, optional)] aria_labelledby: Option<String>,
    #[prop(into, optional)] aria_describedby: Option<String>,
    #[prop(into, optional)] aria_details: Option<String>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let classes = with_default_class("leptonic-RangeCalendar", classes);
    let (mut value, mut on_change) = ValueBinding::from_state_props(value, set_value, on_change);
    let (focused_value, on_focus_change) =
        ValueBinding::from_state_props(focused_value, set_focused_value, on_focused_value_change);
    // Inside a date range picker: its range, selection, limits and states; focused on opening.
    let (
        mut min_value,
        mut max_value,
        mut is_date_unavailable,
        mut allows_non_contiguous_ranges,
        mut is_disabled,
        mut is_read_only,
        mut is_invalid,
        mut auto_focus,
        mut default_focused_value,
    ) = (
        min_value,
        max_value,
        is_date_unavailable,
        allows_non_contiguous_ranges,
        is_disabled,
        is_read_only,
        is_invalid,
        auto_focus,
        default_focused_value,
    );
    if let Some(picker) = use_context::<RangeCalendarPickerContext>() {
        allows_non_contiguous_ranges = picker.allows_non_contiguous_ranges;
        value = Some(ValueBinding::new(
            picker.value,
            Callback::new(move |range: Option<DateRange>| {
                if let Some(range) = range {
                    picker.select.run(range);
                }
            }),
        ));
        on_change = None;
        min_value = picker.min_value;
        max_value = picker.max_value;
        is_date_unavailable = picker.is_date_unavailable;
        is_disabled = picker.is_disabled;
        is_read_only = picker.is_read_only;
        is_invalid = picker.is_invalid;
        auto_focus = true;
        if picker.value.get_untracked().is_none() {
            default_focused_value = picker.default_focused_value.or(default_focused_value);
        }
    }
    let state = use_range_calendar_state(UseRangeCalendarStateInput {
        default_value,
        value,
        on_change,
        min_value,
        max_value,
        is_date_unavailable,
        allows_non_contiguous_ranges,
        is_disabled,
        is_read_only,
        is_invalid,
        auto_focus,
        default_focused_value,
        focused_value,
        on_focus_change,
        visible_duration,
        page_behavior,
        first_day_of_week: Signal::derive(move || first_day_of_week.get()),
        selection_alignment: Signal::derive(move || selection_alignment.get()),
        weeks_in_month: Signal::derive(move || weeks_in_month.get()),
    });
    let calendar = use_range_calendar(UseRangeCalendarInput {
        state,
        commit_behavior,
        id,
        aria_label,
        aria_labelledby,
        aria_describedby,
        aria_details,
    });
    render_calendar(calendar, is_disabled, classes, styles, children)
}

fn render_calendar(
    calendar: UseCalendarReturn,
    is_disabled: Signal<bool>,
    classes: Classes,
    styles: Styles,
    children: Children,
) -> AnyView {
    let UseCalendarReturn {
        calendar_props,
        previous_button,
        next_button,
        error_message_props,
        data,
        ..
    } = calendar;
    let is_invalid = data.state.is_value_invalid();
    let context = CalendarContext {
        data,
        previous: StoredValue::new(previous_button),
        next: StoredValue::new(next_button),
        error_message: StoredValue::new(error_message_props),
    };
    view! {
        <div
            {..calendar_props.into_attrs()}
            class=classes
            style=styles
            data-disabled=flag(is_disabled)
            data-invalid=flag(is_invalid)
        >
            <Provider value=context>{children()}</Provider>
        </div>
    }
    .into_any()
}

/// The heading of the calendar around it: its visible month, "May 2024" (hidden from assistive
/// technology: the calendar's label says it). `offset` shows a later month of a calendar showing
/// several.
///
/// Default class: `leptonic-CalendarHeading`.
#[component]
pub fn CalendarHeading(
    #[prop(optional)] offset: DateDuration,
    #[prop(optional)] format: CalendarHeadingFormat,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
) -> impl IntoView {
    let classes = with_default_class("leptonic-CalendarHeading", classes);
    let Some(context) = expect_calendar("CalendarHeading") else {
        return ().into_any();
    };
    let heading = use_calendar_heading(UseCalendarHeadingInput {
        state: context.data.state,
        offset,
        format,
    });
    view! { <h2 class=classes style=styles aria-hidden="true">{heading}</h2> }.into_any()
}

/// A month picker of the calendar around it (react-aria-components' `CalendarMonthPicker`):
/// renders `children` with the year's months, the focused date's month and a setter moving the
/// focused date, e.g. into a `<select>`. Renders no element of its own.
#[component]
pub fn CalendarMonthPicker<F, V>(
    children: F,
    /// How the months are formatted. Default: short ("Jan").
    #[prop(optional)]
    format: Option<MonthFormat>,
) -> impl IntoView
where
    F: FnOnce(UseCalendarPickerReturn) -> V + 'static,
    V: IntoView + 'static,
{
    let Some(context) = expect_calendar("CalendarMonthPicker") else {
        return ().into_any();
    };
    children(use_calendar_month_picker(UseCalendarMonthPickerInput {
        state: context.data.state,
        format: format.unwrap_or(MonthFormat::Short),
    }))
    .into_any()
}

/// A year picker of the calendar around it (react-aria-components' `CalendarYearPicker`):
/// renders `children` with `visible_years` years around the focused date's (default 20, within
/// min and max), its year and a setter moving the focused date. Renders no element of its own.
#[component]
pub fn CalendarYearPicker<F, V>(
    children: F,
    /// How many years to offer. Default: 20.
    #[prop(optional)]
    visible_years: Option<u8>,
    #[prop(optional)] format: CalendarYearPickerFormat,
) -> impl IntoView
where
    F: FnOnce(UseCalendarPickerReturn) -> V + 'static,
    V: IntoView + 'static,
{
    let Some(context) = expect_calendar("CalendarYearPicker") else {
        return ().into_any();
    };
    children(use_calendar_year_picker(UseCalendarYearPickerInput {
        state: context.data.state,
        visible_years: visible_years.unwrap_or(20),
        format,
    }))
    .into_any()
}

/// A button of the calendar around it.
fn calendar_button(
    input: Option<UseButtonInput>,
    classes: Classes,
    styles: Styles,
    children: Children,
) -> AnyView {
    let Some(input) = input else {
        return ().into_any();
    };
    let UseButtonReturn {
        props,
        is_disabled,
        is_pressed,
        is_hovered,
        is_focused,
        ..
    } = use_button(input);
    let (attrs, button_styles) = props.into_parts();
    view! {
        <button
            {..attrs}
            class=classes
            style=button_styles.merge(styles)
            data-pressed=flag(is_pressed)
            data-hovered=flag(is_hovered)
            data-focused=flag(is_focused)
            data-disabled=flag(is_disabled)
        >
            {children()}
        </button>
    }
    .into_any()
}

/// The button showing the previous page of the calendar around it ("Previous").
///
/// Data attributes: `data-pressed`, `data-hovered`, `data-focused`, `data-focus-visible`,
/// `data-disabled`.
///
/// Default class: `leptonic-CalendarPreviousButton`.
#[component]
pub fn CalendarPreviousButton(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let classes = with_default_class("leptonic-CalendarPreviousButton", classes);
    let input =
        expect_calendar("CalendarPreviousButton").map(|context| context.previous.get_value());
    calendar_button(input, classes, styles, children)
}

/// The button showing the next page of the calendar around it ("Next").
///
/// Data attributes: `data-pressed`, `data-hovered`, `data-focused`, `data-focus-visible`,
/// `data-disabled`.
///
/// Default class: `leptonic-CalendarNextButton`.
#[component]
pub fn CalendarNextButton(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let classes = with_default_class("leptonic-CalendarNextButton", classes);
    let input = expect_calendar("CalendarNextButton").map(|context| context.next.get_value());
    calendar_button(input, classes, styles, children)
}

/// The error message of the calendar around it; invalid selected dates refer to it.
///
/// Default class: `leptonic-CalendarErrorMessage`.
#[component]
pub fn CalendarErrorMessage(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let classes = with_default_class("leptonic-CalendarErrorMessage", classes);
    let props =
        expect_calendar("CalendarErrorMessage").map(|context| context.error_message.get_value());
    let Some(props) = props else {
        return ().into_any();
    };
    view! { <div {..props.into_attrs()} class=classes style=styles>{children()}</div> }.into_any()
}

/// A grid of the calendar around it: a month of dates (`offset`: a later month of a calendar
/// showing several), a `table`. Put a `CalendarGridHeader` and a `CalendarGridBody` inside.
///
/// Default class: `leptonic-CalendarGrid`.
#[component]
pub fn CalendarGrid(
    #[prop(optional)] offset: DateDuration,
    /// How the weekday names are formatted. Default: narrow ("M").
    #[prop(optional)]
    weekday_style: Option<DateTimeFormat>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let classes = with_default_class("leptonic-CalendarGrid", classes);
    let Some(context) = expect_calendar("CalendarGrid") else {
        return ().into_any();
    };
    let calendar = context.data.state.calendar();
    let start_date = Signal::derive(move || calendar.visible_range.get().start.add(offset));
    // A month view's grid shows a month (react-aria-components: always).
    let end_date = Signal::derive(move || {
        let duration = calendar.visible_duration.get();
        if duration.days == 0 && duration.weeks == 0 {
            start_date.get().last_of_month()
        } else {
            calendar.visible_range.get().end
        }
    });
    let UseCalendarGridReturn {
        grid_props,
        start_date,
        week_days,
        weeks_in_month,
    } = use_calendar_grid(UseCalendarGridInput {
        start_date: Some(start_date),
        end_date: Some(end_date),
        weekday_style: weekday_style.unwrap_or(DateTimeFormat::Narrow),
        data: context.data.clone(),
    });
    let grid = GridContext {
        start_date,
        week_days,
        weeks_in_month,
    };
    view! {
        <table {..grid_props.into_attrs()} class=classes style=styles>
            <Provider value=grid>{children()}</Provider>
        </table>
    }
    .into_any()
}

/// The header of the grid around it (a `thead`, hidden from assistive technology: the cells'
/// labels name their weekdays). Put a `CalendarHeaderRow` inside.
///
/// Default class: `leptonic-CalendarGridHeader`.
#[component]
pub fn CalendarGridHeader(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let classes = with_default_class("leptonic-CalendarGridHeader", classes);
    view! {
        <thead class=classes style=styles aria-hidden="true">
            {children()}
        </thead>
    }
}

/// The row of weekday names of the grid around it (a `tr`), rendering `children` per weekday
/// (e.g. a `CalendarHeaderCell`).
///
/// Default class: `leptonic-CalendarHeaderRow`.
#[component]
pub fn CalendarHeaderRow<F, V>(
    children: F,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
) -> impl IntoView
where
    F: Fn(String) -> V + Send + Sync + 'static,
    V: IntoView + 'static,
{
    let classes = with_default_class("leptonic-CalendarHeaderRow", classes);
    let Some(grid) = use_context::<GridContext>() else {
        crate::utils::dev_warn!("CalendarHeaderRow: not inside a CalendarGrid");
        return ().into_any();
    };
    let children = std::sync::Arc::new(children);
    view! {
        <tr class=classes style=styles>
            <For
                each=move || grid.week_days.get().into_iter().enumerate()
                key=|(index, day)| (*index, day.clone())
                children=move |(_, day)| children(day)
            />
        </tr>
    }
    .into_any()
}

/// A weekday name of the grid's header (a `th`).
///
/// Default class: `leptonic-CalendarHeaderCell`.
#[component]
pub fn CalendarHeaderCell(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let classes = with_default_class("leptonic-CalendarHeaderCell", classes);
    view! { <th class=classes style=styles>{children()}</th> }
}

/// The body of the grid around it (a `tbody`), rendering `children` per week row (e.g. a
/// `CalendarWeek`).
///
/// Default class: `leptonic-CalendarGridBody`.
#[component]
pub fn CalendarGridBody<F, V>(
    children: F,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
) -> impl IntoView
where
    F: Fn(u8) -> V + Send + Sync + 'static,
    V: IntoView + 'static,
{
    let classes = with_default_class("leptonic-CalendarGridBody", classes);
    let Some(grid) = use_context::<GridContext>() else {
        crate::utils::dev_warn!("CalendarGridBody: not inside a CalendarGrid");
        return ().into_any();
    };
    let children = std::sync::Arc::new(children);
    view! {
        <tbody class=classes style=styles>
            <For
                each=move || 0..grid.weeks_in_month.get()
                key=|week| *week
                children=move |week| children(week)
            />
        </tbody>
    }
    .into_any()
}

/// A week row of the grid around it (a `tr`), rendering `children` per date (e.g. a
/// `CalendarCell`).
///
/// Default class: `leptonic-CalendarWeek`.
#[component]
pub fn CalendarWeek<F, V>(
    /// The week's index in the grid's month.
    week: u8,
    children: F,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
) -> impl IntoView
where
    F: Fn(Signal<Date>) -> V + Send + Sync + 'static,
    V: IntoView + 'static,
{
    let classes = with_default_class("leptonic-CalendarWeek", classes);
    let (Some(context), Some(grid)) = (
        expect_calendar("CalendarWeek"),
        use_context::<GridContext>(),
    ) else {
        return ().into_any();
    };
    let calendar = context.data.state.calendar();
    let children = std::sync::Arc::new(children);
    let dates = Memo::new(move |_| calendar.dates_in_week(week, Some(grid.start_date.get())));
    // Cells are keyed by their position, so that they (and the focus) stay when the calendar
    // pages; their dates change. Dates before the representable range (year -9999) are left out.
    view! {
        <tr class=classes style=styles>
            <For
                each=move || 0..dates.with(Vec::len)
                key=|position| *position
                children=move |position| {
                    let date = Memo::new(move |_| {
                        dates.with(|dates| dates.get(position).copied().flatten())
                    });
                    let has_date = Memo::new(move |_| date.with(Option::is_some));
                    let children = children.clone();
                    move || {
                        has_date
                            .get()
                            .then(|| children(Signal::derive(move || date.get().unwrap_or(Date::MIN))))
                    }
                }
            />
        </tr>
    }
    .into_any()
}

/// What a cell's button needs from its cell.
#[derive(Clone)]
struct CellContext {
    cell: StoredValue<Option<UseCalendarCellReturn>>,
    is_outside_month: Signal<bool>,
    is_selection_start: Signal<bool>,
    is_selection_end: Signal<bool>,
}

/// A date of the grid around it (a `td` with the grid cell role). Put a `CalendarCellButton`
/// inside.
///
/// Default class: `leptonic-CalendarCell`.
#[component]
pub fn CalendarCell(
    /// The cell's date: a value or any signal.
    #[prop(into)]
    date: Signal<Date>,
    /// Disables the date regardless of the calendar.
    #[prop(into, optional)]
    is_disabled: Signal<bool>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let classes = with_default_class("leptonic-CalendarCell", classes);
    let Some(context) = expect_calendar("CalendarCell") else {
        return ().into_any();
    };
    let state: CalendarStates = context.data.state;
    let calendar = state.calendar();
    let grid = use_context::<GridContext>();
    let is_outside_month = Signal::derive(move || {
        let duration = calendar.visible_duration.get();
        if duration.days != 0 || duration.weeks != 0 {
            return false;
        }
        let month = grid.map_or_else(
            || calendar.visible_range.get().start,
            |grid| grid.start_date.get(),
        );
        !date.get().is_same_month(month)
    });
    let cell = use_calendar_cell(UseCalendarCellInput {
        is_disabled,
        is_outside_month,
        data: context.data.clone(),
        date,
        element: CapturedElement::new(),
    });
    let range = state.range();
    let is_selection_start = Signal::derive(move || {
        range.is_some_and(|range| {
            range
                .highlighted_range
                .get()
                .is_some_and(|highlighted| highlighted.start == date.get())
        })
    });
    let is_selection_end = Signal::derive(move || {
        range.is_some_and(|range| {
            range
                .highlighted_range
                .get()
                .is_some_and(|highlighted| highlighted.end == date.get())
        })
    });
    let cell_props = cell.cell_props.clone();
    let cell_context = CellContext {
        cell: StoredValue::new(Some(cell)),
        is_outside_month,
        is_selection_start,
        is_selection_end,
    };
    view! {
        <td {..cell_props.into_attrs()} class=classes style=styles>
            <Provider value=cell_context>{children()}</Provider>
        </td>
    }
    .into_any()
}

/// The focusable button of the cell around it, labelled with its date; shows the day number
/// unless given children.
///
/// Data attributes: `data-focused`, `data-focus-visible`, `data-hovered`, `data-pressed`,
/// `data-selected`, `data-selection-start`, `data-selection-end`, `data-disabled`,
/// `data-unavailable`, `data-invalid`, `data-outside-month`, `data-outside-visible-range`,
/// `data-today`.
///
/// Default class: `leptonic-CalendarCellButton`.
#[component]
pub fn CalendarCellButton(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let classes = with_default_class("leptonic-CalendarCellButton", classes);
    let Some(context) = use_context::<CellContext>() else {
        crate::utils::dev_warn!("CalendarCellButton: not inside a CalendarCell");
        return ().into_any();
    };
    let Some(cell) = context.cell.try_update_value(Option::take).flatten() else {
        crate::utils::dev_warn!("CalendarCellButton: used twice in one CalendarCell");
        return ().into_any();
    };
    let UseCalendarCellReturn {
        button_props,
        is_pressed,
        is_focused,
        is_selected,
        is_disabled,
        is_unavailable,
        is_outside_visible_range,
        is_invalid,
        is_today,
        formatted_date,
        ..
    } = cell;
    let hover = use_hover(UseHoverInput {
        is_disabled: Signal::derive(move || is_disabled.get() || is_unavailable.get()),
        ..UseHoverInput::default()
    });
    let UseFocusRingReturn {
        props: focus_ring,
        is_focus_visible,
        ..
    } = use_focus_ring(UseFocusRingInput::default());
    let is_focus_visible = Signal::derive(move || is_focus_visible.get() && is_focused.get());
    let (attrs, button_styles) = button_props.into_parts();
    view! {
        <div
            {..attrs}
            {..focus_ring.into_attrs()}
            {..hover.props.into_attrs()}
            class=classes
            style=button_styles.merge(styles)
            data-focused=flag(is_focused)
            data-focus-visible=flag(is_focus_visible)
            data-hovered=flag(hover.is_hovered)
            data-pressed=flag(is_pressed)
            data-selected=flag(is_selected)
            data-selection-start=flag(context.is_selection_start)
            data-selection-end=flag(context.is_selection_end)
            data-disabled=flag(is_disabled)
            data-unavailable=flag(is_unavailable)
            data-invalid=flag(is_invalid)
            data-outside-month=flag(context.is_outside_month)
            data-outside-visible-range=flag(is_outside_visible_range)
            data-today=flag(is_today)
        >
            {match children {
                Some(children) => children().into_any(),
                None => formatted_date.into_any(),
            }}
        </div>
    }
    .into_any()
}
