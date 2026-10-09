use std::{collections::HashMap, sync::Arc};

use leptonic::{
    I18nProvider, Locale,
    atoms::{
        button::Button,
        calendar::{
            Calendar, CalendarCell, CalendarCellButton, CalendarGrid, CalendarGridBody,
            CalendarGridHeader, CalendarHeaderCell, CalendarHeaderRow, CalendarHeading,
            CalendarNextButton, CalendarPreviousButton, CalendarWeek, RangeCalendar,
        },
        datepicker::{
            DateField, DateInput, DatePicker, DatePickerButton, DatePickerGroup, DatePickerProps,
            DateRangePicker, DateSegment, TimeField, use_date_picker_state_context,
            use_date_range_picker_state_context,
        },
        dialog::Dialog,
        field::{Description, FieldError, FieldErrorMessage, Label},
        form::Form,
        popover::Popover,
    },
    hooks::{
        datepicker::{Granularity, HourCycle, RangePart, RangeValue},
        form::{ValidationBehavior, ValidationResult},
    },
    jiff::{
        Zoned,
        civil::{Date, DateTime, Time, date},
    },
    use_i18n,
};
use leptos::{ev::SubmitEvent, prelude::*};

use crate::pages::Section;

fn show<T: ToString>(value: Option<T>) -> String {
    value.map_or_else(|| "none".to_owned(), |value| value.to_string())
}

fn show_range<T: ToString>(value: Option<RangeValue<T>>) -> String {
    value.map_or_else(
        || "none".to_owned(),
        |range| format!("{} - {}", range.start.to_string(), range.end.to_string()),
    )
}

fn locale(tag: &str) -> Locale {
    tag.parse().expect("a locale")
}

/// The segments of a field.
#[component]
fn Segments() -> impl IntoView {
    view! {
        <DateInput children=|segment| view! { <DateSegment segment=segment /> } />
    }
}

