//! Headless date and time field and picker atoms.
// Upstream: react-aria-components/src/DateField.tsx @ 99e6102368
use std::sync::Arc;

use leptos::{context::Provider, prelude::*};

use super::{
    calendar::{CalendarPickerContext, RangeCalendarPickerContext},
    dialog::DialogTriggerContext,
    field::{FieldContext, LabelContext, LabelPresence},
    form::use_validation_behavior,
    popover::PopoverDefaults,
};
use crate::{
    Out,
    hooks::{
        IntoAttrs, Placement, PropsWithStyles, UseButtonInput, UseButtonReturn, UseFocusRingInput,
        UseFocusRingReturn, UseHoverInput, ValidateFn, ValidationBehavior,
        calendar::DateAvailabilityQuery,
        datepicker::{
            DateFieldData, DateFieldOptions, DateFieldPicker, DateFieldState, DatePickerOptions,
            DateSegment, DateSegmentType, DateValue, Granularity, HourCycle, RangePart, RangeValue,
            TimeValue, UseDateFieldInput, UseDateFieldProps, UseDateFieldReturn,
            UseDateFieldStateInput, UseDatePickerInput, UseDatePickerReturn,
            UseDatePickerStateInput, UseDateRangePickerInput, UseDateRangePickerStateInput,
            UseDateSegmentInput, UseDateSegmentReturn, UseHiddenDateInputInput,
            UseHiddenDateInputReturn, UseTimeFieldInput, UseTimeFieldStateInput,
            use_date_field, use_date_field_state, use_date_picker, use_date_picker_state,
            use_date_range_picker, use_date_range_picker_state, use_date_segment,
            use_hidden_date_input, use_time_field,
            use_time_field_state,
        },
        use_button, use_focus_ring, use_hover,
    },
    utils::{
        CapturedElement, ValueBinding, classes::Classes, data_attributes::flag,
        default_class::with_default_class, scoped_context::scoped_view, styles::Styles,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Generic over the value type (`DateField<jiff::civil::Date>`, `DateField<jiff::Zoned>`,
//   `TimeField<jiff::civil::Time>`, ...); `DateInput` and `DateSegment` are not (the field's
//   context is type-erased).
// - `DateInput`'s children render a segment from a `Signal<DateSegment>` (react-aria-components:
//   a function of the segment); segments are kept by position, their content changes.
// - State props per C4: `value` + `set_value`, `default_value`, `on_change`.
//
// =============================================================================

/// An optional prop as a signal.
fn maybe<T: Clone + Send + Sync + 'static>(prop: MaybeProp<T>) -> Signal<Option<T>> {
    Signal::derive(move || prop.get())
}

/// What a field's `DateInput` and segments need, independent of its value type.
#[derive(Clone)]
struct DateInputContext {
    segments: Signal<Vec<DateSegment>>,
    field_props: StoredValue<Option<PropsWithStyles<UseDateFieldProps>>>,
    segment:
        Arc<dyn Fn(Signal<DateSegment>, CapturedElement) -> UseDateSegmentReturn + Send + Sync>,
    is_disabled: Signal<bool>,
    is_read_only: Signal<bool>,
    is_invalid: Signal<bool>,
}

/// The hover and focus state of a group (react-aria-components' `Group`): `data-hovered`,
/// `data-focus-within`, `data-focus-visible`.
fn group_state(
    is_disabled: Signal<bool>,
) -> (
    (
        <crate::hooks::UseHoverProps as IntoAttrs>::Attrs,
        <crate::hooks::UseFocusRingProps as IntoAttrs>::Attrs,
    ),
    impl Fn() -> Option<&'static str> + Clone + Send + Sync,
    impl Fn() -> Option<&'static str> + Clone + Send + Sync,
    impl Fn() -> Option<&'static str> + Clone + Send + Sync,
) {
    let hover = use_hover(UseHoverInput {
        is_disabled,
        ..UseHoverInput::default()
    });
    let UseFocusRingReturn {
        props: focus_ring,
        is_focused,
        is_focus_visible,
    } = use_focus_ring(UseFocusRingInput {
        within: true,
        ..UseFocusRingInput::default()
    });
    (
        (hover.props.into_attrs(), focus_ring.into_attrs()),
        flag(hover.is_hovered),
        flag(is_focused),
        flag(is_focus_visible),
    )
}

fn render_field<V: DateValue>(
    state: &DateFieldState<V>,
    options: DateFieldOptions,
    // The field's hook: `use_date_field`, or `use_time_field` (its hidden input submits the
    // time).
    use_field: impl FnOnce(DateFieldOptions, CapturedElement, CapturedElement) -> UseDateFieldReturn<V>,
    label_presence: LabelPresence,
    // The hidden date input for autofill (date fields; react-aria-components' time fields have
    // none).
    autofill: Option<UseHiddenDateInputReturn>,
    classes: Classes,
    styles: Styles,
    children: Children,
) -> AnyView {
    let state = *state;
    let element = CapturedElement::new();
    let input_element = CapturedElement::new();
    let UseDateFieldReturn {
        label_props,
        field_props,
        input_props,
        description_props,
        error_message_props,
        data,
    } = use_field(options, element, input_element);
    let is_invalid = state.is_invalid;
    let validation = state.validation;
    let provide = move || {
        provide_context(
            LabelContext::span(label_props.label)
                .with_presence(label_presence)
                .with_on_click(label_props.on_click),
        );
        provide_context(FieldContext {
            description: description_props,
            error_message: error_message_props,
            is_invalid,
            validation_errors: validation.validation_errors,
            validation_details: Signal::derive(move || {
                validation.display_validation.get().validation_details
            }),
        });
        provide_context(DateInputContext {
            segments: state.segments,
            field_props: StoredValue::new(Some(field_props)),
            segment: Arc::new(move |segment, element| {
                use_date_segment(UseDateSegmentInput {
                    segment,
                    data,
                    element,
                })
            }),
            is_disabled: state.is_disabled,
            is_read_only: state.is_read_only,
            is_invalid,
        });
    };
    scoped_view(provide, move || {
        view! {
            <div
                class=classes
                style=styles
                data-disabled=flag(state.is_disabled)
                data-readonly=flag(state.is_read_only)
                data-required=flag(state.is_required)
                data-invalid=flag(is_invalid)
            >
                {children()}
                <input {..input_props.into_attrs()} />
                {autofill.map(hidden_date_input)}
            </div>
        }
    })
    .into_any()
}

/// The visually hidden date input for autofill.
fn hidden_date_input(autofill: UseHiddenDateInputReturn) -> impl IntoView {
    let UseHiddenDateInputReturn {
        container_props,
        input_props,
    } = autofill;
    let styles = container_props.styles.clone();
    view! {
        <div {..container_props.into_attrs()} style=styles>
            <input {..input_props.into_attrs()} />
        </div>
    }
}

/// A date field (react-aria-components' `DateField`): a date edited in segments (month, day,
/// year, and hours to seconds for values with a time). Put a `Label`, a `DateInput` with
/// `DateSegment`s, and `Description`/`FieldError` inside. With a `name`, a hidden input submits
/// the value (ISO 8601).
///
/// Data attributes: `data-disabled`, `data-readonly`, `data-required`, `data-invalid`.
///
/// Default class: `leptonic-DateField`.
#[component]
#[allow(clippy::too_many_arguments)]
pub fn DateField<V: DateValue>(
    #[prop(optional)] default_value: Option<V>,
    /// The value (controlled): a value or any signal.
    #[prop(into, optional)]
    value: Option<Signal<Option<V>>>,
    /// Receives the new value: an `RwSignal`, `WriteSignal`, closure, `Callback`, ...
    #[prop(into, optional)]
    set_value: Option<Out<Option<V>>>,
    #[prop(into, optional)] on_change: Option<Callback<Option<V>>>,
    /// The value the segments start from when edited. Default: today, midnight.
    #[prop(into, optional)]
    placeholder_value: MaybeProp<V>,
    #[prop(into, optional)] min_value: Signal<Option<V>>,
    #[prop(into, optional)] max_value: Signal<Option<V>>,
    #[prop(into, optional)] is_date_unavailable: Option<Callback<V, bool>>,
    /// The finest unit. Default: the minute for values with a time, else the day.
    #[prop(into, optional)]
    granularity: MaybeProp<Granularity>,
    #[prop(into, optional)] hour_cycle: MaybeProp<HourCycle>,
    #[prop(into, optional)] hide_time_zone: Signal<bool>,
    #[prop(into, optional)] should_force_leading_zeros: Signal<bool>,
    #[prop(into, optional)] is_disabled: Signal<bool>,
    #[prop(into, optional)] is_read_only: Signal<bool>,
    #[prop(into, optional)] is_required: Signal<bool>,
    #[prop(into, optional)] is_invalid: Signal<bool>,
    #[prop(optional)] validate: Option<ValidateFn<Option<V>>>,
    /// Default: the surrounding [`Form`](super::form::Form)'s, else `Native`.
    #[prop(optional)]
    validation_behavior: Option<ValidationBehavior>,
    #[prop(into, optional)] name: Option<String>,
    #[prop(into, optional)] form: Option<String>,
    /// What the browser may autofill (`autocomplete`, e.g. `"bday"`), through a visually hidden
    /// date input.
    #[prop(into, optional)]
    auto_complete: Option<String>,
    #[prop(optional)] auto_focus: bool,
    #[prop(into, optional)] id: Option<String>,
    #[prop(into, optional)] aria_label: MaybeProp<String>,
    #[prop(into, optional)] aria_labelledby: Option<String>,
    #[prop(into, optional)] aria_describedby: Option<String>,
    #[prop(into, optional)] on_focus_change: Option<Callback<bool>>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let classes = with_default_class("leptonic-DateField", classes);
    let (value, on_change) = ValueBinding::from_state_props(value, set_value, on_change);
    let state = use_date_field_state(UseDateFieldStateInput {
        default_value,
        value,
        on_change,
        placeholder_value: maybe(placeholder_value),
        min_value,
        max_value,
        is_date_unavailable,
        granularity: maybe(granularity),
        hour_cycle: maybe(hour_cycle),
        hide_time_zone,
        should_force_leading_zeros,
        is_disabled,
        is_read_only,
        is_required,
        is_invalid,
        validate,
        validation_behavior: use_validation_behavior(validation_behavior),
        name: name.clone(),
        ..UseDateFieldStateInput::default()
    });
    let autofill = use_hidden_date_input(UseHiddenDateInputInput {
        value: state.value,
        base: state.date_value,
        granularity: state.granularity,
        set_value: Callback::new(move |value| state.set_value(value)),
        auto_complete,
        name,
        is_disabled: state.is_disabled,
    });
    let label_presence = LabelPresence::new(aria_label, aria_labelledby.as_ref());
    render_field(
        &state,
        DateFieldOptions {
            id,
            has_label: label_presence.has_label,
            aria_label,
            aria_labelledby,
            aria_describedby,
            auto_focus,
            on_focus_change,
            form,
            ..DateFieldOptions::default()
        },
        move |options, element, input_element| {
            use_date_field(UseDateFieldInput {
                state,
                element,
                input_element,
                options,
            })
        },
        label_presence,
        Some(autofill),
        classes,
        styles,
        children,
    )
}

/// A time field (react-aria-components' `TimeField`): hours to seconds edited in segments, with a
/// day period in 12-hour locales. Its parts are those of [`DateField`].
///
/// Data attributes: `data-disabled`, `data-readonly`, `data-required`, `data-invalid`.
///
/// Default class: `leptonic-TimeField`.
#[component]
#[allow(clippy::too_many_arguments)]
pub fn TimeField<T: TimeValue>(
    #[prop(optional)] default_value: Option<T>,
    #[prop(into, optional)] value: Option<Signal<Option<T>>>,
    #[prop(into, optional)] set_value: Option<Out<Option<T>>>,
    #[prop(into, optional)] on_change: Option<Callback<Option<T>>>,
    /// The time the segments start from when edited. Default: midnight.
    #[prop(into, optional)]
    placeholder_value: MaybeProp<T>,
    #[prop(into, optional)] min_value: Signal<Option<T>>,
    #[prop(into, optional)] max_value: Signal<Option<T>>,
    /// Hour, minute (default) or second.
    #[prop(into, optional)]
    granularity: MaybeProp<Granularity>,
    #[prop(into, optional)] hour_cycle: MaybeProp<HourCycle>,
    #[prop(into, optional)] hide_time_zone: Signal<bool>,
    #[prop(into, optional)] should_force_leading_zeros: Signal<bool>,
    #[prop(into, optional)] is_disabled: Signal<bool>,
    #[prop(into, optional)] is_read_only: Signal<bool>,
    #[prop(into, optional)] is_required: Signal<bool>,
    #[prop(into, optional)] is_invalid: Signal<bool>,
    #[prop(optional)] validate: Option<ValidateFn<Option<T>>>,
    #[prop(optional)] validation_behavior: Option<ValidationBehavior>,
    #[prop(into, optional)] name: Option<String>,
    #[prop(into, optional)] form: Option<String>,
    #[prop(optional)] auto_focus: bool,
    #[prop(into, optional)] id: Option<String>,
    #[prop(into, optional)] aria_label: MaybeProp<String>,
    #[prop(into, optional)] aria_labelledby: Option<String>,
    #[prop(into, optional)] aria_describedby: Option<String>,
    #[prop(into, optional)] on_focus_change: Option<Callback<bool>>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let classes = with_default_class("leptonic-TimeField", classes);
    let (value, on_change) = ValueBinding::from_state_props(value, set_value, on_change);
    let state = use_time_field_state(UseTimeFieldStateInput {
        default_value,
        value,
        on_change,
        placeholder_value: maybe(placeholder_value),
        min_value,
        max_value,
        granularity: maybe(granularity),
        hour_cycle: maybe(hour_cycle),
        hide_time_zone,
        should_force_leading_zeros,
        is_disabled,
        is_read_only,
        is_required,
        is_invalid,
        validate,
        validation_behavior: use_validation_behavior(validation_behavior),
        name,
    });
    let label_presence = LabelPresence::new(aria_label, aria_labelledby.as_ref());
    render_field(
        &state.field,
        DateFieldOptions {
            id,
            has_label: label_presence.has_label,
            aria_label,
            aria_labelledby,
            aria_describedby,
            auto_focus,
            on_focus_change,
            form,
            ..DateFieldOptions::default()
        },
        move |options, element, input_element| {
            use_time_field(UseTimeFieldInput {
                state,
                element,
                input_element,
                options,
            })
        },
        label_presence,
        None,
        classes,
        styles,
        children,
    )
}

/// The group of segments of the field around it, rendering `children` per segment (e.g. a
/// `DateSegment`).
///
/// Data attributes: `data-hovered`, `data-focus-within`, `data-focus-visible`, `data-disabled`,
/// `data-invalid`.
///
/// Default class: `leptonic-DateInput`.
#[component]
pub fn DateInput<F, V>(
    children: F,
    /// Inside a `DateRangePicker`: its start or end field (react-aria-components: `slot`, a
    /// name reserved by Leptos' `view!`).
    #[prop(optional)]
    part: Option<RangePart>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
) -> impl IntoView
where
    F: Fn(Signal<DateSegment>) -> V + Send + Sync + 'static,
    V: IntoView + 'static,
{
    let classes = with_default_class("leptonic-DateInput", classes);
    let context = match part {
        Some(part) => use_context::<DateRangeInputContexts>().map(|contexts| match part {
            RangePart::Start => contexts.start,
            RangePart::End => contexts.end,
        }),
        None => use_context::<DateInputContext>(),
    };
    let Some(context) = context else {
        crate::utils::dev_warn!(
            "DateInput: not inside a DateField, TimeField or DatePicker (or a DateRangePicker with a `part`)"
        );
        return ().into_any();
    };
    let Some(field_props) = context.field_props.try_update_value(Option::take).flatten() else {
        crate::utils::dev_warn!("DateInput: used twice in one field");
        return ().into_any();
    };
    let (attrs, field_styles) = field_props.into_parts();
    let segments = context.segments;
    let children = Arc::new(children);
    let (is_disabled, is_invalid) = (context.is_disabled, context.is_invalid);
    let (group_attrs, hovered, focus_within, focus_visible) = group_state(is_disabled);
    view! {
        <div
            {..attrs}
            {..group_attrs}
            class=classes
            style=field_styles.merge(styles)
            data-disabled=flag(is_disabled)
            data-invalid=flag(is_invalid)
            data-hovered=hovered
            data-focus-within=focus_within
            data-focus-visible=focus_visible
        >
            // Kept by position and kind: a segment's element (and the focus) stays while its
            // text changes. The segments use this field's context.
            <Provider value=context.clone()>
            <For
                each=move || {
                    segments.with(|segments| {
                        segments.iter().enumerate().map(|(index, segment)| (index, segment.kind)).collect::<Vec<_>>()
                    })
                }
                key=|key| *key
                children=move |(index, kind)| {
                    let segment = Signal::derive(move || {
                        segments.with(|segments| {
                            segments
                                .get(index)
                                .filter(|segment| segment.kind == kind)
                                .cloned()
                                .unwrap_or_else(|| DateSegment {
                                    kind,
                                    text: String::new(),
                                    value: None,
                                    min_value: None,
                                    max_value: None,
                                    is_placeholder: false,
                                    placeholder: String::new(),
                                    is_editable: false,
                                })
                        })
                    });
                    children(segment)
                }
            />
            </Provider>
        </div>
    }
    .into_any()
}

/// A segment of the field around it: an editable part (month, day, ...) as a spin button
/// editable as text, or a literal between them (hidden from assistive technology).
///
/// Data attributes: `data-type`, `data-placeholder`, `data-readonly`, `data-disabled`,
/// `data-invalid`, `data-hovered`, `data-focused`, `data-focus-visible`.
///
/// Default class: `leptonic-DateSegment`.
#[component]
pub fn DateSegment(
    segment: Signal<DateSegment>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
) -> impl IntoView {
    let classes = with_default_class("leptonic-DateSegment", classes);
    let Some(context) = use_context::<DateInputContext>() else {
        crate::utils::dev_warn!("DateSegment: not inside a DateInput");
        return ().into_any();
    };
    let kind = segment.with_untracked(|segment| segment.kind);
    let text = move || segment.with(|segment| segment.text.clone());
    if kind == DateSegmentType::Literal {
        return view! {
            <span class=classes style=styles aria-hidden="true" data-type="literal">
                {text}
            </span>
        }
        .into_any();
    }
    let element = CapturedElement::new();
    let UseDateSegmentReturn { segment_props } = (context.segment)(segment, element);
    let hover = use_hover(UseHoverInput {
        is_disabled: context.is_disabled,
        ..UseHoverInput::default()
    });
    let UseFocusRingReturn {
        props: focus_ring,
        is_focused,
        is_focus_visible,
    } = use_focus_ring(UseFocusRingInput::default());
    let (attrs, segment_styles) = segment_props.into_parts();
    view! {
        <span
            {..attrs}
            {..focus_ring.into_attrs()}
            {..hover.props.into_attrs()}
            class=classes
            style=segment_styles.merge(styles)
            data-type=kind.as_str()
            data-readonly=flag(context.is_read_only)
            data-disabled=flag(context.is_disabled)
            data-invalid=flag(context.is_invalid)
            data-hovered=flag(hover.is_hovered)
            data-focused=flag(is_focused)
            data-focus-visible=flag(is_focus_visible)
        >
            {text}
        </span>
    }
    .into_any()
}

/// What a date picker's group and button need from it.
#[derive(Clone)]
struct DatePickerContext {
    group_props: StoredValue<Option<PropsWithStyles<UseDateFieldProps>>>,
    button: StoredValue<UseButtonInput>,
    is_open: Signal<bool>,
    is_disabled: Signal<bool>,
    is_invalid: Signal<bool>,
}

/// A date picker (react-aria-components' `DatePicker`): a date field with a button opening a
/// calendar in a popover. Put a `Label`, a `DatePickerGroup` (with a `DateInput` and a
/// `DatePickerButton`), a `Popover` with a `Dialog` and a `Calendar`, and
/// `Description`/`FieldError` inside. With a `name`, a hidden input submits the value.
///
/// Data attributes: `data-disabled`, `data-readonly`, `data-required`, `data-invalid`, `data-open`.
///
/// Default class: `leptonic-DatePicker`.
#[component]
#[allow(
    clippy::too_many_arguments,
    clippy::too_many_lines,
    clippy::fn_params_excessive_bools
)]
pub fn DatePicker<V: DateValue>(
    #[prop(optional)] default_value: Option<V>,
    #[prop(into, optional)] value: Option<Signal<Option<V>>>,
    #[prop(into, optional)] set_value: Option<Out<Option<V>>>,
    #[prop(into, optional)] on_change: Option<Callback<Option<V>>>,
    /// The value the segments start from when edited, and the month the calendar opens on.
    #[prop(into, optional)]
    placeholder_value: MaybeProp<V>,
    #[prop(into, optional)] min_value: Signal<Option<V>>,
    #[prop(into, optional)] max_value: Signal<Option<V>>,
    #[prop(into, optional)] is_date_unavailable: Option<Callback<V, bool>>,
    #[prop(into, optional)] granularity: MaybeProp<Granularity>,
    #[prop(into, optional)] hour_cycle: MaybeProp<HourCycle>,
    #[prop(into, optional)] hide_time_zone: Signal<bool>,
    #[prop(into, optional)] should_force_leading_zeros: Signal<bool>,
    /// Whether selecting a date closes the popover. Default: `true`.
    #[prop(into, default = Signal::stored(true))]
    should_close_on_select: Signal<bool>,
    /// Whether the popover is open (controlled): a value or any signal.
    #[prop(into, optional)]
    is_open: Option<Signal<bool>>,
    #[prop(into, optional)] set_open: Option<Out<bool>>,
    #[prop(optional)] default_open: bool,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    #[prop(into, optional)] is_disabled: Signal<bool>,
    #[prop(into, optional)] is_read_only: Signal<bool>,
    #[prop(into, optional)] is_required: Signal<bool>,
    #[prop(into, optional)] is_invalid: Signal<bool>,
    #[prop(optional)] validate: Option<ValidateFn<Option<V>>>,
    #[prop(optional)] validation_behavior: Option<ValidationBehavior>,
    #[prop(into, optional)] name: Option<String>,
    #[prop(into, optional)] form: Option<String>,
    /// What the browser may autofill (`autocomplete`, e.g. `"bday"`), through a visually hidden
    /// date input.
    #[prop(into, optional)]
    auto_complete: Option<String>,
    #[prop(optional)] auto_focus: bool,
    #[prop(into, optional)] id: Option<String>,
    #[prop(into, optional)] aria_label: MaybeProp<String>,
    #[prop(into, optional)] aria_labelledby: Option<String>,
    #[prop(into, optional)] aria_describedby: Option<String>,
    #[prop(into, optional)] on_focus_change: Option<Callback<bool>>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let classes = with_default_class("leptonic-DatePicker", classes);
    let (value, on_change) = ValueBinding::from_state_props(value, set_value, on_change);
    let (is_open, on_open_change) =
        ValueBinding::from_state_props(is_open, set_open, on_open_change);
    let validation_behavior = use_validation_behavior(validation_behavior);
    let placeholder_date = placeholder_value.get_untracked().map(|value| value.date());
    let placeholder_value = maybe(placeholder_value);
    let (granularity, hour_cycle) = (maybe(granularity), maybe(hour_cycle));
    let state = use_date_picker_state(UseDatePickerStateInput {
        default_value,
        value,
        on_change,
        placeholder_value,
        min_value,
        max_value,
        is_date_unavailable,
        granularity,
        hour_cycle,
        hide_time_zone,
        should_force_leading_zeros,
        should_close_on_select,
        default_open,
        is_open,
        on_open_change,
        is_invalid,
        validate,
        validation_behavior,
        name: name.clone(),
    });
    let label_presence = LabelPresence::new(aria_label, aria_labelledby.as_ref());
    let group = CapturedElement::new();
    let trigger = DialogTriggerContext::new(state.overlay, group);
    let UseDatePickerReturn {
        label_props,
        group_props,
        field_describedby,
        button,
        description_props,
        error_message_props,
        labelledby,
        dialog_labelledby,
        ..
    } = use_date_picker(UseDatePickerInput {
        state,
        group,
        options: DatePickerOptions {
            id,
            has_label: label_presence.has_label,
            aria_label,
            aria_labelledby,
            aria_describedby,
            is_disabled,
            is_read_only,
            on_focus_change,
            dialog_id: trigger.overlay_id.into(),
            ..DatePickerOptions::default()
        },
    });

    // The field, owned by the picker: its value and validation.
    let field_state = use_date_field_state(UseDateFieldStateInput {
        value: Some(state.binding),
        placeholder_value,
        min_value,
        max_value,
        granularity: Signal::derive(move || Some(state.granularity.get())),
        hour_cycle,
        hide_time_zone,
        should_force_leading_zeros,
        is_disabled,
        is_read_only,
        is_required,
        validation_behavior,
        name: name.clone(),
        validation: Some(state.validation),
        ..UseDateFieldStateInput::default()
    });
    let autofill = use_hidden_date_input(UseHiddenDateInputInput {
        value: state.value,
        base: field_state.date_value,
        granularity: state.granularity,
        set_value: Callback::new(move |value| state.set_value(value)),
        auto_complete,
        name,
        is_disabled: field_state.is_disabled,
    });
    let field_element = CapturedElement::new();
    let input_element = CapturedElement::new();
    let UseDateFieldReturn {
        field_props,
        input_props,
        data,
        ..
    } = use_date_field(UseDateFieldInput {
        state: field_state,
        element: field_element,
        input_element,
        options: DateFieldOptions {
            auto_focus,
            form,
            picker: Some(DateFieldPicker {
                overlay: state.overlay,
                focus_manager: None,
            }),
            ..DateFieldOptions::default()
        },
    });
    // The segments are labelled and described by the picker (react-aria: `useDatePicker`'s
    // field props).
    let data = DateFieldData {
        aria_labelledby: labelledby,
        aria_describedby: field_describedby,
        ..data
    };

    let is_disabled_state = field_state.is_disabled;
    let is_invalid = state.is_invalid;
    let validation = state.validation;
    let open = state.overlay.is_open;
    // The calendar's dates as the picker's values (on its time and zone).
    let picker_unavailable = is_date_unavailable.map(|unavailable| {
        Callback::new(move |date: jiff::civil::Date| unavailable.run(state.date_to_value(date)))
    });
    let provide = move || {
        provide_context(
            LabelContext::span(label_props.label)
                .with_presence(label_presence)
                .with_on_click(label_props.on_click),
        );
        provide_context(FieldContext {
            description: description_props,
            error_message: error_message_props,
            is_invalid,
            validation_errors: validation.validation_errors,
            validation_details: Signal::derive(move || {
                validation.display_validation.get().validation_details
            }),
        });
        provide_context(DateInputContext {
            segments: field_state.segments,
            field_props: StoredValue::new(Some(field_props)),
            segment: Arc::new(move |segment, element| {
                use_date_segment(UseDateSegmentInput {
                    segment,
                    data,
                    element,
                })
            }),
            is_disabled: field_state.is_disabled,
            is_read_only: field_state.is_read_only,
            is_invalid,
        });
        provide_context(DatePickerContext {
            group_props: StoredValue::new(Some(group_props)),
            button: StoredValue::new(button),
            is_open: open,
            is_disabled: is_disabled_state,
            is_invalid,
        });
        // The generic `Popover` and `Dialog` open from the group (react-aria-components'
        // `PopoverContext`), starting at its start edge.
        provide_context(trigger.with_dialog_labelledby(dialog_labelledby));
        provide_context(Some(PopoverDefaults {
            placement: Placement::BottomStart,
            offset: 8.0,
        }));
        provide_context(CalendarPickerContext {
            value: state.date_value,
            select: Callback::new(move |date| state.select_date(date)),
            min_value: Signal::derive(move || min_value.get().map(|value| value.date())),
            max_value: Signal::derive(move || max_value.get().map(|value| value.date())),
            is_date_unavailable: picker_unavailable,
            is_disabled: is_disabled_state,
            is_read_only: field_state.is_read_only,
            is_invalid,
            default_focused_value: placeholder_date,
        });
    };
    scoped_view(provide, move || {
        view! {
            <div
                class=classes
                style=styles
                data-disabled=flag(is_disabled_state)
                data-readonly=flag(field_state.is_read_only)
                data-required=flag(field_state.is_required)
                data-invalid=flag(is_invalid)
                data-open=flag(open)
            >
                {children()}
                <input {..input_props.into_attrs()} />
                {hidden_date_input(autofill)}
            </div>
        }
    })
}

