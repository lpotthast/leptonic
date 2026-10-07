//! A themed calendar to pick a date, built on the calendar atoms.

use jiff::civil::{Date, Weekday};
use leptos::{html, prelude::*};

use crate::{
    Out,
    atoms::{
        button::Button,
        calendar::{
            Calendar, CalendarCell, CalendarCellButton, CalendarErrorMessage, CalendarGrid,
            CalendarGridBody, CalendarGridHeader, CalendarHeaderCell, CalendarHeaderRow,
            CalendarNextButton, CalendarPreviousButton, CalendarWeek, use_calendar_states,
        },
    },
    hooks::calendar::{
        CalendarHeadingFormat, CalendarPickerItem, CalendarStates, CalendarYearPickerFormat,
        UseCalendarHeadingInput, UseCalendarMonthPickerInput, UseCalendarYearPickerInput,
        use_calendar_heading, use_calendar_month_picker, use_calendar_year_picker,
    },
    utils::{
        aria::AriaCurrent,
        classes::Classes,
        data_attributes::flag,
        date::{DateDuration, DateExt, use_today},
        date_time_formatter::MonthFormat,
        styles::Styles,
    },
};

/// The years a [`DateSelector`]'s year view shows at once.
const VISIBLE_YEARS: u8 = 12;

/// What a [`DateSelector`] shows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DateSelectorView {
    /// The days of a month.
    #[default]
    Days,
    /// The months of a year.
    Months,
    /// A range of years.
    Years,
}

/// A themed calendar to pick a date: a month of days, paged by the arrow buttons. Its heading
/// switches to the years, then the months, to get to a distant date quickly.
///
/// State props (C4): `value` + `set_value` (controlled), or `default_value`; `on_change` observes.
#[component]
#[allow(clippy::too_many_arguments)]
pub fn DateSelector(
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
    /// The initially focused (shown) date. Default: the value, else today.
    #[prop(optional)]
    default_focused_value: Option<Date>,
    #[prop(into, optional)] min_value: Signal<Option<Date>>,
    #[prop(into, optional)] max_value: Signal<Option<Date>>,
    /// Whether a date can't be selected (e.g. booked out).
    #[prop(into, optional)]
    is_date_unavailable: Option<Callback<Date, bool>>,
    #[prop(into, optional)] is_disabled: Signal<bool>,
    #[prop(into, optional)] is_read_only: Signal<bool>,
    #[prop(into, optional)] is_invalid: Signal<bool>,
    /// The first day of the week. Default: the locale's.
    #[prop(into, optional)]
    first_day_of_week: MaybeProp<Weekday>,
    /// The view shown first. `Years` asks for the year, then the month, then the day (e.g. for
    /// birth dates).
    #[prop(optional)]
    initial_view: DateSelectorView,
    /// The error message, shown while the value is invalid (`is_invalid`, out of range or
    /// unavailable).
    #[prop(into, optional)]
    error_message: MaybeProp<String>,
    #[prop(into, optional)] aria_label: MaybeProp<String>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
) -> impl IntoView {
    let current_view = RwSignal::new(initial_view);
    view! {
        <Calendar
            nostrip:default_value=default_value
            nostrip:value=value
            nostrip:set_value=set_value
            nostrip:on_change=on_change
            nostrip:default_focused_value=default_focused_value
            min_value=min_value
            max_value=max_value
            nostrip:is_date_unavailable=is_date_unavailable
            is_disabled=is_disabled
            is_read_only=is_read_only
            is_invalid=is_invalid
            first_day_of_week=first_day_of_week
            aria_label=aria_label
            classes=classes.add("leptonic-date-selector")
            styles=styles
        >
            <DateSelectorContent current_view=current_view error_message=error_message />
        </Calendar>
    }
}

