use leptonic::{
    atoms::{
        calendar::{
            CalendarCell, CalendarCellButton, CalendarGrid, CalendarGridBody, CalendarGridHeader,
            CalendarHeaderCell, CalendarHeaderRow, CalendarHeading, CalendarNextButton,
            CalendarPreviousButton, CalendarWeek, RangeCalendar,
        },
        datepicker::{DateInput, DatePickerButton, DatePickerGroup, DateRangePicker, DateSegment},
        dialog::Dialog,
        field::{FieldError, Label},
        popover::Popover,
    },
    components::prelude::{Checkbox, Icon},
    hooks::{
        ValidationBehavior,
        datepicker::{RangePart, RangeValue},
    },
    jiff::civil::Date,
    prelude::icondata,
};
use leptos::prelude::*;

#[component]
pub fn DateRangePickerAtomDemo() -> impl IntoView {
    let trip = RwSignal::new(None::<RangeValue<Date>>);
    let disabled = RwSignal::new(false);

    view! {
        <DateRangePicker<Date>
            value=trip
            set_value=trip
            validation_behavior=ValidationBehavior::Aria
            is_disabled=disabled
            classes="demo-date-field"
        >
            <Label classes="demo-field-label">"Trip"</Label>
            <DatePickerGroup classes="demo-date-input">
                <DateInput
                    part=RangePart::Start
                    classes="demo-date-segments"
                    children=|segment| view! { <DateSegment segment classes="demo-date-segment"/> }
                />
                <span aria-hidden="true" class="demo-date-range-separator">"\u{2013}"</span>
                <DateInput
                    part=RangePart::End
                    classes="demo-date-segments"
                    children=|segment| view! { <DateSegment segment classes="demo-date-segment"/> }
                />
                <DatePickerButton classes="demo-date-picker-button">
                    <Icon icon=icondata::BsCalendar3/>
                </DatePickerButton>
            </DatePickerGroup>
            <FieldError classes="demo-field-error"/>
            <Popover classes="demo-date-picker-popover">
                <Dialog>
                    <RangeCalendar classes=["demo-date-picker-calendar", "demo-calendar-range"]>
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
                            <CalendarGridBody children=|week| view! {
                                <CalendarWeek week children=|date| view! {
                                    <CalendarCell date>
                                        <CalendarCellButton classes="demo-calendar-day"/>
                                    </CalendarCell>
                                }/>
                            }/>
                        </CalendarGrid>
                    </RangeCalendar>
                </Dialog>
            </Popover>
        </DateRangePicker<Date>>

        <p class="demo-status">
            {move || {
                trip.get().map_or_else(
                    || "No dates chosen.".to_owned(),
                    |RangeValue { start, end }| {
                        if end < start {
                            return "The end date lies before the start date (invalid).".to_owned();
                        }
                        let nights = (end - start).get_days();
                        let nights = if nights == 1 { "1 night".to_owned() } else { format!("{nights} nights") };
                        format!("{} to {} ({nights}).", start.strftime("%B %-d"), end.strftime("%B %-d, %Y"))
                    },
                )
            }}
        </p>

        <div class="demo-controls">
            <Checkbox is_selected=disabled set_selected=disabled>"Disabled"</Checkbox>
        </div>
    }
}
