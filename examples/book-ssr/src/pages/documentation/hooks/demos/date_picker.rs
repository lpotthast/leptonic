use leptonic::{
    atoms::{
        calendar::{
            Calendar, CalendarCell, CalendarCellButton, CalendarGrid, CalendarGridBody,
            CalendarGridHeader, CalendarHeaderCell, CalendarHeaderRow, CalendarHeading,
            CalendarNextButton, CalendarPreviousButton, CalendarWeek,
        },
        popover::Popover,
    },
    components::prelude::{Checkbox, Icon},
    hooks::{
        IntoAttrs, Placement, UseButtonReturn,
        datepicker::{
            DateFieldData, DateSegment, DateSegmentType, UseDateFieldInput, UseDateFieldReturn,
            UseDateFieldStateInput, UseDatePickerInput, UseDatePickerReturn,
            UseDatePickerStateInput, UseDateSegmentReturn, use_date_field, use_date_field_state,
            use_date_picker, use_date_picker_state, use_date_segment,
        },
        use_button,
    },
    jiff::civil::Date,
    prelude::{ValueBinding, icondata},
    utils::{CapturedElement, id::use_id},
};
use leptos::prelude::*;

#[component]
pub fn DatePickerHookDemo() -> impl IntoView {
    let disabled = RwSignal::new(false);
    let state = use_date_picker_state(UseDatePickerStateInput::<Date>::default());
    let group = CapturedElement::new();
    // The dialog in the popover: moving the focus into it doesn't leave the picker.
    let dialog_id = use_id("departure-dialog");
    let UseDatePickerReturn {
        label_props,
        group_props,
        field_describedby,
        labelledby,
        button,
        dialog_labelledby,
        ..
    } = use_date_picker(
        UseDatePickerInput {
            has_label: true.into(),
            is_disabled: disabled.into(),
            dialog_id: Signal::stored(Some(dialog_id.clone())),
            ..UseDatePickerInput::default()
        },
        state,
        group,
    );

    // The field edits the picker's value and shares its validation.
    let field_state = use_date_field_state(UseDateFieldStateInput {
        value: Some(ValueBinding::new(
            state.value,
            Callback::new(move |value| state.set_value(value)),
        )),
        granularity: Some(state.granularity),
        is_disabled: disabled.into(),
        validation: Some(state.validation),
        ..UseDateFieldStateInput::default()
    });
    let UseDateFieldReturn {
        field_props,
        input_props,
        mut data,
        ..
    } = use_date_field(
        UseDateFieldInput {
            is_in_picker: true,
            // Alt + ArrowDown in the field.
            open: Some(Callback::new(move |()| state.set_open(true))),
            ..UseDateFieldInput::default()
        },
        field_state,
        CapturedElement::new(),
        CapturedElement::new(),
    );
    // The segments are named and described by the picker: its label, its description and its value.
    data.aria_labelledby = labelledby;
    data.aria_describedby = field_describedby;

    let UseButtonReturn {
        props: button_props,
        ..
    } = use_button(button);
    let (group_attrs, group_styles) = group_props.into_parts();
    let (field_attrs, field_styles) = field_props.into_parts();
    let (button_attrs, button_styles) = button_props.into_parts();
    let segments = field_state.segments;
    let dialog_labelledby = dialog_labelledby.get_untracked();

    view! {
        <div class="demo-date-field">
            <span {..label_props.into_attrs()} class="demo-field-label">"Departure"</span>
            <div {..group_attrs} style=group_styles class="demo-date-input">
                <div {..field_attrs} style=field_styles class="demo-date-segments">
                    <For
                        each=move || {
                            segments.with(|segments| {
                                segments.iter().enumerate().map(|(index, segment)| (index, segment.kind)).collect::<Vec<_>>()
                            })
                        }
                        key=|key| *key
                        children=move |(index, kind)| {
                            let initial = segments.with_untracked(|segments| segments[index].clone());
                            let segment = Signal::derive(move || {
                                segments.with(|segments| {
                                    segments.get(index).filter(|segment| segment.kind == kind).cloned()
                                })
                                .unwrap_or_else(|| initial.clone())
                            });
                            view! { <Segment segment data/> }
                        }
                    />
                </div>
                <button {..button_attrs} style=button_styles class="demo-date-picker-button">
                    <Icon icon=icondata::BsCalendar3/>
                </button>
            </div>
            <input {..input_props.into_attrs()}/>
        </div>

        // Positioned at the group. It keeps the focus inside while open and returns it when it closes.
        <Popover
            is_open=state.overlay.is_open
            set_open={move |is_open| state.set_open(is_open)}
            trigger=group
            placement=Placement::BottomStart
            classes="demo-date-picker-popover"
        >
            <div id=dialog_id.clone() role="dialog" aria-labelledby=dialog_labelledby.clone()>
                // Created on every opening: it starts at the picker's date, with the focus.
                <Calendar
                    value=state.date_value
                    on_change={move |date: Option<Date>| {
                        if let Some(date) = date {
                            state.select_date(date);
                        }
                    }}
                    auto_focus=true
                    classes="demo-date-picker-calendar"
                >
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
                </Calendar>
            </div>
        </Popover>

        <p class="demo-status">
            {move || {
                state.value.get().map_or_else(
                    || "No departure date".to_owned(),
                    |date| format!("Departure on {}", date.strftime("%A, %B %-d, %Y")),
                )
            }}
        </p>

        <div class="demo-controls">
            <Checkbox is_selected=disabled set_selected=disabled>"Disabled"</Checkbox>
        </div>
    }
}

/// An editable segment, or a literal between them.
#[component]
fn Segment(segment: Signal<DateSegment>, data: DateFieldData<Date>) -> impl IntoView {
    let kind = segment.with_untracked(|segment| segment.kind);
    let text = move || segment.with(|segment| segment.text.clone());
    if kind == DateSegmentType::Literal {
        return view! {
            <span aria-hidden="true" class="demo-date-segment" data-type=kind.as_str()>{text}</span>
        }
        .into_any();
    }
    let UseDateSegmentReturn { segment_props } =
        use_date_segment(segment, data, CapturedElement::new());
    let (attrs, styles) = segment_props.into_parts();
    view! {
        <span {..attrs} style=styles class="demo-date-segment" data-type=kind.as_str()>{text}</span>
    }
    .into_any()
}