/// The header and the current view of a [`DateSelector`].
#[component]
fn DateSelectorContent(
    current_view: RwSignal<DateSelectorView>,
    error_message: MaybeProp<String>,
) -> impl IntoView {
    let Some(states) = use_calendar_states() else {
        return ().into_any();
    };
    let calendar = states.calendar();
    let is_value_invalid = states.is_value_invalid();
    let content = NodeRef::<html::Div>::new();

    // After switching the view, the focus goes to its current day, month or year (the button
    // that had it is gone).
    Effect::new(move |previous: Option<DateSelectorView>| {
        let view = current_view.get();
        if previous.is_some_and(|previous| previous != view) {
            if view == DateSelectorView::Days {
                calendar.set_focused(true);
            } else {
                request_animation_frame(move || {
                    if let Some(content) = content.get_untracked()
                        && let Ok(Some(current)) = content.query_selector("[data-selected]")
                        && let Some(current) =
                            wasm_bindgen::JsCast::dyn_ref::<web_sys::HtmlElement>(&current)
                    {
                        let _ = current.focus();
                    }
                });
            }
        }
        view
    });

    view! {
        <div class="leptonic-date-selector-header">
            {move || match current_view.get() {
                DateSelectorView::Days => days_header(&states, current_view).into_any(),
                DateSelectorView::Months => months_header(&states, current_view).into_any(),
                DateSelectorView::Years => years_header(&states, current_view).into_any(),
            }}
        </div>
        <div node_ref=content class="leptonic-date-selector-content">
            {move || match current_view.get() {
                DateSelectorView::Days => days().into_any(),
                DateSelectorView::Months => months(&states, current_view).into_any(),
                DateSelectorView::Years => years(&states, current_view).into_any(),
            }}
        </div>
        <Show when=move || is_value_invalid.get() && error_message.with(Option::is_some)>
            <CalendarErrorMessage classes="leptonic-date-selector-error">
                {move || error_message.get()}
            </CalendarErrorMessage>
        </Show>
    }
    .into_any()
}

/// A paging button of the month and year views.
fn page_button(
    label: &'static str,
    class: &'static str,
    on_press: impl Fn() + Send + Sync + 'static,
) -> impl IntoView {
    view! {
        <Button
            classes=class
            aria_label=label
            on_press=Callback::new(move |_| on_press())
        >
            <span class="leptonic-date-selector-arrow" aria-hidden="true"></span>
        </Button>
    }
}

fn days_header(states: &CalendarStates, current_view: RwSignal<DateSelectorView>) -> impl IntoView {
    let heading = use_calendar_heading(UseCalendarHeadingInput {
        state: *states,
        offset: DateDuration::default(),
        format: CalendarHeadingFormat::default(),
    });
    view! {
        <CalendarPreviousButton classes="leptonic-date-selector-previous">
            <span class="leptonic-date-selector-arrow" aria-hidden="true"></span>
        </CalendarPreviousButton>
        <Button
            classes="leptonic-date-selector-heading"
            aria_label=Signal::derive(move || format!("{}, choose a year", heading.get()))
            on_press=Callback::new(move |_| current_view.set(DateSelectorView::Years))
        >
            {heading}
        </Button>
        <CalendarNextButton classes="leptonic-date-selector-next">
            <span class="leptonic-date-selector-arrow" aria-hidden="true"></span>
        </CalendarNextButton>
    }
}

fn months_header(
    states: &CalendarStates,
    current_view: RwSignal<DateSelectorView>,
) -> impl IntoView {
    let calendar = states.calendar();
    let year = Signal::derive(move || calendar.focused_date.get().year().to_string());
    view! {
        {page_button("Previous year", "leptonic-date-selector-previous", move || {
            calendar.focus_previous_section(true);
        })}
        <Button
            classes="leptonic-date-selector-heading"
            aria_label=Signal::derive(move || format!("{}, choose a year", year.get()))
            on_press=Callback::new(move |_| current_view.set(DateSelectorView::Years))
        >
            {year}
        </Button>
        {page_button("Next year", "leptonic-date-selector-next", move || {
            calendar.focus_next_section(true);
        })}
    }
}

