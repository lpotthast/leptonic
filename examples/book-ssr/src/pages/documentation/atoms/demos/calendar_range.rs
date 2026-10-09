use leptonic::{
    DateDuration, DateRange,
    atoms::{
        calendar::{
            CalendarCell, CalendarCellButton, CalendarGrid, CalendarGridBody, CalendarGridHeader,
            CalendarHeaderCell, CalendarHeaderRow, CalendarHeading, CalendarNextButton,
            CalendarPreviousButton, CalendarWeek, RangeCalendar,
        },
        checkbox::{CheckboxButton, CheckboxField},
    },
    jiff::civil::date,
};
use leptos::prelude::*;

#[component]
pub fn AtomRangeCalendarDemo() -> impl IntoView {
    let value = RwSignal::new(Some(DateRange {
        start: date(2026, 3, 27),
        end: date(2026, 4, 3),
    }));
    let disabled = RwSignal::new(false);

    let status = move || {
        value.get().map_or_else(
            || "No range selected".to_owned(),
            |range| {
                let nights = match (range.end - range.start).get_days() {
                    1 => "1 night".to_owned(),
                    nights => format!("{nights} nights"),
                };
                format!(
                    "{} \u{2013} {} ({nights})",
                    range.start.strftime("%b %-d"),
                    range.end.strftime("%b %-d")
                )
            },
        )
    };

    view! {
        <RangeCalendar
            value
            set_value=value
            visible_duration=DateDuration::months(2)
            is_disabled=disabled
            aria_label="Trip dates"
            classes=["demo-calendar", "demo-calendar-range"]
        >
            <header class="demo-calendar-header">
                <CalendarPreviousButton classes="demo-calendar-nav">
                    <span aria-hidden="true">"\u{2039}"</span>
                </CalendarPreviousButton>
                <CalendarNextButton classes="demo-calendar-nav">
                    <span aria-hidden="true">"\u{203a}"</span>
                </CalendarNextButton>
            </header>
            <div class="demo-calendar-months">
                <Month offset=DateDuration::months(0)/>
                <Month offset=DateDuration::months(1)/>
            </div>
        </RangeCalendar>

        <p class="demo-status">{status}</p>

        <div class="demo-controls">
            <CheckboxField is_selected=disabled set_selected=disabled>
                <CheckboxButton classes="demo-check">
                    <span class="demo-check-box" aria-hidden="true"></span>
                    "Disabled"
                </CheckboxButton>
            </CheckboxField>
        </div>
    }
}

/// A month of the calendar around it: `offset` after the first visible one.
#[component]
fn Month(offset: DateDuration) -> impl IntoView {
    view! {
        <div class="demo-calendar-month">
            <CalendarHeading offset classes="demo-calendar-title"/>
            <CalendarGrid offset classes="demo-calendar-grid">
                <CalendarGridHeader>
                    <CalendarHeaderRow children=|day| view! { <CalendarHeaderCell>{day}</CalendarHeaderCell> }/>
                </CalendarGridHeader>
                <CalendarGridBody children=|week| {
                    view! {
                        <CalendarWeek
                            week
                            children=|date| {
                                view! {
                                    <CalendarCell date>
                                        <CalendarCellButton classes="demo-calendar-day"/>
                                    </CalendarCell>
                                }
                            }
                        />
                    }
                }/>
            </CalendarGrid>
        </div>
    }
}
