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
        field::{FieldError, Label},
        popover::Popover,
    },
    hooks::datepicker::{Granularity, HourCycle, RangePart, RangeValue},
    jiff::civil::{Date, DateTime, Time, date},
    utils::i18n::{I18nProvider, Locale, use_i18n},
};
use leptos::prelude::*;

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
fn PickerParts() -> impl IntoView {
    view! {
        <DatePickerGroup>
            <Segments />
            <DatePickerButton>"▼"</DatePickerButton>
        </DatePickerGroup>
        <FieldError />
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
                    i18n.set_locale.run(locale(to));
                }
            }
        >
            {format!("Switch to {to}")}
        </button>
    }
}

/// Date pickers and fields beyond the basics of `/atoms/date-field` (react-aria-components'
/// `DatePicker`, `DateRangePicker`, `TimeField` and `DateField` tests), each in a section
/// `#test-dp-<name>`:
/// - close-true, close-false: pickers on February 3, 2019, closing on select or not.
/// - disabled: a disabled picker.
/// - empty: an empty picker set to February 3, 2020 by `#test-dp-empty-set`.
/// - required, time-required: a required picker and time field in forms
///   (`#test-dp-required-form`, `#test-dp-time-required-form`), with their errors.
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
            <section id="test-dp-required">
                <form id="test-dp-required-form">
                    <DatePicker value=required set_value=required name="date" is_required=true>
                        <Label>"Birth date"</Label>
                        <PickerParts />
                    </DatePicker>
                </form>
                <button id="test-dp-required-after">"After"</button>
            </section>
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
            <section id="test-dp-empty-field">
                <DateField value=empty_field set_value=empty_field>
                    <Label>"Start"</Label>
                    <Segments />
                </DateField>
            </section>
            <section id="test-dp-de">
                <I18nProvider locale=locale("de-DE")>
                    <DateField value=de set_value=de>
                        <Label>"Datum"</Label>
                        <Segments />
                    </DateField>
                </I18nProvider>
                <div>"Value: " <span id="test-dp-de-value">{move || show(de.get())}</span></div>
            </section>
            // The locale's own 12-hour clock (`Intl`'s `hour12: true`) at 0:30.
            <section id="test-dp-de-12h">
                <I18nProvider locale=locale("de-DE")>
                    <TimeField default_value=Time::constant(0, 30, 0, 0) hour_cycle=HourCycle::H12>
                        <Label>"Uhrzeit"</Label>
                        <Segments />
                    </TimeField>
                </I18nProvider>
            </section>
            // Hour-only: ICU4X's German pattern has a flexible day period ("12 nachts").
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
            <section id="test-dp-ja-12h">
                <I18nProvider locale=locale("ja-JP")>
                    <TimeField default_value=Time::constant(0, 30, 0, 0) hour_cycle=HourCycle::H12>
                        <Label>"時刻"</Label>
                        <Segments />
                    </TimeField>
                </I18nProvider>
            </section>
            // Laid out right to left, as an app does (`I18nProvider` renders no `dir`), the group's
            // parts in a row.
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
            <section id="test-dp-switch">
                <I18nProvider locale=locale("en-US")>
                    <LocaleSwitch id="test-dp-switch-he" to="he-IL" />
                    <DateField default_value=date(2024, 6, 5)>
                        <Label>"Switching"</Label>
                        <Segments />
                    </DateField>
                </I18nProvider>
            </section>
        </div>
    }
}
