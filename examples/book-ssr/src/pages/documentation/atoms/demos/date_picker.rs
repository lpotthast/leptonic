use leptonic::{
    atoms::{
        calendar::{
            Calendar, CalendarCell, CalendarCellButton, CalendarGrid, CalendarGridBody,
            CalendarGridHeader, CalendarHeaderCell, CalendarHeaderRow, CalendarHeading,
            CalendarNextButton, CalendarPreviousButton, CalendarWeek,
        },
        checkbox::{CheckboxButton, CheckboxField},
        datepicker::{DateInput, DatePicker, DatePickerButton, DatePickerGroup, DateSegment},
        dialog::Dialog,
        field::{Description, FieldError, Label},
        popover::Popover,
    },
    hooks::form::ValidationBehavior,
    jiff::civil::{Date, Weekday, date},
};
use leptos::prelude::*;

#[component]
pub fn DatePickerAtomDemo() -> impl IntoView {
    let appointment = RwSignal::new(None::<Date>);
    let disabled = RwSignal::new(false);

    view! {
        <DatePicker<Date>
            value=appointment
            set_value=appointment
            min_value=date(2026, 3, 2)
            max_value=date(2026, 4, 30)
            placeholder_value=date(2026, 3, 2)
            // The practice is closed on weekends.
            is_date_unavailable=|date: Date| matches!(date.weekday(), Weekday::Saturday | Weekday::Sunday)
            validation_behavior=ValidationBehavior::Aria
            is_disabled=disabled
            classes="demo-date-field"
        >
            <Label classes="demo-field-label">"Appointment"</Label>
            <DatePickerGroup classes="demo-date-input">
                <DateInput
                    classes="demo-date-segments"
                    children=|segment| view! { <DateSegment segment classes="demo-date-segment"/> }
                />
                <DatePickerButton classes="demo-date-picker-button">
                    <span aria-hidden="true">"\u{25be}"</span>
                </DatePickerButton>
            </DatePickerGroup>
            <Description classes="demo-field-description">"Weekdays in March and April 2026."</Description>
            <FieldError classes="demo-field-error"/>
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
                appointment.get().map_or_else(
                    || "No appointment".to_owned(),
                    |date| format!("Appointment on {}", date.strftime("%A, %B %-d, %Y")),
                )
            }}
        </p>

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