fn years_header(
    states: &CalendarStates,
    current_view: RwSignal<DateSelectorView>,
) -> impl IntoView {
    let calendar = states.calendar();
    let picker = use_calendar_year_picker(UseCalendarYearPickerInput {
        state: *states,
        visible_years: VISIBLE_YEARS,
        format: CalendarYearPickerFormat::default(),
    });
    let range = Signal::derive(move || {
        picker
            .items
            .with(|items| match (items.first(), items.last()) {
                (Some(first), Some(last)) => format!("{} – {}", first.formatted, last.formatted),
                _ => String::new(),
            })
    });
    let page = move |years: i32| {
        let focused = calendar.focused_date.get_untracked();
        calendar.set_focused_date(focused.add(DateDuration::years(years)));
    };
    view! {
        {page_button("Previous years", "leptonic-date-selector-previous", move || {
            page(-i32::from(VISIBLE_YEARS));
        })}
        <Button
            classes="leptonic-date-selector-heading"
            aria_label=Signal::derive(move || format!("{}, back to the days", range.get()))
            on_press=Callback::new(move |_| current_view.set(DateSelectorView::Days))
        >
            {range}
        </Button>
        {page_button("Next years", "leptonic-date-selector-next", move || {
            page(i32::from(VISIBLE_YEARS));
        })}
    }
}

fn days() -> impl IntoView {
    view! {
        <CalendarGrid classes="leptonic-date-selector-grid">
            <CalendarGridHeader>
                <CalendarHeaderRow children=|day| {
                    view! { <CalendarHeaderCell classes="leptonic-date-selector-weekday">{day}</CalendarHeaderCell> }
                } />
            </CalendarGridHeader>
            <CalendarGridBody children=|week| {
                view! {
                    <CalendarWeek
                        week=week
                        children=|date| {
                            view! {
                                <CalendarCell date=date classes="leptonic-date-selector-cell">
                                    <CalendarCellButton classes="leptonic-date-selector-day" />
                                </CalendarCell>
                            }
                        }
                    />
                }
            } />
        </CalendarGrid>
    }
}

/// A month or year to pick, marked current (today's) and selected (the focused date's).
fn pick_button(
    item: CalendarPickerItem,
    is_selected: Signal<bool>,
    is_current: Signal<bool>,
    is_disabled: Signal<bool>,
    on_pick: impl Fn() + Send + Sync + 'static,
) -> impl IntoView {
    view! {
        <Button
            classes="leptonic-date-selector-pick"
            is_disabled=is_disabled
            aria_current=Signal::derive(move || is_current.get().then_some(AriaCurrent::Date))
            on_press=Callback::new(move |_| on_pick())
            attr:data-selected=flag(is_selected)
            attr:data-current=flag(is_current)
        >
            {item.formatted}
        </Button>
    }
}

/// The columns of the month and year views (`leptonic-date-selector-picks` in the theme).
const PICK_COLUMNS: usize = 3;