/// The group of the [`DatePicker`]'s field and button (`role="group"`).
///
/// Data attributes: `data-hovered`, `data-focus-within`, `data-focus-visible`, `data-disabled`,
/// `data-invalid`, `data-open`.
///
/// Default class: `leptonic-DatePickerGroup`.
#[component]
pub fn DatePickerGroup(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let classes = with_default_class("leptonic-DatePickerGroup", classes);
    let Some(context) = use_context::<DatePickerContext>() else {
        crate::utils::dev_warn!("DatePickerGroup: not inside a DatePicker");
        return ().into_any();
    };
    let Some(props) = context.group_props.try_update_value(Option::take).flatten() else {
        crate::utils::dev_warn!("DatePickerGroup: used twice in one DatePicker");
        return ().into_any();
    };
    let (attrs, group_styles) = props.into_parts();
    let (group_attrs, hovered, focus_within, focus_visible) = group_state(context.is_disabled);
    view! {
        <div
            {..attrs}
            {..group_attrs}
            class=classes
            style=group_styles.merge(styles)
            data-hovered=hovered
            data-focus-within=focus_within
            data-focus-visible=focus_visible
            data-disabled=flag(context.is_disabled)
            data-invalid=flag(context.is_invalid)
            data-open=flag(context.is_open)
        >
            {children()}
        </div>
    }
    .into_any()
}

