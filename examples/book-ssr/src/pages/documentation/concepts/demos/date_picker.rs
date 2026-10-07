use leptonic::{
    atoms::{
        calendar::{
            Calendar, CalendarCell, CalendarCellButton, CalendarGrid, CalendarGridBody,
            CalendarGridHeader, CalendarHeaderCell, CalendarHeaderRow, CalendarHeading,
            CalendarNextButton, CalendarPreviousButton, CalendarWeek,
        },
        datepicker::{DateInput, DatePicker, DatePickerButton, DatePickerGroup, DateSegment},
        dialog::Dialog,
        field::Label,
        popover::Popover,
    },
    jiff::civil::Date,
};
use leptos::prelude::*;

#[component]
pub fn DatePickerConceptDemo() -> impl IntoView {
    let departure = RwSignal::new(None::<Date>);

    view! {
        <DatePicker<Date> value=departure set_value=departure classes="demo-date-field">
            <Label classes="demo-field-label">"Departure"</Label>
            <DatePickerGroup classes="demo-date-input">
                <DateInput
                    classes="demo-date-segments"
                    children=|segment| view! { <DateSegment segment classes="demo-date-segment"/> }
                />
                <DatePickerButton classes="demo-date-picker-button">
                    <span aria-hidden="true">"\u{25be}"</span>
                </DatePickerButton>
            </DatePickerGroup>
            <Popover classes="demo-date-picker-popover">
                <Dialog>
                    <Calendar classes="demo-date-picker-calendar">
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
                            <CalendarGridBody children=|week| view! {
                                <CalendarWeek week children=|date| view! {
                                    <CalendarCell date>
                                        <CalendarCellButton classes="demo-calendar-day"/>
                                    </CalendarCell>
                                }/>
                            }/>
                        </CalendarGrid>
                    </Calendar>
                </Dialog>
            </Popover>
        </DatePicker<Date>>

        <p class="demo-status">
            {move || {
                departure.get().map_or_else(
                    || "No departure date.".to_owned(),
                    |date| format!("Departure on {}.", date.strftime("%A, %B %-d, %Y")),
                )
            }}
        </p>
    }
}