/// Arrow keys between the months or years (by column and row; left and right swapped in
/// right-to-left locales), Home and End to the first and last; disabled ones are skipped.
fn pick_navigation() -> impl Fn(web_sys::KeyboardEvent) + Clone + 'static {
    let direction = crate::utils::i18n::use_direction();
    move |e: web_sys::KeyboardEvent| {
        let rtl = direction.get_untracked() == crate::utils::locale::WritingDirection::Rtl;
        let step: isize = match e.key().as_str() {
            "ArrowRight" if rtl => -1,
            "ArrowLeft" if rtl => 1,
            "ArrowRight" => 1,
            "ArrowLeft" => -1,
            "ArrowDown" => PICK_COLUMNS.cast_signed(),
            "ArrowUp" => -PICK_COLUMNS.cast_signed(),
            "Home" => isize::MIN,
            "End" => isize::MAX,
            _ => return,
        };
        let Some(container) = e
            .current_target()
            .and_then(|target| wasm_bindgen::JsCast::dyn_into::<web_sys::Element>(target).ok())
        else {
            return;
        };
        let Ok(buttons) = container.query_selector_all("button") else {
            return;
        };
        let buttons: Vec<web_sys::HtmlElement> = (0..buttons.length())
            .filter_map(|index| buttons.item(index))
            .filter_map(|node| wasm_bindgen::JsCast::dyn_into::<web_sys::HtmlElement>(node).ok())
            .collect();
        let active = container
            .owner_document()
            .as_ref()
            .and_then(crate::utils::shadow_dom::get_active_element);
        let Some(current) = buttons
            .iter()
            .position(|button| active.as_ref().is_some_and(|active| active == &**button))
        else {
            return;
        };
        let enabled = |index: usize| !buttons[index].has_attribute("disabled");
        let target = match step {
            isize::MIN => (0..buttons.len()).find(|index| enabled(*index)),
            isize::MAX => (0..buttons.len()).rev().find(|index| enabled(*index)),
            step => {
                // The next enabled one in that direction, in steps of `step`.
                let mut index = current.checked_add_signed(step);
                while let Some(candidate) = index.filter(|index| *index < buttons.len()) {
                    if enabled(candidate) {
                        break;
                    }
                    index = candidate.checked_add_signed(step);
                }
                index.filter(|index| *index < buttons.len())
            }
        };
        e.prevent_default();
        if let Some(target) = target {
            let _ = buttons[target].focus();
        }
    }
}

/// Whether the range from `first` to `last` lies outside min and max.
fn outside(calendar_states: &CalendarStates, first: Date, last: Date) -> bool {
    let calendar = calendar_states.calendar();
    calendar.is_disabled.get()
        || calendar.min_value.get().is_some_and(|min| last < min)
        || calendar.max_value.get().is_some_and(|max| first > max)
}

fn months(states: &CalendarStates, current_view: RwSignal<DateSelectorView>) -> impl IntoView {
    let states = *states;
    let picker = use_calendar_month_picker(UseCalendarMonthPickerInput {
        state: states,
        format: MonthFormat::Short,
    });
    let today = use_today();
    view! {
        <div class="leptonic-date-selector-picks" on:keydown=pick_navigation()>
            <For
                each=move || picker.items.get()
                key=|item| (item.id, item.formatted.clone())
                children=move |item| {
                    let id = item.id;
                    let date = item.date;
                    pick_button(
                        item,
                        Signal::derive(move || picker.value.get() == id),
                        Signal::derive(move || {
                            today.get().is_some_and(|today| today.is_same_month(date))
                        }),
                        Signal::derive(move || {
                            outside(&states, date.first_of_month(), date.last_of_month())
                        }),
                        move || {
                            picker.on_change.run(id);
                            current_view.set(DateSelectorView::Days);
                        },
                    )
                }
            />
        </div>
    }
}

fn years(states: &CalendarStates, current_view: RwSignal<DateSelectorView>) -> impl IntoView {
    let states = *states;
    let picker = use_calendar_year_picker(UseCalendarYearPickerInput {
        state: states,
        visible_years: VISIBLE_YEARS,
        format: CalendarYearPickerFormat::default(),
    });
    let today = use_today();
    view! {
        <div class="leptonic-date-selector-picks" on:keydown=pick_navigation()>
            <For
                each=move || picker.items.get()
                key=|item| (item.id, item.formatted.clone())
                children=move |item| {
                    let id = item.id;
                    let date = item.date;
                    pick_button(
                        item,
                        Signal::derive(move || picker.value.get() == id),
                        Signal::derive(move || today.get().is_some_and(|today| today.year() == id)),
                        Signal::derive(move || {
                            outside(&states, date.first_of_year(), date.last_of_year())
                        }),
                        move || {
                            picker.on_change.run(id);
                            current_view.set(DateSelectorView::Months);
                        },
                    )
                }
            />
        </div>
    }
}