/// The button of the [`DatePicker`] opening its popover ("Calendar").
///
/// Data attributes: `data-pressed` (also while the popover is open), `data-hovered`, `data-focused`, `data-focus-visible`,
/// `data-disabled`.
///
/// Default class: `leptonic-DatePickerButton`.
#[component]
pub fn DatePickerButton(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let classes = with_default_class("leptonic-DatePickerButton", classes);
    let Some(context) = use_context::<DatePickerContext>() else {
        crate::utils::dev_warn!("DatePickerButton: not inside a DatePicker");
        return ().into_any();
    };
    let UseButtonReturn {
        props,
        is_disabled,
        is_pressed,
        is_hovered,
        is_focused,
        is_focus_visible,
        ..
    } = use_button(context.button.get_value());
    // Pressed while the popover is open (react-aria-components: `isPressed: state.isOpen`).
    let is_open = context.is_open;
    let is_pressed = Signal::derive(move || is_pressed.get() || is_open.get());
    let (attrs, button_styles) = props.into_parts();
    view! {
        <button
            {..attrs}
            class=classes
            style=button_styles.merge(styles)
            data-pressed=flag(is_pressed)
            data-hovered=flag(is_hovered)
            data-focused=flag(is_focused)
            data-focus-visible=flag(is_focus_visible)
            data-disabled=flag(is_disabled)
        >
            {children()}
        </button>
    }
    .into_any()
}

