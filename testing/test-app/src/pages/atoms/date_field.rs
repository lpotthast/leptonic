use std::{collections::HashMap, str::FromStr, sync::Arc};

use leptonic::{
    I18nProvider, Locale,
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
        form::Form,
        popover::Popover,
    },
    hooks::{
        datepicker::{Granularity, HourCycle, RangePart, RangeValue, TimeBound},
        form::{ValidateFn, ValidationBehavior, ValidationResult},
    },
    jiff::{
        Zoned,
        civil::{Date, DateTime, Time, date, time},
    },
};
use leptos::{ev::SubmitEvent, prelude::*};

use crate::pages::Section;

fn show<T: ToString>(value: Option<T>) -> String {
    value.map_or_else(|| "none".to_owned(), |value| value.to_string())
}

/// A text input setting `value` to the value typed into it (ISO 8601: `2019-02-03`,
/// `2019-02-03T08:05:10`, `2021-11-07T01:45:00-07:00[America/Los_Angeles]`, `20:24:00`), so that a
/// case starts its field from any value; text that doesn't parse is ignored.
#[component]
fn SetValue<T>(id: String, value: RwSignal<Option<T>>) -> impl IntoView
where
    T: FromStr + Send + Sync + 'static,
{
    view! {
        <input
            id=id
            aria-label="Set the value"
            on:input=move |e| {
                if let Ok(parsed) = event_target_value(&e).parse::<T>() {
                    value.set(Some(parsed));
                }
            }
        />
    }
}

