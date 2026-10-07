use leptonic::{
    atoms::calendar::{
        Calendar, CalendarCell, CalendarCellButton, CalendarGrid, CalendarGridBody,
        CalendarGridHeader, CalendarHeaderCell, CalendarHeaderRow, CalendarHeading,
        CalendarNextButton, CalendarPreviousButton, CalendarWeek,
    },
    jiff::civil::Date,
};
use leptos::prelude::*;

#[component]
pub fn CalendarConceptDemo() -> impl IntoView {
    let value = RwSignal::new(None::<Date>);

    view! {
        <Calendar value set_value=value aria_label="Appointment date" classes="demo-calendar">
            <header class="demo-calendar-header">
                <CalendarPreviousButton classes="demo-calendar-nav">
                    <span aria-hidden="true">"\u{2039}"</span>
                </CalendarPreviousButton>
                <CalendarHeading classes="demo-calendar-title"/>
                <CalendarNextButton classes="demo-calendar-nav">
                    <span aria-hidden="true">"\u{203a}"</span>
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

        <p class="demo-status">
            {move || value.get().map_or_else(|| "No date selected".to_owned(), |date| format!("Selected: {date}"))}
        </p>
    }
}
