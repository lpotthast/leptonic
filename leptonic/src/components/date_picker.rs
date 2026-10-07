use leptos::prelude::*;

use crate::{
    Out,
    atoms::{
        datepicker::{
            DateInput, DatePicker as DatePickerAtom, DatePickerButton, DatePickerGroup,
            DatePickerProps as DatePickerAtomProps, DateSegment,
        },
        dialog::Dialog,
        popover::Popover,
    },
    components::{
        date_selector::{DateSelector, DateSelectorView},
        icon::Icon,
        text_field::field_parts,
    },
    hooks::{
        ValidateFn, ValidationBehavior,
        datepicker::{DateValue, Granularity, HourCycle},
    },
    utils::{classes::Classes, styles::Styles},
};

/// A date field with a button opening a `DateSelector` in a popover, plus its label, description
/// and validation errors. Generic over the value: a `jiff::civil::Date`, a `civil::DateTime`
/// (segments for the time too) or a `jiff::Zoned`.
///
/// State props (C4): `value` + `set_value` (controlled), or `default_value`; `on_change` observes.
#[allow(clippy::too_many_arguments, clippy::needless_pass_by_value)]
#[component]
pub fn DatePicker<V: DateValue>(
    /// The visible label. Without it, set `aria_label`.
    #[prop(into, optional)]
    label: MaybeProp<String>,
    #[prop(into, optional)] description: MaybeProp<String>,
    #[prop(optional)] default_value: Option<V>,
    /// The value (controlled): a value or any signal.
    #[prop(into, optional)]
    value: Option<Signal<Option<V>>>,
    /// Receives the new value: an `RwSignal`, `WriteSignal`, closure, `Callback`, ...
    #[prop(into, optional)]
    set_value: Option<Out<Option<V>>>,
    #[prop(into, optional)] on_change: Option<Callback<Option<V>>>,
    /// What the empty segments start from when edited, and the month the selector opens with.
    /// Default: today, at midnight.
    #[prop(optional)]
    placeholder_value: Option<V>,
    #[prop(into, optional)] min_value: Signal<Option<V>>,
    #[prop(into, optional)] max_value: Signal<Option<V>>,
    #[prop(into, optional)] is_date_unavailable: Option<Callback<V, bool>>,
    /// The smallest segment shown. Default: days for dates, minutes otherwise.
    #[prop(optional)]
    granularity: Option<Granularity>,
    /// Default: the locale's.
    #[prop(optional)]
    hour_cycle: Option<HourCycle>,
    #[prop(optional)] hide_time_zone: bool,
    #[prop(optional)] should_force_leading_zeros: bool,
    /// Whether selecting a date closes the popover. Default: `true`.
    #[prop(into, default = Signal::stored(true))]
    should_close_on_select: Signal<bool>,
    /// What the selector shows first when opened. Default: the days of a month.
    #[prop(optional)]
    initial_view: DateSelectorView,
    #[prop(into, optional)] is_disabled: Signal<bool>,
    #[prop(into, optional)] is_read_only: Signal<bool>,
    #[prop(into, optional)] is_required: Signal<bool>,
    #[prop(into, optional)] is_invalid: Signal<bool>,
    #[prop(optional)] validate: Option<ValidateFn<Option<V>>>,
    #[prop(optional)] validation_behavior: Option<ValidationBehavior>,
    #[prop(into, optional)] name: Option<String>,
    #[prop(into, optional)] form: Option<String>,
    #[prop(optional)] auto_focus: bool,
    #[prop(into, optional)] id: Option<String>,
    #[prop(into, optional)] aria_label: MaybeProp<String>,
    #[prop(into, optional)] aria_describedby: Option<String>,
    #[prop(into, optional)] on_focus_change: Option<Callback<bool>>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
) -> impl IntoView {
    let input = move || {
        view! {
            <DatePickerGroup classes="leptonic-date-picker-group">
                <DateInput
                    classes="leptonic-date-picker-input"
                    children=|segment| {
                        view! { <DateSegment segment=segment classes="leptonic-date-picker-segment" /> }
                    }
                />
                <DatePickerButton classes="leptonic-date-picker-button">
                    <Icon icon=icondata::BsCalendar3 />
                </DatePickerButton>
            </DatePickerGroup>
            <Popover classes="leptonic-date-picker-popover">
                <Dialog classes="leptonic-date-picker-dialog">
                    <DateSelector initial_view=initial_view />
                </Dialog>
            </Popover>
        }
        .into_any()
    };
    let mut props = DatePickerAtomProps::builder()
        .min_value(min_value)
        .max_value(max_value)
        .hide_time_zone(hide_time_zone)
        .should_force_leading_zeros(should_force_leading_zeros)
        .should_close_on_select(should_close_on_select)
        .is_disabled(is_disabled)
        .is_read_only(is_read_only)
        .is_required(is_required)
        .is_invalid(is_invalid)
        .auto_focus(auto_focus)
        .aria_label(aria_label)
        .classes(classes.add("leptonic-date-picker"))
        .styles(styles)
        .children(Box::new(move || field_parts(label, description, input)))
        .build();
    props.default_value = default_value;
    props.value = value;
    props.set_value = set_value;
    props.on_change = on_change;
    props.placeholder_value = placeholder_value.into();
    props.is_date_unavailable = is_date_unavailable;
    props.granularity = granularity.into();
    props.hour_cycle = hour_cycle.into();
    props.validate = validate;
    props.validation_behavior = validation_behavior;
    props.name = name;
    props.form = form;
    props.id = id;
    props.aria_describedby = aria_describedby;
    props.on_focus_change = on_focus_change;
    DatePickerAtom(props)
}
