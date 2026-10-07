use leptonic::{
    atoms::{
        calendar::{
            Calendar, CalendarCell, CalendarCellButton, CalendarGrid, CalendarGridBody,
            CalendarGridHeader, CalendarHeaderCell, CalendarHeaderRow, CalendarHeading,
            CalendarNextButton, CalendarPreviousButton, CalendarWeek, RangeCalendar,
        },
        datepicker::{
            DateField, DateInput, DatePicker, DatePickerButton, DatePickerGroup, DateRangePicker,
            DateSegment, TimeField,
        },
        dialog::Dialog,
        field::{Description, FieldError, Label},
        popover::Popover,
    },
    hooks::{
        ValidationBehavior,
        datepicker::{HourCycle, RangePart, RangeValue},
    },
    jiff::{
        Zoned,
        civil::{Date, DateTime, Time, date},
    },
};
use leptos::prelude::*;

fn show<T: ToString>(value: Option<T>) -> String {
    value.map_or_else(|| "none".to_owned(), |value| value.to_string())
}

/// The segments of a field.
#[component]
fn Segments() -> impl IntoView {
    view! {
        <DateInput children=|segment| view! { <DateSegment segment=segment /> } />
    }
}

/// The grid of a calendar in a picker's popover.
#[component]
fn PickerGrid() -> impl IntoView {
    view! {
        <header>
            <CalendarPreviousButton>"<"</CalendarPreviousButton>
            <CalendarHeading />
            <CalendarNextButton>">"</CalendarNextButton>
        </header>
        <CalendarGrid>
            <CalendarGridHeader>
                <CalendarHeaderRow children=|day| {
                    view! { <CalendarHeaderCell>{day}</CalendarHeaderCell> }
                } />
            </CalendarGridHeader>
            <CalendarGridBody children=|week| {
                view! {
                    <CalendarWeek
                        week=week
                        children=|date| {
                            view! {
                                <CalendarCell date=date>
                                    <CalendarCellButton />
                                </CalendarCell>
                            }
                        }
                    />
                }
            } />
        </CalendarGrid>
    }
}

/// A calendar for a picker's popover.
#[component]
fn PickerCalendar() -> impl IntoView {
    view! {
        <Calendar>
            <header>
                <CalendarPreviousButton>"<"</CalendarPreviousButton>
                <CalendarHeading />
                <CalendarNextButton>">"</CalendarNextButton>
            </header>
            <CalendarGrid>
                <CalendarGridHeader>
                    <CalendarHeaderRow children=|day| {
                        view! { <CalendarHeaderCell>{day}</CalendarHeaderCell> }
                    } />
                </CalendarGridHeader>
                <CalendarGridBody children=|week| {
                    view! {
                        <CalendarWeek
                            week=week
                            children=|date| {
                                view! {
                                    <CalendarCell date=date>
                                        <CalendarCellButton />
                                    </CalendarCell>
                                }
                            }
                        />
                    }
                } />
            </CalendarGrid>
        </Calendar>
    }
}