/// The parts of a calendar in a picker's popover.
#[component]
fn CalendarParts() -> impl IntoView {
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

/// The group, popover and calendar of a date picker.
#[component]
fn PickerParts(#[prop(optional)] error_message: Option<FieldErrorMessage>) -> impl IntoView {
    view! {
        <DatePickerGroup>
            <Segments />
            <DatePickerButton>"▼"</DatePickerButton>
        </DatePickerGroup>
        {error_message.map_or_else(
            || view! { <FieldError /> }.into_any(),
            |message| view! { <FieldError message=message /> }.into_any(),
        )}
        <Popover>
            <Dialog>
                <Calendar>
                    <CalendarParts />
                </Calendar>
            </Dialog>
        </Popover>
    }
}

/// The group, popover and range calendar of a date range picker.
#[component]
fn RangePickerParts() -> impl IntoView {
    view! {
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
        <Popover>
            <Dialog>
                <RangeCalendar>
                    <CalendarParts />
                </RangeCalendar>
            </Dialog>
        </Popover>
    }
}

/// A date picker `#test-dp-{name}` on February 3, 2019, its value in `#test-dp-{name}-value`.
#[component]
fn TestDatePicker(
    name: &'static str,
    #[prop(optional)] should_close_on_select: Option<bool>,
    #[prop(optional)] is_disabled: bool,
    #[prop(optional)] empty: bool,
) -> impl IntoView {
    let value = RwSignal::new((!empty).then(|| date(2019, 2, 3)));
    view! {
        <Section name=name>
            <section id=format!("test-dp-{name}")>
                <DatePicker
                    value=value
                    set_value=value
                    should_close_on_select=should_close_on_select.unwrap_or(true)
                    is_disabled=is_disabled
                    name=format!("{name}-date")
                >
                    <Label>{name}</Label>
                    <PickerParts />
                </DatePicker>
                <div>"Value: " <span id=format!("test-dp-{name}-value")>{move || show(value.get())}</span></div>
                <button
                    id=format!("test-dp-{name}-set")
                    on:click=move |_| value.set(Some(date(2020, 2, 3)))
                >
                    "Set February 3, 2020"
                </button>
            </section>
        </Section>
    }
}

/// Switches the locale of the `I18nProvider` around it.
#[component]
fn LocaleSwitch(id: &'static str, to: &'static str) -> impl IntoView {
    let i18n = use_i18n();
    view! {
        <button
            id=id
            on:click=move |_| {
                if let Some(i18n) = &i18n {
                    i18n.set_locale(locale(to));
                }
            }
        >
            {format!("Switch to {to}")}
        </button>
    }
}

/// Date pickers and fields beyond the basics of `/atoms/date-field` (react-aria-components'
/// `DatePicker`, `DateRangePicker`, `TimeField` and `DateField` tests), each in a section
/// `#test-dp-<name>` (the [`Section`] `<name>`):
/// - close-true, close-false: pickers on February 3, 2019, closing on select or not.
/// - disabled: a disabled picker.
/// - empty: an empty picker set to February 3, 2020 by `#test-dp-empty-set`.
/// - required, time-required: a required picker and time field in forms
///   (`#test-dp-required-form`, reset by `#test-dp-required-reset`, and
///   `#test-dp-time-required-form`), with their errors.
/// - range-time: a range picker of dates and times (to the second) on January 2023.
/// - range-open: a range picker of dates and times not closing on select, placeholder time 10:30.
/// - keys: a date field on December 31, 2024; empty: an empty date field.
/// - de: a German date field (June 5, 2024); rtl: a Hebrew date picker with a time (`dir="rtl"`); switch: an
///   English date field switching to Hebrew.
#[component]
pub fn PageAtomDatePicker() -> impl IntoView {
    let range_time = RwSignal::new(None::<RangeValue<DateTime>>);
    let range_open = RwSignal::new(None::<RangeValue<DateTime>>);
    let required = RwSignal::new(None::<Date>);
    let time_required = RwSignal::new(None::<Time>);
    let keys = RwSignal::new(Some(date(2024, 12, 31)));
    let empty_field = RwSignal::new(None::<Date>);
    let de = RwSignal::new(Some(date(2024, 6, 5)));
    let rtl = RwSignal::new(Some(date(2024, 6, 5).at(9, 30, 0, 0)));
    view! {
        <div id="test-page-atom-date-picker">
            <h1>"Date pickers"</h1>
            <TestDatePicker name="close-true" />
            <TestDatePicker name="close-false" should_close_on_select=false />
            <TestDatePicker name="disabled" is_disabled=true />
            <TestDatePicker name="empty" empty=true />
            <Section name="required">
                <section id="test-dp-required">
                    <form id="test-dp-required-form">
                        <DatePicker value=required set_value=required name="date" is_required=true>
                            <Label>"Birth date"</Label>
                            <PickerParts />
                        </DatePicker>
                        <button id="test-dp-required-reset" type="reset">"Reset"</button>
                    </form>
                    <button id="test-dp-required-after">"After"</button>
                </section>
            </Section>
            <Section name="time-required">
                <section id="test-dp-time-required">
                    <form id="test-dp-time-required-form">
                        <TimeField value=time_required set_value=time_required name="time" is_required=true>
                            <Label>"Time"</Label>
                            <Segments />
                            <FieldError />
                        </TimeField>
                    </form>
                    <button id="test-dp-time-required-after">"After"</button>
                </section>
            </Section>
            <Section name="range-time">
                <section id="test-dp-range-time">
                    <DateRangePicker
                        value=range_time
                        set_value=range_time
                        granularity=Granularity::Second
                        placeholder_value=date(2023, 1, 10).at(0, 0, 0, 0)
                    >
                        <Label>"Stay"</Label>
                        <RangePickerParts />
                    </DateRangePicker>
                    <div>
                        "Value: " <span id="test-dp-range-time-value">{move || show_range(range_time.get())}</span>
                    </div>
                </section>
            </Section>
            <Section name="range-open">
                <section id="test-dp-range-open">
                    <DateRangePicker
                        value=range_open
                        set_value=range_open
                        should_close_on_select=false
                        placeholder_value=date(2023, 1, 10).at(10, 30, 0, 0)
                    >
                        <Label>"Trip"</Label>
                        <RangePickerParts />
                    </DateRangePicker>
                    <div>
                        "Value: " <span id="test-dp-range-open-value">{move || show_range(range_open.get())}</span>
                    </div>
                </section>
            </Section>
            <Section name="keys">
                <section id="test-dp-keys">
                    <button id="test-dp-keys-before">"Before"</button>
                    <form id="test-dp-keys-form">
                        <DateField value=keys set_value=keys name="keys">
                            <Label>"Due"</Label>
                            <Segments />
                        </DateField>
                    </form>
                    <div>"Value: " <span id="test-dp-keys-value">{move || show(keys.get())}</span></div>
                </section>
            </Section>
            <Section name="empty-field">
                <section id="test-dp-empty-field">
                    <DateField value=empty_field set_value=empty_field>
                        <Label>"Start"</Label>
                        <Segments />
                    </DateField>
                </section>
            </Section>
            <Section name="de">
                <section id="test-dp-de">
                    <I18nProvider locale=locale("de-DE")>
                        <DateField value=de set_value=de>
                            <Label>"Datum"</Label>
                            <Segments />
                        </DateField>
                    </I18nProvider>
                    <div>"Value: " <span id="test-dp-de-value">{move || show(de.get())}</span></div>
                </section>
            </Section>
            // The locale's own 12-hour clock (`Intl`'s `hour12: true`) at 0:30.
            <Section name="de-12h">
                <section id="test-dp-de-12h">
                    <I18nProvider locale=locale("de-DE")>
                        <TimeField default_value=Time::constant(0, 30, 0, 0) hour_cycle=HourCycle::H12>
                            <Label>"Uhrzeit"</Label>
                            <Segments />
                        </TimeField>
                    </I18nProvider>
                </section>
            </Section>
            // Hour-only: ICU4X's German pattern has a flexible day period ("12 nachts").
            <Section name="de-12h-hour">
                <section id="test-dp-de-12h-hour">
                    <I18nProvider locale=locale("de-DE")>
                        <TimeField
                            default_value=Time::constant(0, 30, 0, 0)
                            hour_cycle=HourCycle::H12
                            granularity=Granularity::Hour
                        >
                            <Label>"Stunde"</Label>
                            <Segments />
                        </TimeField>
                    </I18nProvider>
                </section>
            </Section>
            <Section name="ja-12h">
                <section id="test-dp-ja-12h">
                    <I18nProvider locale=locale("ja-JP")>
                        <TimeField default_value=Time::constant(0, 30, 0, 0) hour_cycle=HourCycle::H12>
                            <Label>"時刻"</Label>
                            <Segments />
                        </TimeField>
                    </I18nProvider>
                </section>
            </Section>
            // Laid out right to left, as an app does (`I18nProvider` renders no `dir`), the group's
            // parts in a row.
            <Section name="rtl">
                <section id="test-dp-rtl" dir="rtl">
                    <style>"#test-dp-rtl .leptonic-DatePickerGroup { display: inline-flex; }"</style>
                    <I18nProvider locale=locale("he-IL")>
                        <DatePicker<DateTime>
                            value=rtl
                            set_value=rtl
                            granularity=Granularity::Minute
                            hour_cycle=HourCycle::H24
                        >
                            <Label>"תאריך"</Label>
                            <PickerParts />
                        </DatePicker<DateTime>>
                    </I18nProvider>
                    <div>"Value: " <span id="test-dp-rtl-value">{move || show(rtl.get())}</span></div>
                </section>
            </Section>
            <Section name="switch">
                <section id="test-dp-switch">
                    <I18nProvider locale=locale("en-US")>
                        <LocaleSwitch id="test-dp-switch-he" to="he-IL" />
                        <DateField default_value=date(2024, 6, 5)>
                            <Label>"Switching"</Label>
                            <Segments />
                        </DateField>
                    </I18nProvider>
                </section>
            </Section>
            <StructurePickers />
            <TimePickers />
            <LabelledPickers />
            <ValidationPickers />
            <FormPickers />
        </div>
    }
}

/// The group, popover and calendar of a date picker, the popover also holding a time field of the
/// picker's time (react-spectrum's date picker with a time).
#[component]
fn PickerPartsWithTime() -> impl IntoView {
    view! {
        <DatePickerGroup>
            <Segments />
            <DatePickerButton>"▼"</DatePickerButton>
        </DatePickerGroup>
        <Popover>
            <Dialog>
                <Calendar>
                    <CalendarParts />
                </Calendar>
                <PickerTimeField />
            </Dialog>
        </Popover>
    }
}

/// A time field "Time" of the `DatePicker<DateTime>` around, through its state context.
#[component]
fn PickerTimeField() -> impl IntoView {
    let Some(state) = use_date_picker_state_context::<DateTime>() else {
        return ().into_any();
    };
    view! {
        <TimeField<Time>
            value=state.time_value
            set_value={move |time: Option<Time>| {
                if let Some(time) = time {
                    state.select_time(time);
                }
            }}
        >
            <Label>"Time"</Label>
            <Segments />
        </TimeField<Time>>
    }
    .into_any()
}

/// The group, popover and range calendar of a date range picker, the popover also holding time
/// fields of the start and end times.
#[component]
fn RangePickerPartsWithTimes() -> impl IntoView {
    view! {
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
        <Popover>
            <Dialog>
                <RangeCalendar>
                    <CalendarParts />
                </RangeCalendar>
                <RangeTimeField part=RangePart::Start />
                <RangeTimeField part=RangePart::End />
            </Dialog>
        </Popover>
    }
}

/// A time field ("Start time", "End time") of the `DateRangePicker<DateTime>` around.
#[component]
fn RangeTimeField(part: RangePart) -> impl IntoView {
    let Some(state) = use_date_range_picker_state_context::<DateTime>() else {
        return ().into_any();
    };
    let (value, label) = match part {
        RangePart::Start => (state.start_time, "Start time"),
        RangePart::End => (state.end_time, "End time"),
    };
    view! {
        <TimeField<Time>
            value=value
            set_value={move |time: Option<Time>| {
                if let Some(time) = time {
                    state.select_time(part, time);
                }
            }}
        >
            <Label>{label}</Label>
            <Segments />
        </TimeField<Time>>
    }
    .into_any()
}

/// A popover with a label, a text and a button besides its calendar: none of them is the
/// picker's (react-aria-components' "should clear contexts inside popover").
#[component]
fn PopoverClutter() -> impl IntoView {
    view! {
        <Label>"Hi"</Label>
        <Button>"Hi"</Button>
    }
}

/// The value of a picker in `#test-dp-{name}-value` and its number of changes in
/// `#test-dp-{name}-changes`.
#[component]
fn Output(name: &'static str, value: Signal<String>, changes: ReadSignal<usize>) -> impl IntoView {
    view! {
        <div>
            "Value: " <span id=format!("test-dp-{name}-value")>{move || value.get()}</span>
            " Changes: " <span id=format!("test-dp-{name}-changes")>{move || changes.get()}</span>
        </div>
    }
}

/// Structure, state attributes and popover behavior (react-aria-components' `DatePicker` and
/// `DateRangePicker` tests, react-spectrum's `DatePickerBase`), in sections `#test-dp-<name>`:
/// - slots, slots-description-after, range-slots: empty pickers with a label, a description
///   before or after the controls, and `data-foo="bar"`.
/// - form-value: a picker (February 3, 2020) and a range picker (January 10 to 20, 2023) naming
///   the form `#test-dp-form-value-form` around nothing.
/// - invalid: a picker before its minimum and a reversed range, with `Aria` validation.
/// - range-close-true, range-close-false: range pickers on January 10 to 20, 2023.
/// - range-disabled, read-only, range-read-only: disabled and read-only pickers.
/// - clear-contexts, range-clear-contexts: popovers with their own label and button.
/// - placeholder, range-placeholder: placeholder June 5, 2019; selected: July 5, 2019 over it.
/// - leading-zeros: July 5, 2019 with forced leading zeros; zoned-placeholder: a zoned
///   placeholder (America/Los_Angeles).
/// - range-focus: an empty range picker; era: a picker in 2020 BC.
/// - auto-focus, range-auto-focus: pickers focusing their first segment when mounted.
#[component]
fn StructurePickers() -> impl IntoView {
    let range_close_true = RwSignal::new(Some(RangeValue {
        start: date(2023, 1, 10),
        end: date(2023, 1, 20),
    }));
    let range_close_false = RwSignal::new(Some(RangeValue {
        start: date(2023, 1, 10),
        end: date(2023, 1, 20),
    }));
    let era = RwSignal::new(Some(date(-2019, 2, 3)));
    let zoned_placeholder = "2021-11-07T00:45-07:00[America/Los_Angeles]"
        .parse::<Zoned>()
        .expect("a zoned value");
    view! {
        <Section name="slots">
            <section id="test-dp-slots">
                <DatePicker<Date> attr:data-foo="bar">
                    <Label>"Birth date"</Label>
                    <Description>"Description"</Description>
                    <PickerParts />
                </DatePicker<Date>>
            </section>
        </Section>
        <Section name="slots-description-after">
            <section id="test-dp-slots-description-after">
                <DatePicker<Date> attr:data-foo="bar">
                    <Label>"Birth date"</Label>
                    <PickerParts />
                    <Description>"Description"</Description>
                </DatePicker<Date>>
            </section>
        </Section>
        <Section name="range-slots">
            <section id="test-dp-range-slots">
                <DateRangePicker<Date> attr:data-foo="bar">
                    <Label>"Trip dates"</Label>
                    <Description>"Description"</Description>
                    <RangePickerParts />
                </DateRangePicker<Date>>
            </section>
        </Section>
        <Section name="form-value">
            <section id="test-dp-form-value">
                <form id="test-dp-form-value-form"></form>
                <DatePicker<Date>
                    value=Signal::stored(Some(date(2020, 2, 3)))
                    name="birthday"
                    form="test-dp-form-value-form"
                    is_required=true
                >
                    <Label>"Birthday"</Label>
                    <PickerParts />
                </DatePicker<Date>>
                <DateRangePicker<Date>
                    value={Signal::stored(Some(RangeValue { start: date(2023, 1, 10), end: date(2023, 1, 20) }))}
                    start_name="start"
                    end_name="end"
                    form="test-dp-form-value-form"
                    is_required=true
                >
                    <Label>"Trip"</Label>
                    <RangePickerParts />
                </DateRangePicker<Date>>
            </section>
        </Section>
        <Section name="invalid">
            <section id="test-dp-invalid">
                <DatePicker<Date>
                    min_value=Signal::stored(Some(date(2023, 1, 1)))
                    default_value=date(2020, 2, 3)
                    validation_behavior=ValidationBehavior::Aria
                >
                    <Label>"Birth date"</Label>
                    <PickerParts />
                </DatePicker<Date>>
                <DateRangePicker
                    default_value={RangeValue { start: date(2023, 1, 10), end: date(2023, 1, 1) }}
                    validation_behavior=ValidationBehavior::Aria
                >
                    <Label>"Trip dates"</Label>
                    <RangePickerParts />
                </DateRangePicker>
            </section>
        </Section>
        <Section name="range-close-true">
            <section id="test-dp-range-close-true">
                <DateRangePicker value=range_close_true set_value=range_close_true>
                    <Label>"Trip dates"</Label>
                    <RangePickerParts />
                </DateRangePicker>
                <div>
                    "Value: "
                    <span id="test-dp-range-close-true-value">
                        {move || show_range(range_close_true.get())}
                    </span>
                </div>
            </section>
        </Section>
        <Section name="range-close-false">
            <section id="test-dp-range-close-false">
                <DateRangePicker
                    value=range_close_false
                    set_value=range_close_false
                    should_close_on_select=false
                >
                    <Label>"Trip dates"</Label>
                    <RangePickerParts />
                </DateRangePicker>
                <div>
                    "Value: "
                    <span id="test-dp-range-close-false-value">
                        {move || show_range(range_close_false.get())}
                    </span>
                </div>
            </section>
        </Section>
        <Section name="range-disabled">
            <section id="test-dp-range-disabled">
                <DateRangePicker<Date> is_disabled=true start_name="start" end_name="end">
                    <Label>"Trip dates"</Label>
                    <RangePickerParts />
                </DateRangePicker<Date>>
            </section>
        </Section>
        <Section name="read-only">
            <section id="test-dp-read-only">
                <DatePicker<Date> is_read_only=true>
                    <Label>"Date"</Label>
                    <PickerParts />
                </DatePicker<Date>>
            </section>
        </Section>
        <Section name="range-read-only">
            <section id="test-dp-range-read-only">
                <DateRangePicker<Date> is_read_only=true>
                    <Label>"Date"</Label>
                    <RangePickerParts />
                </DateRangePicker<Date>>
            </section>
        </Section>
        <Section name="clear-contexts">
            <section id="test-dp-clear-contexts">
                <DatePicker<Date>>
                    <Label>"Birth date"</Label>
                    <DatePickerGroup>
                        <Segments />
                        <DatePickerButton>"▼"</DatePickerButton>
                    </DatePickerGroup>
                    <Description>"Description"</Description>
                    <Popover>
                        <Dialog>
                            <PopoverClutter />
                            <Calendar>
                                <CalendarParts />
                            </Calendar>
                        </Dialog>
                    </Popover>
                </DatePicker<Date>>
            </section>
        </Section>
        <Section name="range-clear-contexts">
            <section id="test-dp-range-clear-contexts">
                <DateRangePicker<Date>>
                    <Label>"Trip dates"</Label>
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
                    <Description>"Description"</Description>
                    <Popover>
                        <Dialog>
                            <PopoverClutter />
                            <RangeCalendar>
                                <CalendarParts />
                            </RangeCalendar>
                        </Dialog>
                    </Popover>
                </DateRangePicker<Date>>
            </section>
        </Section>
        <Section name="placeholder">
            <section id="test-dp-placeholder">
                <DatePicker<Date> placeholder_value=date(2019, 6, 5)>
                    <Label>"Date"</Label>
                    <PickerParts />
                </DatePicker<Date>>
            </section>
        </Section>
        <Section name="range-placeholder">
            <section id="test-dp-range-placeholder">
                <DateRangePicker<Date> placeholder_value=date(2019, 6, 5)>
                    <Label>"Date"</Label>
                    <RangePickerParts />
                </DateRangePicker<Date>>
            </section>
        </Section>
        <Section name="selected">
            <section id="test-dp-selected">
                <DatePicker default_value=date(2019, 7, 5) placeholder_value=date(2019, 6, 5)>
                    <Label>"Date"</Label>
                    <PickerParts />
                </DatePicker>
                <DateRangePicker
                    default_value={RangeValue { start: date(2019, 7, 5), end: date(2019, 7, 10) }}
                    placeholder_value=date(2019, 6, 5)
                >
                    <Label>"Range"</Label>
                    <RangePickerParts />
                </DateRangePicker>
            </section>
        </Section>
        <Section name="leading-zeros">
            <section id="test-dp-leading-zeros">
                <DatePicker default_value=date(2019, 7, 5) should_force_leading_zeros=true>
                    <Label>"Date"</Label>
                    <PickerParts />
                </DatePicker>
                <DateRangePicker
                    default_value={RangeValue { start: date(2019, 7, 5), end: date(2019, 7, 8) }}
                    should_force_leading_zeros=true
                >
                    <Label>"Range"</Label>
                    <RangePickerParts />
                </DateRangePicker>
            </section>
        </Section>
        <Section name="zoned-placeholder">
            <section id="test-dp-zoned-placeholder">
                <DatePicker<Zoned> placeholder_value=zoned_placeholder.clone()>
                    <Label>"Date"</Label>
                    <PickerParts />
                </DatePicker<Zoned>>
                <DateRangePicker<Zoned> placeholder_value=zoned_placeholder>
                    <Label>"Range"</Label>
                    <RangePickerParts />
                </DateRangePicker<Zoned>>
            </section>
        </Section>
        <Section name="range-focus">
            <section id="test-dp-range-focus">
                <DateRangePicker<Date>>
                    <Label>"Range"</Label>
                    <RangePickerParts />
                </DateRangePicker<Date>>
            </section>
        </Section>
        <Section name="auto-focus">
            <section id="test-dp-auto-focus">
                <DatePicker<Date> auto_focus=true>
                    <Label>"Date"</Label>
                    <PickerParts />
                </DatePicker<Date>>
            </section>
        </Section>
        <Section name="range-auto-focus">
            <section id="test-dp-range-auto-focus">
                <DateRangePicker<Date> auto_focus=true>
                    <Label>"Range"</Label>
                    <RangePickerParts />
                </DateRangePicker<Date>>
            </section>
        </Section>
        <Section name="era">
            <section id="test-dp-era">
                <DatePicker value=era set_value=era>
                    <Label>"Date"</Label>
                    <PickerParts />
                </DatePicker>
                <div>"Value: " <span id="test-dp-era-value">{move || show(era.get())}</span></div>
            </section>
        </Section>
    }
}

/// Pickers with times (react-spectrum's `DatePicker` and `DateRangePicker` "calendar popover"
/// tests), their popovers holding time fields, in sections `#test-dp-<name>` with their value and
/// number of changes (`Output`):
/// - date-time: February 3, 2019, 10:45 AM, not closing on select.
/// - date-time-empty: empty (to the minute), the calendar on February 2019; cleared by
///   `#test-dp-date-time-empty-clear`.
/// - second: February 3, 2019, 12:00 AM to the second.
/// - range-times: a range picker of February 3, 2019, 8:45 AM to May 6, 2019, 10:45 AM with
///   times, not closing on select; range-times-empty: an empty one to the minute, the calendar on
///   February 2019, cleared by `#test-dp-range-times-empty-clear`.
/// - zoned: September 21, 2024, 12:00 AM in America/Los_Angeles.
#[component]
fn TimePickers() -> impl IntoView {
    let date_time = RwSignal::new(Some(date(2019, 2, 3).at(10, 45, 0, 0)));
    let (date_time_changes, set_date_time_changes) = signal(0_usize);
    let date_time_empty = RwSignal::new(None::<DateTime>);
    let (empty_changes, set_empty_changes) = signal(0_usize);
    let range_times = RwSignal::new(Some(RangeValue {
        start: date(2019, 2, 3).at(8, 45, 0, 0),
        end: date(2019, 5, 6).at(10, 45, 0, 0),
    }));
    let (range_changes, set_range_changes) = signal(0_usize);
    let range_times_empty = RwSignal::new(None::<RangeValue<DateTime>>);
    let (range_empty_changes, set_range_empty_changes) = signal(0_usize);
    let zoned = RwSignal::new(Some(
        "2024-09-21T00:00:00[America/Los_Angeles]"
            .parse::<Zoned>()
            .expect("a zoned value"),
    ));
    view! {
        <Section name="date-time">
            <section id="test-dp-date-time">
                <DatePicker<DateTime>
                    value=date_time
                    set_value=date_time
                    on_change={move |_: Option<DateTime>| {
                        set_date_time_changes.update(|changes| *changes += 1);
                    }}
                    should_close_on_select=false
                >
                    <Label>"Date"</Label>
                    <PickerPartsWithTime />
                </DatePicker<DateTime>>
                <Output
                    name="date-time"
                    value=Signal::derive(move || show(date_time.get()))
                    changes=date_time_changes
                />
            </section>
        </Section>
        <Section name="date-time-empty">
            <section id="test-dp-date-time-empty">
                <DatePicker<DateTime>
                    value=date_time_empty
                    set_value=date_time_empty
                    on_change={move |_: Option<DateTime>| {
                        set_empty_changes.update(|changes| *changes += 1);
                    }}
                    granularity=Granularity::Minute
                    placeholder_value=date(2019, 2, 1).at(0, 0, 0, 0)
                    should_close_on_select=false
                >
                    <Label>"Date"</Label>
                    <PickerPartsWithTime />
                </DatePicker<DateTime>>
                <Output
                    name="date-time-empty"
                    value=Signal::derive(move || show(date_time_empty.get()))
                    changes=empty_changes
                />
                <button id="test-dp-date-time-empty-clear" on:click=move |_| date_time_empty.set(None)>
                    "Clear"
                </button>
            </section>
        </Section>
        <Section name="second">
            <section id="test-dp-second">
                <DatePicker<DateTime>
                    default_value=date(2019, 2, 3).at(0, 0, 0, 0)
                    granularity=Granularity::Second
                >
                    <Label>"Date"</Label>
                    <PickerParts />
                </DatePicker<DateTime>>
            </section>
        </Section>
        <Section name="range-times">
            <section id="test-dp-range-times">
                <DateRangePicker<DateTime>
                    value=range_times
                    set_value=range_times
                    on_change={move |_: Option<RangeValue<DateTime>>| {
                        set_range_changes.update(|changes| *changes += 1);
                    }}
                    should_close_on_select=false
                >
                    <Label>"Range"</Label>
                    <RangePickerPartsWithTimes />
                </DateRangePicker<DateTime>>
                <Output
                    name="range-times"
                    value=Signal::derive(move || show_range(range_times.get()))
                    changes=range_changes
                />
            </section>
        </Section>
        <Section name="range-times-empty">
            <section id="test-dp-range-times-empty">
                <DateRangePicker<DateTime>
                    value=range_times_empty
                    set_value=range_times_empty
                    on_change={move |_: Option<RangeValue<DateTime>>| {
                        set_range_empty_changes.update(|changes| *changes += 1);
                    }}
                    granularity=Granularity::Minute
                    placeholder_value=date(2019, 2, 1).at(0, 0, 0, 0)
                    should_close_on_select=false
                >
                    <Label>"Range"</Label>
                    <RangePickerPartsWithTimes />
                </DateRangePicker<DateTime>>
                <Output
                    name="range-times-empty"
                    value=Signal::derive(move || show_range(range_times_empty.get()))
                    changes=range_empty_changes
                />
                <button
                    id="test-dp-range-times-empty-clear"
                    on:click=move |_| range_times_empty.set(None)
                >
                    "Clear"
                </button>
            </section>
        </Section>
        <Section name="zoned">
            <section id="test-dp-zoned">
                <DatePicker<Zoned> value=zoned set_value=zoned>
                    <Label>"Date"</Label>
                    <PickerParts />
                </DatePicker<Zoned>>
                <div>"Value: " <span id="test-dp-zoned-value">{move || show(zoned.get())}</span></div>
            </section>
        </Section>
    }
}

/// Labelling and description (react-spectrum's `DatePicker` "labeling" and "events" tests), in
/// sections `#test-dp-<name>`:
/// - events: an empty picker logging its focus changes to `#test-dp-events-log`, between the
///   buttons `#test-dp-events-before` and `#test-dp-events-after`.
/// - aria-label: named by `aria-label` "Birth date"; labelledby: by `#test-dp-labelledby-foo`.
/// - help, help-value: a description "Help text", empty and on February 3, 2020.
/// - error, error-value: invalid with the error "Error message", empty and on February 3, 2020;
///   not-invalid: the error message of a valid picker.
/// - range-description, range-description-same: range pickers of February 3, 2020, 8:00 AM to
///   February 10, 2020, 10:00 AM, and to February 3, 2020, 8:00 AM.
#[component]
fn LabelledPickers() -> impl IntoView {
    let log = RwSignal::new(Vec::<String>::new());
    view! {
        <Section name="events">
            <section id="test-dp-events">
                <button id="test-dp-events-before">"Before"</button>
                <DatePicker<Date> on_focus_change={move |focused: bool| {
                    log.update(|log| log.push(format!("focus:{focused}")));
                }}>
                    <Label>"Date"</Label>
                    <PickerParts />
                </DatePicker<Date>>
                <button id="test-dp-events-after">"After"</button>
                <div id="test-dp-events-log">{move || log.get().join(" ")}</div>
            </section>
        </Section>
        <Section name="aria-label">
            <section id="test-dp-aria-label">
                <DatePicker<Date> aria_label="Birth date">
                    <PickerParts />
                </DatePicker<Date>>
            </section>
        </Section>
        <Section name="labelledby">
            <section id="test-dp-labelledby">
                <span id="test-dp-labelledby-foo">"Foo"</span>
                <DatePicker<Date> aria_labelledby="test-dp-labelledby-foo">
                    <PickerParts />
                </DatePicker<Date>>
            </section>
        </Section>
        <Section name="help">
            <section id="test-dp-help">
                <DatePicker<Date>>
                    <Label>"Date"</Label>
                    <Description>"Help text"</Description>
                    <PickerParts />
                </DatePicker<Date>>
            </section>
        </Section>
        <Section name="help-value">
            <section id="test-dp-help-value">
                <DatePicker<Date> value=Signal::stored(Some(date(2020, 2, 3)))>
                    <Label>"Date"</Label>
                    <Description>"Help text"</Description>
                    <PickerParts />
                </DatePicker<Date>>
            </section>
        </Section>
        <Section name="error">
            <section id="test-dp-error">
                <DatePicker<Date> is_invalid=true>
                    <Label>"Date"</Label>
                    <FieldError>"Error message"</FieldError>
                    <PickerParts />
                </DatePicker<Date>>
            </section>
        </Section>
        <Section name="error-value">
            <section id="test-dp-error-value">
                <DatePicker<Date> value=Signal::stored(Some(date(2020, 2, 3))) is_invalid=true>
                    <Label>"Date"</Label>
                    <FieldError>"Error message"</FieldError>
                    <PickerParts />
                </DatePicker<Date>>
            </section>
        </Section>
        <Section name="not-invalid">
            <section id="test-dp-not-invalid">
                <DatePicker<Date>>
                    <Label>"Date"</Label>
                    <FieldError>"Error message"</FieldError>
                    <PickerParts />
                </DatePicker<Date>>
            </section>
        </Section>
        <Section name="range-description">
            <section id="test-dp-range-description">
                <DateRangePicker<DateTime> value={Signal::stored(Some(RangeValue {
                    start: date(2020, 2, 3).at(8, 0, 0, 0),
                    end: date(2020, 2, 10).at(10, 0, 0, 0),
                }))}>
                    <Label>"Date"</Label>
                    <RangePickerParts />
                </DateRangePicker<DateTime>>
            </section>
        </Section>
        <Section name="range-description-same">
            <section id="test-dp-range-description-same">
                <DateRangePicker<DateTime> value={Signal::stored(Some(RangeValue {
                    start: date(2020, 2, 3).at(8, 0, 0, 0),
                    end: date(2020, 2, 3).at(8, 0, 0, 0),
                }))}>
                    <Label>"Date"</Label>
                    <RangePickerParts />
                </DateRangePicker<DateTime>>
            </section>
        </Section>
    }
}

/// Validation as the user edits (react-spectrum's `DatePicker` and `DateRangePicker` "validation"
/// tests, `Aria` behavior), in sections `#test-dp-<name>`:
/// - min, max: January 1, 1985 with that minimum or maximum.
/// - range-min, range-max: January 1 to 5, 1985 with the minimum January 1, 1985 or the maximum
///   January 5, 1985; range-reversed: an empty range to fill in reversed.
#[component]
fn ValidationPickers() -> impl IntoView {
    view! {
        <Section name="min">
            <section id="test-dp-min">
                <DatePicker<Date>
                    default_value=date(1985, 1, 1)
                    min_value=Signal::stored(Some(date(1985, 1, 1)))
                    validation_behavior=ValidationBehavior::Aria
                >
                    <Label>"Date"</Label>
                    <PickerParts />
                </DatePicker<Date>>
            </section>
        </Section>
        <Section name="max">
            <section id="test-dp-max">
                <DatePicker<Date>
                    default_value=date(1985, 1, 1)
                    max_value=Signal::stored(Some(date(1985, 1, 1)))
                    validation_behavior=ValidationBehavior::Aria
                >
                    <Label>"Date"</Label>
                    <PickerParts />
                </DatePicker<Date>>
            </section>
        </Section>
        <Section name="range-min">
            <section id="test-dp-range-min">
                <DateRangePicker
                    default_value={RangeValue { start: date(1985, 1, 1), end: date(1985, 1, 5) }}
                    min_value=Signal::stored(Some(date(1985, 1, 1)))
                    validation_behavior=ValidationBehavior::Aria
                >
                    <Label>"Range"</Label>
                    <RangePickerParts />
                </DateRangePicker>
            </section>
        </Section>
        <Section name="range-max">
            <section id="test-dp-range-max">
                <DateRangePicker
                    default_value={RangeValue { start: date(1985, 1, 1), end: date(1985, 1, 5) }}
                    max_value=Signal::stored(Some(date(1985, 1, 5)))
                    validation_behavior=ValidationBehavior::Aria
                >
                    <Label>"Range"</Label>
                    <RangePickerParts />
                </DateRangePicker>
            </section>
        </Section>
    }
}

/// Fails the year before 2022 ("Invalid value").
fn before_2022(value: &Option<Date>) -> Result<(), Vec<String>> {
    match value {
        Some(value) if value.year() < 2022 => Err(vec!["Invalid value".to_owned()]),
        _ => Ok(()),
    }
}

/// A form `#test-dp-{name}-form` with a date picker `date`, its error, a reset button
/// `#test-dp-{name}-reset` and a button `#test-dp-{name}-after` to leave the picker.
#[component]
fn FormPicker(
    name: &'static str,
    #[prop(optional)] default_value: Option<Date>,
    #[prop(optional)] is_required: bool,
    #[prop(optional)] min_value: Option<Date>,
    #[prop(optional)] max_value: Option<Date>,
    /// Fails years before 2022 ("Invalid value").
    #[prop(optional)]
    validate: bool,
    #[prop(default = ValidationBehavior::Native)] validation_behavior: ValidationBehavior,
) -> impl IntoView {
    let props = DatePickerProps::builder()
        .name("date")
        .default_value(default_value.unwrap_or(date(2020, 2, 3)))
        .is_required(is_required)
        .min_value(min_value)
        .max_value(max_value)
        .children(Box::new(|| {
            view! {
                <Label>"Date"</Label>
                <PickerParts />
            }
            .into_any()
        }));
    let props = if validate {
        props.validate(Arc::new(before_2022)).build()
    } else {
        props.build()
    };
    view! {
        <Section name=name>
            <section id=format!("test-dp-{name}")>
                <Form attr:id=format!("test-dp-{name}-form") validation_behavior=validation_behavior>
                    {DatePicker::<Date>(props)}
                    <button id=format!("test-dp-{name}-reset") type="reset">"Reset"</button>
                </Form>
                <button id=format!("test-dp-{name}-after")>"After"</button>
            </section>
        </Section>
    }
}

/// Pickers in forms (react-spectrum's `DatePicker` and `DateRangePicker` "forms" tests), in
/// sections `#test-dp-<name>` (see `FormPicker`):
/// - form-reset: a picker bound to a signal on February 3, 2020, `#test-dp-form-reset-reset`.
/// - form-min-max: February 3, 2019, minimum February 3, 2020, maximum February 3, 2024.
/// - form-validate: February 3, 2020, failing years before 2022.
/// - form-server: an empty picker whose form sets the server error "Invalid value" on submit
///   (`#test-dp-form-server-submit`).
/// - form-custom: an empty required picker with the message "Please enter a value".
/// - aria-min-max, aria-validate: as form-min-max and form-validate with `Aria` behavior;
///   aria-server: February 3, 2020 with the server error "Invalid value".
/// - range-required: an empty required range picker in `#test-dp-range-required-form`.
#[component]
fn FormPickers() -> impl IntoView {
    let reset_value = RwSignal::new(Some(date(2020, 2, 3)));
    let server_errors = RwSignal::new(HashMap::<String, Vec<String>>::new());
    let on_submit = move |e: SubmitEvent| {
        e.prevent_default();
        server_errors.set(HashMap::from([(
            "date".to_owned(),
            vec!["Invalid value".to_owned()],
        )]));
    };
    let aria_server_errors = Signal::stored(HashMap::from([(
        "value".to_owned(),
        vec!["Invalid value".to_owned()],
    )]));
    view! {
        <Section name="form-reset">
            <section id="test-dp-form-reset">
                <form id="test-dp-form-reset-form">
                    <DatePicker value=reset_value set_value=reset_value name="date">
                        <Label>"Value"</Label>
                        <PickerParts />
                    </DatePicker>
                    <button id="test-dp-form-reset-reset" type="reset">"Reset"</button>
                </form>
            </section>
        </Section>
        <FormPicker
            name="form-min-max"
            default_value=date(2019, 2, 3)
            min_value=date(2020, 2, 3)
            max_value=date(2024, 2, 3)
        />
        <FormPicker name="form-validate" validate=true />
        <Section name="form-server">
            <section id="test-dp-form-server">
                <Form attr:id="test-dp-form-server-form" validation_errors=server_errors on:submit=on_submit>
                    <DatePicker<Date> name="date">
                        <Label>"Value"</Label>
                        <PickerParts />
                    </DatePicker<Date>>
                    <button id="test-dp-form-server-submit" type="submit">"Submit"</button>
                </Form>
            </section>
        </Section>
        <Section name="form-custom">
            <section id="test-dp-form-custom">
                <Form attr:id="test-dp-form-custom-form">
                    <DatePicker<Date> name="date" is_required=true>
                        <Label>"Value"</Label>
                        <PickerParts error_message=Arc::new(|result: &ValidationResult| {
                            result
                                .validation_details
                                .value_missing
                                .then(|| "Please enter a value".to_owned())
                        }) />
                    </DatePicker<Date>>
                </Form>
            </section>
        </Section>
        <FormPicker
            name="aria-min-max"
            default_value=date(2019, 2, 3)
            min_value=date(2020, 2, 3)
            max_value=date(2024, 2, 3)
            validation_behavior=ValidationBehavior::Aria
        />
        <FormPicker name="aria-validate" validate=true validation_behavior=ValidationBehavior::Aria />
        <Section name="aria-server">
            <section id="test-dp-aria-server">
                <Form validation_errors=aria_server_errors>
                    <DatePicker name="value" default_value=date(2020, 2, 3)>
                        <Label>"Value"</Label>
                        <PickerParts />
                    </DatePicker>
                </Form>
            </section>
        </Section>
        <Section name="range-required">
            <section id="test-dp-range-required">
                <form id="test-dp-range-required-form">
                    <DateRangePicker<Date> start_name="start" end_name="end" is_required=true>
                        <Label>"Trip dates"</Label>
                        <RangePickerParts />
                        <FieldError />
                    </DateRangePicker<Date>>
                </form>
                <button id="test-dp-range-required-after">"After"</button>
            </section>
        </Section>
    }
}
