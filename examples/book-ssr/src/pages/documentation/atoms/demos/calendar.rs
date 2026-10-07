use leptonic::{
    atoms::calendar::{
        Calendar, CalendarCell, CalendarCellButton, CalendarGrid, CalendarGridBody,
        CalendarGridHeader, CalendarHeaderCell, CalendarHeaderRow, CalendarHeading,
        CalendarNextButton, CalendarPreviousButton, CalendarWeek,
    },
    components::prelude::*,
    jiff::civil::{Date, Weekday, date},
    prelude::icondata,
};
use leptos::prelude::*;

#[component]
pub fn AtomCalendarDemo() -> impl IntoView {
    let value = RwSignal::new(Some(date(2026, 3, 12)));
    let disabled = RwSignal::new(false);
    let read_only = RwSignal::new(false);

    let status = move || {
        value.get().map_or_else(
            || "No date selected".to_owned(),
            |date| format!("Selected: {}", date.strftime("%A, %B %-d, %Y")),
        )
    };

    view! {
        <Calendar
            value
            set_value=value
            // Weekends are closed.
            is_date_unavailable=|date: Date| matches!(date.weekday(), Weekday::Saturday | Weekday::Sunday)
            is_disabled=disabled
            is_read_only=read_only
            aria_label="Appointment date"
            classes="demo-calendar"
        >
            <header class="demo-calendar-header">
                <CalendarPreviousButton classes="demo-calendar-nav">
                    <Icon icon=icondata::BsChevronLeft/>
                </CalendarPreviousButton>
                <CalendarHeading classes="demo-calendar-title"/>
                <CalendarNextButton classes="demo-calendar-nav">
                    <Icon icon=icondata::BsChevronRight/>
                </CalendarNextButton>
            </header>
            <CalendarGrid classes="demo-calendar-grid">
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
        </Calendar>

        <p class="demo-status">{status}</p>

        <div class="demo-controls">
            <Checkbox is_selected=disabled set_selected=disabled>"Disabled"</Checkbox>
            <Checkbox is_selected=read_only set_selected=read_only>"Read-only"</Checkbox>
        </div>
    }
}