/// Date and time fields (react-aria-components' `DateField`/`TimeField` tests), each in a section
/// `#test-df-<name>` with its value in `#test-df-<name>-value`:
/// - basic: an empty date field (placeholder June 1, 2024), named `birthday` in a form with a
///   reset button.
/// - min: May 2024 or later (native validation), starting April 30, 2024.
/// - disabled, read-only: June 5, 2024.
/// - date-time: a date with hours and minutes (24 hours).
/// - zoned: a zoned date and time in New York (12 hours).
/// - time: a time field in 12 hours, empty, named `alarm`.
/// - picker: an empty date picker (placeholder June 1, 2024) with a calendar in its popover,
///   cleared by `#test-df-picker-clear`.
/// - range: an empty date range picker (placeholder June 1, 2024) with a range calendar.
#[component]
pub fn PageAtomDateField() -> impl IntoView {
    let basic = RwSignal::new(None::<Date>);
    let min = RwSignal::new(Some(date(2024, 4, 30)));
    let date_time = RwSignal::new(None::<DateTime>);
    let zoned = RwSignal::new(Some(
        date(2024, 6, 5)
            .at(9, 30, 0, 0)
            .in_tz("America/New_York")
            .expect("a zoned value"),
    ));
    let time = RwSignal::new(None::<Time>);
    let picked = RwSignal::new(None::<Date>);
    let range = RwSignal::new(None::<RangeValue<Date>>);
    view! {
        <div id="test-page-atom-date-field">
            <h1>"Date fields"</h1>
            <section id="test-df-basic">
                <button id="test-df-basic-before">"Before"</button>
                <form id="test-df-form">
                    <DateField
                        value=basic
                        set_value=basic
                        placeholder_value=date(2024, 6, 1)
                        name="birthday"
                    >
                        <Label>"Birthday"</Label>
                        <Segments />
                        <Description>"Your birthday"</Description>
                    </DateField>
                    <button type="reset" id="test-df-reset">"Reset"</button>
                </form>
                <div>"Value: " <span id="test-df-basic-value">{move || show(basic.get())}</span></div>
            </section>
            <section id="test-df-min">
                <DateField
                    value=min
                    set_value=min
                    min_value=date(2024, 5, 1)
                    validation_behavior=ValidationBehavior::Aria
                >
                    <Label>"Start"</Label>
                    <Segments />
                    <FieldError />
                </DateField>
                <div>"Value: " <span id="test-df-min-value">{move || show(min.get())}</span></div>
                <button id="test-df-min-after">"After"</button>
            </section>
            <section id="test-df-disabled">
                <DateField default_value=date(2024, 6, 5) is_disabled=true>
                    <Label>"Disabled"</Label>
                    <Segments />
                </DateField>
            </section>
            <section id="test-df-read-only">
                <DateField default_value=date(2024, 6, 5) is_read_only=true>
                    <Label>"Read only"</Label>
                    <Segments />
                </DateField>
            </section>
            <section id="test-df-date-time">
                <DateField
                    value=date_time
                    set_value=date_time
                    placeholder_value=date(2024, 6, 1).at(0, 0, 0, 0)
                    hour_cycle=HourCycle::H24
                >
                    <Label>"Appointment"</Label>
                    <Segments />
                </DateField>
                <div>
                    "Value: " <span id="test-df-date-time-value">{move || show(date_time.get())}</span>
                </div>
            </section>
            <section id="test-df-zoned">
                <DateField<Zoned> value=zoned set_value=zoned hour_cycle=HourCycle::H12>
                    <Label>"Meeting"</Label>
                    <Segments />
                </DateField<Zoned>>
                <div>
                    "Value: " <span id="test-df-zoned-value">{move || show(zoned.get())}</span>
                </div>
            </section>
            <section id="test-df-time">
                <TimeField value=time set_value=time hour_cycle=HourCycle::H12 name="alarm">
                    <Label>"Alarm"</Label>
                    <Segments />
                </TimeField>
                <div>"Value: " <span id="test-df-time-value">{move || show(time.get())}</span></div>
            </section>
            <section id="test-df-picker">
                <DatePicker value=picked set_value=picked placeholder_value=date(2024, 6, 1)>
                    <Label>"Event"</Label>
                    <DatePickerGroup>
                        <Segments />
                        <DatePickerButton>"▼"</DatePickerButton>
                    </DatePickerGroup>
                    <Popover>
                        <Dialog>
                            <PickerCalendar />
                        </Dialog>
                    </Popover>
                </DatePicker>
                <div>"Value: " <span id="test-df-picker-value">{move || show(picked.get())}</span></div>
                <button id="test-df-picker-clear" on:click=move |_| picked.set(None)>"Clear"</button>
            </section>
            <section id="test-df-range">
                <DateRangePicker
                    value=range
                    set_value=range
                    placeholder_value=date(2024, 6, 1)
                    validation_behavior=ValidationBehavior::Native
                >
                    <Label>"Trip"</Label>
                    <DatePickerGroup>
                        <DateInput
                            part=RangePart::Start
                            children=|segment| view! { <DateSegment segment=segment /> }
                        />
                        <span aria-hidden="true">" – "</span>
                        <DateInput
                            part=RangePart::End
                            children=|segment| view! { <DateSegment segment=segment /> }
                        />
                        <DatePickerButton>"▼"</DatePickerButton>
                    </DatePickerGroup>
                    <FieldError />
                    <Popover>
                        <Dialog>
                            <RangeCalendar>
                                <PickerGrid />
                            </RangeCalendar>
                        </Dialog>
                    </Popover>
                </DateRangePicker>
                <div>
                    "Value: "
                    <span id="test-df-range-value">
                        {move || {
                            range
                                .get()
                                .map_or_else(
                                    || "none".to_owned(),
                                    |range| format!("{} - {}", range.start, range.end),
                                )
                        }}
                    </span>
                </div>
                <button id="test-df-range-after">"After"</button>
            </section>
        </div>
    }
}