/// The start and end fields of a date range picker, for `DateInput part=..`.
#[derive(Clone)]
struct DateRangeInputContexts {
    start: DateInputContext,
    end: DateInputContext,
}

/// A date range picker (react-aria-components' `DateRangePicker`): a start and an end date field
/// with a button opening a range calendar in a popover. Put a `Label`, a `DatePickerGroup` (with
/// `DateInput part=RangePart::Start`, a separator, `DateInput part=RangePart::End` and a
/// `DatePickerButton`), a `Popover` with a `Dialog` and a `RangeCalendar`, and
/// `Description`/`FieldError` inside. With `start_name` and `end_name`, hidden inputs submit the
/// ends.
///
/// Data attributes: `data-disabled`, `data-readonly`, `data-required`, `data-invalid`, `data-open`.
///
/// Default class: `leptonic-DateRangePicker`.
#[component]
#[allow(
    clippy::too_many_arguments,
    clippy::too_many_lines,
    clippy::fn_params_excessive_bools
)]
pub fn DateRangePicker<V: DateValue>(
    #[prop(optional)] default_value: Option<RangeValue<V>>,
    #[prop(into, optional)] value: Option<Signal<Option<RangeValue<V>>>>,
    #[prop(into, optional)] set_value: Option<Out<Option<RangeValue<V>>>>,
    #[prop(into, optional)] on_change: Option<Callback<Option<RangeValue<V>>>>,
    /// The value the segments start from when edited, and the month the calendar opens on.
    #[prop(into, optional)]
    placeholder_value: MaybeProp<V>,
    #[prop(into, optional)] min_value: Signal<Option<V>>,
    #[prop(into, optional)] max_value: Signal<Option<V>>,
    #[prop(into, optional)] is_date_unavailable: Option<Callback<V, bool>>,
    /// Whether a range may span unavailable dates.
    #[prop(optional)]
    allows_non_contiguous_ranges: bool,
    #[prop(into, optional)] granularity: MaybeProp<Granularity>,
    #[prop(into, optional)] hour_cycle: MaybeProp<HourCycle>,
    #[prop(into, optional)] hide_time_zone: Signal<bool>,
    #[prop(into, optional)] should_force_leading_zeros: Signal<bool>,
    #[prop(into, default = Signal::stored(true))] should_close_on_select: Signal<bool>,
    #[prop(into, optional)] is_open: Option<Signal<bool>>,
    #[prop(into, optional)] set_open: Option<Out<bool>>,
    #[prop(optional)] default_open: bool,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    #[prop(into, optional)] is_disabled: Signal<bool>,
    #[prop(into, optional)] is_read_only: Signal<bool>,
    #[prop(into, optional)] is_required: Signal<bool>,
    #[prop(into, optional)] is_invalid: Signal<bool>,
    #[prop(optional)] validate: Option<ValidateFn<Option<RangeValue<V>>>>,
    #[prop(optional)] validation_behavior: Option<ValidationBehavior>,
    #[prop(into, optional)] start_name: Option<String>,
    #[prop(into, optional)] end_name: Option<String>,
    #[prop(into, optional)] form: Option<String>,
    #[prop(optional)] auto_focus: bool,
    #[prop(into, optional)] id: Option<String>,
    #[prop(into, optional)] aria_label: MaybeProp<String>,
    #[prop(into, optional)] aria_labelledby: Option<String>,
    #[prop(into, optional)] aria_describedby: Option<String>,
    #[prop(into, optional)] on_focus_change: Option<Callback<bool>>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let classes = with_default_class("leptonic-DateRangePicker", classes);
    let (value, on_change) = ValueBinding::from_state_props(value, set_value, on_change);
    let (is_open, on_open_change) =
        ValueBinding::from_state_props(is_open, set_open, on_open_change);
    let validation_behavior = use_validation_behavior(validation_behavior);
    let placeholder_date = placeholder_value.get_untracked().map(|value| value.date());
    let placeholder_value = maybe(placeholder_value);
    let (granularity, hour_cycle) = (maybe(granularity), maybe(hour_cycle));
    let state = use_date_range_picker_state(UseDateRangePickerStateInput {
        default_value,
        value,
        on_change,
        placeholder_value,
        min_value,
        max_value,
        is_date_unavailable,
        granularity,
        hour_cycle,
        hide_time_zone,
        should_force_leading_zeros,
        should_close_on_select,
        default_open,
        is_open,
        on_open_change,
        is_invalid,
        validate,
        validation_behavior,
        start_name: start_name.clone(),
        end_name: end_name.clone(),
    });
    let label_presence = LabelPresence::new(aria_label, aria_labelledby.as_ref());
    let group = CapturedElement::new();
    let trigger = DialogTriggerContext::new(state.overlay, group);
    let UseDatePickerReturn {
        label_props,
        group_props,
        field_describedby,
        button,
        description_props,
        error_message_props,
        labelledby,
        focus_manager,
        dialog_labelledby,
        ..
    } = use_date_range_picker(UseDateRangePickerInput {
        state,
        group,
        options: DatePickerOptions {
            id,
            has_label: label_presence.has_label,
            aria_label,
            aria_labelledby,
            aria_describedby,
            is_disabled,
            is_read_only,
            on_focus_change,
            dialog_id: trigger.overlay_id.into(),
            ..DatePickerOptions::default()
        },
    });

    // The start and end fields, owned by the picker: its ends and validation.
    let field = |part: RangePart, name: Option<String>, auto_focus: bool| {
        let value = match part {
            RangePart::Start => state.start,
            RangePart::End => state.end,
        };
        let field_state = use_date_field_state(UseDateFieldStateInput {
            value: Some(ValueBinding::new(
                value,
                Callback::new(move |value| state.set_date_time(part, value)),
            )),
            placeholder_value,
            min_value,
            max_value,
            granularity: Signal::derive(move || Some(state.granularity.get())),
            hour_cycle,
            hide_time_zone,
            should_force_leading_zeros,
            is_disabled,
            is_read_only,
            is_required,
            validation_behavior,
            name,
            validation: Some(state.validation),
            ..UseDateFieldStateInput::default()
        });
        let input_element = CapturedElement::new();
        let UseDateFieldReturn {
            field_props,
            input_props,
            data,
            ..
        } = use_date_field(UseDateFieldInput {
            state: field_state,
            element: CapturedElement::new(),
            input_element,
            options: DateFieldOptions {
                aria_label: MaybeProp::from(
                    match part {
                        RangePart::Start => "Start Date",
                        RangePart::End => "End Date",
                    }
                    .to_owned(),
                ),
                auto_focus,
                form: form.clone(),
                picker: Some(DateFieldPicker {
                    overlay: state.overlay,
                    focus_manager: Some(focus_manager.clone()),
                }),
                ..DateFieldOptions::default()
            },
        });
        let data = DateFieldData {
            aria_labelledby: labelledby,
            aria_describedby: field_describedby,
            ..data
        };
        (
            DateInputContext {
                segments: field_state.segments,
                field_props: StoredValue::new(Some(field_props)),
                segment: Arc::new(move |segment, element| {
                    use_date_segment(UseDateSegmentInput {
                        segment,
                        data,
                        element,
                    })
                }),
                is_disabled: field_state.is_disabled,
                is_read_only: field_state.is_read_only,
                is_invalid: state.is_invalid,
            },
            input_props,
        )
    };
    let (start_context, start_input) = field(RangePart::Start, start_name, auto_focus);
    let (end_context, end_input) = field(RangePart::End, end_name, false);

    let is_invalid = state.is_invalid;
    let validation = state.validation;
    let open = state.overlay.is_open;
    let is_disabled_state = start_context.is_disabled;
    let is_read_only_state = start_context.is_read_only;
    let picker_unavailable = is_date_unavailable.map(|unavailable| {
        Callback::new(move |query: DateAvailabilityQuery| {
            unavailable.run(state.date_to_value(query.date))
        })
    });
    let provide = move || {
        provide_context(
            LabelContext::span(label_props.label)
                .with_presence(label_presence)
                .with_on_click(label_props.on_click),
        );
        provide_context(FieldContext {
            description: description_props,
            error_message: error_message_props,
            is_invalid,
            validation_errors: validation.validation_errors,
            validation_details: Signal::derive(move || {
                validation.display_validation.get().validation_details
            }),
        });
        provide_context(DateRangeInputContexts {
            start: start_context,
            end: end_context,
        });
        provide_context(DatePickerContext {
            group_props: StoredValue::new(Some(group_props)),
            button: StoredValue::new(button),
            is_open: open,
            is_disabled: is_disabled_state,
            is_invalid,
        });
        provide_context(trigger.with_dialog_labelledby(dialog_labelledby));
        provide_context(Some(PopoverDefaults {
            placement: Placement::BottomStart,
            offset: 8.0,
        }));
        provide_context(RangeCalendarPickerContext {
            value: state.date_range,
            select: Callback::new(move |range| state.select_range(range)),
            min_value: Signal::derive(move || min_value.get().map(|value| value.date())),
            max_value: Signal::derive(move || max_value.get().map(|value| value.date())),
            is_date_unavailable: picker_unavailable,
            allows_non_contiguous_ranges,
            is_disabled: is_disabled_state,
            is_read_only: is_read_only_state,
            is_invalid,
            default_focused_value: placeholder_date,
        });
    };
    scoped_view(provide, move || {
        view! {
            <div
                class=classes
                style=styles
                data-disabled=flag(is_disabled_state)
                data-readonly=flag(is_read_only_state)
                data-required=flag(is_required)
                data-invalid=flag(is_invalid)
                data-open=flag(open)
            >
                {children()}
                <input {..start_input.into_attrs()} />
                <input {..end_input.into_attrs()} />
            </div>
        }
    })
}