/// The segments of a field.
#[component]
fn Segments() -> impl IntoView {
    view! { <DateInput children=|segment| view! { <DateSegment segment=segment /> } /> }
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

fn locale(tag: &str) -> Locale {
    tag.parse().expect("a locale")
}

/// A field's value output `#test-df-<name>-value` and its setter `#test-df-<name>-set`.
#[component]
fn ValueControls<T>(name: &'static str, value: RwSignal<Option<T>>) -> impl IntoView
where
    T: FromStr + ToString + Clone + Send + Sync + 'static,
{
    view! {
        <div>
            "Value: " <span id=format!("test-df-{name}-value")>{move || show(value.get())}</span>
        </div>
        <SetValue id=format!("test-df-{name}-set") value=value />
    }
}

/// Fields whose segments are edited from values the cases set (`#test-df-<name>-set`): arrow
/// keys, typing, Backspace (react-spectrum's `DatePicker.test.js`, "editing").
#[component]
fn EditingSections() -> impl IntoView {
    let edit_date = RwSignal::new(Some(date(2019, 2, 3)));
    let edit_12h = RwSignal::new(Some(date(2019, 2, 3).at(8, 5, 10, 0)));
    let edit_24h = RwSignal::new(Some(date(2019, 2, 3).at(8, 5, 10, 0)));
    let arabic = RwSignal::new(Some(date(2019, 2, 3)));
    let dst = RwSignal::new(Some(
        "2021-11-07T00:45:00-07:00[America/Los_Angeles]"
            .parse::<Zoned>()
            .expect("a zoned value"),
    ));
    let dst_time = RwSignal::new(None::<Zoned>);
    let dst_placeholder = "2021-11-07T01:45:00-07:00[America/Los_Angeles]"
        .parse::<Zoned>()
        .expect("a zoned value");
    let september = "2024-09-21T00:00:00-07:00[America/Los_Angeles]"
        .parse::<Zoned>()
        .expect("a zoned value");
    let nonexistent = RwSignal::new(None::<Zoned>);
    let time_bounds = RwSignal::new(Some(date(2024, 6, 5).at(8, 0, 0, 0)));
    let time_absolute_bounds = RwSignal::new(Some(date(2024, 6, 6).at(8, 0, 0, 0)));
    view! {
        <Section name="edit-date">
            <section id="test-df-edit-date">
                <DateField value=edit_date set_value=edit_date>
                    <Label>"Edit date"</Label>
                    <Segments />
                    <Description>"Help text"</Description>
                </DateField>
                <ValueControls name="edit-date" value=edit_date />
            </section>
        </Section>
        <Section name="edit-12h">
            <section id="test-df-edit-12h">
                <DateField<DateTime>
                    value=edit_12h
                    set_value=edit_12h
                    granularity=Granularity::Second
                    hour_cycle=HourCycle::H12
                >
                    <Label>"Edit 12 hours"</Label>
                    <Segments />
                </DateField<DateTime>>
                <ValueControls name="edit-12h" value=edit_12h />
            </section>
        </Section>
        <Section name="edit-24h">
            <section id="test-df-edit-24h">
                <DateField<DateTime>
                    value=edit_24h
                    set_value=edit_24h
                    granularity=Granularity::Second
                    hour_cycle=HourCycle::H24
                >
                    <Label>"Edit 24 hours"</Label>
                    <Segments />
                </DateField<DateTime>>
                <ValueControls name="edit-24h" value=edit_24h />
            </section>
        </Section>
        <Section name="arabic">
            <section id="test-df-arabic">
                <I18nProvider locale=locale("ar-EG")>
                    <DateField value=arabic set_value=arabic>
                        <Label>"التاريخ"</Label>
                        <Segments />
                    </DateField>
                </I18nProvider>
                <ValueControls name="arabic" value=arabic />
            </section>
        </Section>
        <Section name="dst">
            <section id="test-df-dst">
                <DateField<Zoned> value=dst set_value=dst>
                    <Label>"Fall back"</Label>
                    <Segments />
                </DateField<Zoned>>
                <ValueControls name="dst" value=dst />
            </section>
        </Section>
        <Section name="dst-time">
            <section id="test-df-dst-time">
                <TimeField<Zoned> value=dst_time set_value=dst_time placeholder_value=dst_placeholder>
                    <Label>"Zoned time"</Label>
                    <Segments />
                </TimeField<Zoned>>
                <ValueControls name="dst-time" value=dst_time />
            </section>
        </Section>
        <Section name="nonexistent">
            <section id="test-df-nonexistent">
                <DateField<Zoned> value=nonexistent set_value=nonexistent placeholder_value=september>
                    <Label>"Spring forward"</Label>
                    <Segments />
                </DateField<Zoned>>
                <ValueControls name="nonexistent" value=nonexistent />
                <button id="test-df-nonexistent-after">"After"</button>
            </section>
        </Section>
        <Section name="time-bounds">
            <section id="test-df-time-bounds">
                <TimeField<DateTime>
                    value=time_bounds
                    set_value=time_bounds
                    min_value=TimeBound::TimeOfDay(time(9, 0, 0, 0))
                    max_value=TimeBound::TimeOfDay(time(17, 0, 0, 0))
                    validation_behavior=ValidationBehavior::Aria
                >
                    <Label>"Office hours"</Label>
                    <Segments />
                    <FieldError />
                </TimeField<DateTime>>
                <ValueControls name="time-bounds" value=time_bounds />
            </section>
        </Section>
        <Section name="time-absolute-bounds">
            <section id="test-df-time-absolute-bounds">
                <TimeField<DateTime>
                    value=time_absolute_bounds
                    set_value=time_absolute_bounds
                    min_value=TimeBound::Absolute(date(2024, 6, 5).at(9, 0, 0, 0))
                    max_value=TimeBound::Absolute(date(2024, 6, 7).at(17, 0, 0, 0))
                    validation_behavior=ValidationBehavior::Aria
                >
                    <Label>"Delivery window"</Label>
                    <Segments />
                    <FieldError />
                </TimeField<DateTime>>
                <ValueControls name="time-absolute-bounds" value=time_absolute_bounds />
            </section>
        </Section>
    }
}

/// A form `#test-df-<name>-form` around `children`, with a reset button `#test-df-<name>-reset`
/// and a button `#test-df-<name>-after` to move the focus to.
#[component]
fn FormSection(name: &'static str, children: Children) -> impl IntoView {
    view! {
        <section id=format!("test-df-{name}")>
            <form id=format!(
                "test-df-{name}-form",
            )>
                {children()} <button type="reset" id=format!("test-df-{name}-reset")>
                    "Reset"
                </button>
            </form>
            <button id=format!("test-df-{name}-after")>"After"</button>
        </section>
    }
}

/// Fields validated natively and with ARIA (react-spectrum's `DateField.test.js`,
/// `TimeField.test.js`, "validation").
#[component]
fn ValidationSections() -> impl IntoView {
    let before_2022: ValidateFn<Option<Date>> = Arc::new(|value: &Option<Date>| match value {
        Some(value) if value.year() < 2022 => Err(vec!["Invalid value".to_owned()]),
        _ => Ok(()),
    });
    let before_2022_aria = Arc::clone(&before_2022);
    let server_errors = RwSignal::new(HashMap::<String, Vec<String>>::new());
    let on_submit = move |e: SubmitEvent| {
        e.prevent_default();
        server_errors.set(HashMap::from([(
            "date".to_owned(),
            vec!["Invalid value".to_owned()],
        )]));
    };
    let aria_server_errors =
        HashMap::from([("value".to_owned(), vec!["Invalid value".to_owned()])]);
    let time_reset = RwSignal::new(Some(time(8, 30, 0, 0)));
    view! {
        <Section name="v-required">
            <FormSection name="v-required">
                <DateField<Date> name="date" is_required=true>
                    <Label>"Birth date"</Label>
                    <Segments />
                    <FieldError />
                </DateField<Date>>
            </FormSection>
        </Section>
        <Section name="v-minmax">
            <FormSection name="v-minmax">
                <DateField
                    name="date"
                    min_value=date(2020, 2, 3)
                    max_value=date(2024, 2, 3)
                    default_value=date(2019, 2, 3)
                >
                    <Label>"Bounded date"</Label>
                    <Segments />
                    <FieldError />
                </DateField>
            </FormSection>
        </Section>
        <Section name="v-validate">
            <FormSection name="v-validate">
                <DateField name="date" default_value=date(2020, 2, 3) validate=before_2022>
                    <Label>"Validated date"</Label>
                    <Segments />
                    <FieldError />
                </DateField>
            </FormSection>
        </Section>
        <Section name="v-server">
            <section id="test-df-v-server">
                <Form
                    attr:id="test-df-v-server-form"
                    validation_errors=server_errors
                    on:submit=on_submit
                >
                    <DateField<Date> name="date">
                        <Label>"Server date"</Label>
                        <Segments />
                        <FieldError />
                    </DateField<Date>>
                    <button id="test-df-v-server-submit" type="submit">
                        "Submit"
                    </button>
                </Form>
                <button id="test-df-v-server-after">"After"</button>
            </section>
        </Section>
        <Section name="v-custom">
            <FormSection name="v-custom">
                <DateField<Date> name="date" is_required=true>
                    <Label>"Custom message"</Label>
                    <Segments />
                    <FieldError message=Arc::new(|result: &ValidationResult| {
                        result
                            .validation_details
                            .value_missing
                            .then(|| "Please enter a value".to_owned())
                    }) />
                </DateField<Date>>
            </FormSection>
        </Section>
        <Section name="v-aria-minmax">
            <section id="test-df-v-aria-minmax">
                <DateField
                    min_value=date(2020, 2, 3)
                    max_value=date(2024, 2, 3)
                    default_value=date(2019, 2, 3)
                    validation_behavior=ValidationBehavior::Aria
                >
                    <Label>"Bounded"</Label>
                    <Segments />
                    <FieldError />
                </DateField>
            </section>
        </Section>
        <Section name="v-aria-validate">
            <section id="test-df-v-aria-validate">
                <DateField
                    default_value=date(2020, 2, 3)
                    validate=before_2022_aria
                    validation_behavior=ValidationBehavior::Aria
                >
                    <Label>"Validated"</Label>
                    <Segments />
                    <FieldError />
                </DateField>
                <button id="test-df-v-aria-validate-after">"After"</button>
            </section>
        </Section>
        <Section name="v-aria-server">
            <section id="test-df-v-aria-server">
                <Form
                    validation_behavior=ValidationBehavior::Aria
                    validation_errors=Signal::stored(aria_server_errors)
                >
                    <DateField name="value" default_value=date(2020, 2, 3)>
                        <Label>"Server value"</Label>
                        <Segments />
                        <FieldError />
                    </DateField>
                </Form>
                <button id="test-df-v-aria-server-after">"After"</button>
            </section>
        </Section>
        <Section name="time-minmax">
            <FormSection name="time-minmax">
                <TimeField
                    name="time"
                    min_value=TimeBound::TimeOfDay(time(9, 0, 0, 0))
                    max_value=TimeBound::TimeOfDay(time(17, 0, 0, 0))
                    default_value=time(8, 0, 0, 0)
                >
                    <Label>"Opening time"</Label>
                    <Segments />
                    <FieldError />
                </TimeField>
            </FormSection>
        </Section>
        <Section name="time-reset">
            <FormSection name="time-reset">
                <TimeField name="time" value=time_reset set_value=time_reset>
                    <Label>"Reset time"</Label>
                    <Segments />
                </TimeField>
            </FormSection>
        </Section>
    }
}

/// Labels, descriptions, error messages, focus and states (react-spectrum's `DateField.test.js`,
/// "labeling", "basics", "events"; react-aria-components' `DateField.test.js`).
#[component]
fn LabellingSections() -> impl IntoView {
    let error = RwSignal::new(None::<Date>);
    let focus_log = RwSignal::new(Vec::<bool>::new());
    let mouse_down = RwSignal::new(None::<Date>);
    let unmount_shown = RwSignal::new(true);
    let unmount_date = "-002019-02-03"
        .parse::<Date>()
        .expect("a date before Christ");
    view! {
        <Section name="aria-label">
            <section id="test-df-aria-label">
                <DateField<Date> aria_label="Birth date">
                    <Segments />
                </DateField<Date>>
            </section>
        </Section>
        <Section name="aria-labelledby">
            <section id="test-df-aria-labelledby">
                <span id="test-df-external-label">"External label"</span>
                <DateField<Date> aria_labelledby="test-df-external-label">
                    <Segments />
                </DateField<Date>>
            </section>
        </Section>
        <Section name="error">
            <section id="test-df-error">
                <DateField value=error set_value=error is_invalid=true>
                    <Label>"Error"</Label>
                    <Segments />
                    <FieldError>"Error message"</FieldError>
                </DateField>
                <ValueControls name="error" value=error />
            </section>
        </Section>
        <Section name="no-error">
            <section id="test-df-no-error">
                <DateField<Date>>
                    <Label>"No error"</Label>
                    <Segments />
                    <FieldError>"Error message"</FieldError>
                </DateField<Date>>
            </section>
        </Section>
        <Section name="unavailable">
            <section id="test-df-unavailable">
                <DateField<Date>
                    aria_label="Enter date between jan 1 and jan 8, 1980"
                    placeholder_value=date(1980, 6, 1)
                    is_date_unavailable=|day: Date| {
                        (date(1980, 1, 1)..=date(1980, 1, 8)).contains(&day)
                    }
                    validation_behavior=ValidationBehavior::Aria
                >
                    <Segments />
                    <FieldError>"Date unavailable."</FieldError>
                </DateField<Date>>
            </section>
        </Section>
        <Section name="auto-focus">
            <section id="test-df-auto-focus">
                <DateField<Date> auto_focus=true>
                    <Label>"Auto focus"</Label>
                    <Segments />
                </DateField<Date>>
            </section>
        </Section>
        <Section name="focus-events">
            <section id="test-df-focus-events">
                <button id="test-df-focus-events-before">"Before"</button>
                <DateField<Date> on_focus_change=move |focused: bool| {
                    focus_log.update(|log| log.push(focused));
                }>
                    <Label>"Focus events"</Label>
                    <Segments />
                </DateField<Date>>
                <button id="test-df-focus-events-after">"After"</button>
                <div>
                    "Focus changes: "
                    <span id="test-df-focus-events-log">
                        {move || {
                            focus_log
                                .get()
                                .iter()
                                .map(ToString::to_string)
                                .collect::<Vec<_>>()
                                .join(",")
                        }}
                    </span>
                </div>
            </section>
        </Section>
        <Section name="read-only-disabled">
            <section id="test-df-read-only-disabled">
                <DateField default_value=date(2024, 6, 5) is_read_only=true is_disabled=true>
                    <Label>"Read only and disabled"</Label>
                    <Segments />
                </DateField>
            </section>
        </Section>
        // The segments' group is wider than the segments: a press beside them is on the group.
        <Section name="mouse-down">
            <section id="test-df-mouse-down">
                <style>
                    "#test-df-mouse-down .leptonic-DateInput { display: inline-block; width: 300px; }"
                </style>
                <DateField value=mouse_down set_value=mouse_down placeholder_value=date(2020, 2, 3)>
                    <Label>"Mouse down"</Label>
                    <Segments />
                </DateField>
                <ValueControls name="mouse-down" value=mouse_down />
            </section>
        </Section>
        // `#test-df-unmount-remove` removes the whole field 500 ms later (time to focus a segment).
        <Section name="unmount">
            <section id="test-df-unmount">
                <button
                    id="test-df-unmount-remove"
                    on:click=move |_| {
                        set_timeout(
                            move || unmount_shown.set(false),
                            std::time::Duration::from_millis(500),
                        );
                    }
                >
                    "Remove the field"
                </button>
                {move || {
                    unmount_shown
                        .get()
                        .then(|| {
                            view! {
                                <DateField default_value=unmount_date>
                                    <Label>"Removed"</Label>
                                    <Segments />
                                </DateField>
                            }
                        })
                }}
            </section>
        </Section>
    }
}

/// Date and time fields (react-aria-components' `DateField`/`TimeField` tests), each in a section
/// `#test-df-<name>` (the [`Section`] `<name>`) with its value in `#test-df-<name>-value`:
/// - basic: an empty date field (placeholder June 1, 2024), named `birthday` in a form with a
///   reset button.
/// - min: May 2024 or later (native validation), starting April 30, 2024.
/// - disabled, read-only: June 5, 2024.
/// - date-time: a date with hours and minutes (24 hours).
/// - zoned: a zoned date and time in New York (12 hours).
/// - time: a time field in 12 hours, empty, named `alarm` (set by `#test-df-time-set`).
/// - picker: an empty date picker (placeholder June 1, 2024) with a calendar in its popover,
///   cleared by `#test-df-picker-clear`.
/// - range: an empty date range picker (placeholder June 1, 2024) with a range calendar.
///
/// Fields set by the cases (`#test-df-<name>-set` takes an ISO 8601 value; [`EditingSections`]):
/// edit-date (February 3, 2019, with a description), edit-12h and edit-24h (dates and times to the second,
/// 2019-02-03 8:05:10), arabic (`ar-EG`), dst (America/Los_Angeles, 2021-11-07 0:45, the night
/// the clocks go back), dst-time (an empty zoned time field, placeholder 1:45 that night),
/// nonexistent (an empty zoned field, placeholder in September 2024), time-bounds (a time field
/// of dates and times between 9:00 and 17:00 of any day), time-absolute-bounds (a time field of
/// dates and times between June 5, 2024 9:00 and June 7, 2024 17:00). Validation ([`ValidationSections`], each in a form
/// `#test-df-<name>-form` with `#test-df-<name>-reset` and `#test-df-<name>-after`): v-required,
/// v-minmax, v-validate (years before 2022 are invalid), v-server (a server error on submit),
/// v-custom (a custom message for a missing value), v-aria-minmax, v-aria-validate,
/// v-aria-server, time-minmax, time-reset. Labels and states ([`LabellingSections`]):
/// aria-label, aria-labelledby, error, no-error, unavailable (January 1 to 8, 1980),
/// auto-focus, focus-events (`#test-df-focus-events-log`), read-only-disabled, mouse-down (a
/// group wider than its segments), unmount (a field of February 3, 2020 BC removed by
/// `#test-df-unmount-remove` 500 ms later).
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
            <Section name="basic">
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
                        <button type="reset" id="test-df-reset">
                            "Reset"
                        </button>
                    </form>
                    <div>
                        "Value: " <span id="test-df-basic-value">{move || show(basic.get())}</span>
                    </div>
                </section>
            </Section>
            <Section name="min">
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
                    <div>
                        "Value: " <span id="test-df-min-value">{move || show(min.get())}</span>
                    </div>
                    <button id="test-df-min-after">"After"</button>
                </section>
            </Section>
            <Section name="disabled">
                <section id="test-df-disabled">
                    <DateField default_value=date(2024, 6, 5) is_disabled=true>
                        <Label>"Disabled"</Label>
                        <Segments />
                    </DateField>
                </section>
            </Section>
            <Section name="read-only">
                <section id="test-df-read-only">
                    <DateField default_value=date(2024, 6, 5) is_read_only=true>
                        <Label>"Read only"</Label>
                        <Segments />
                    </DateField>
                </section>
            </Section>
            <Section name="date-time">
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
                        "Value: "
                        <span id="test-df-date-time-value">{move || show(date_time.get())}</span>
                    </div>
                </section>
            </Section>
            <Section name="zoned">
                <section id="test-df-zoned">
                    <DateField<Zoned> value=zoned set_value=zoned hour_cycle=HourCycle::H12>
                        <Label>"Meeting"</Label>
                        <Segments />
                    </DateField<Zoned>>
                    <div>
                        "Value: " <span id="test-df-zoned-value">{move || show(zoned.get())}</span>
                    </div>
                </section>
            </Section>
            <Section name="time">
                <section id="test-df-time">
                    <TimeField value=time set_value=time hour_cycle=HourCycle::H12 name="alarm">
                        <Label>"Alarm"</Label>
                        <Segments />
                    </TimeField>
                    <div>
                        "Value: " <span id="test-df-time-value">{move || show(time.get())}</span>
                    </div>
                    <SetValue id="test-df-time-set".to_owned() value=time />
                </section>
            </Section>
            <Section name="picker">
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
                    <div>
                        "Value: "
                        <span id="test-df-picker-value">{move || show(picked.get())}</span>
                    </div>
                    <button id="test-df-picker-clear" on:click=move |_| picked.set(None)>
                        "Clear"
                    </button>
                </section>
            </Section>
            <Section name="range">
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
            </Section>
            <EditingSections />
            <ValidationSections />
            <LabellingSections />
        </div>
    }
}
