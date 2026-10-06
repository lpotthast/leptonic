use leptonic::{
    components::prelude::*,
    hooks::{PlacementX, PlacementY, *},
    prelude::icondata,
    utils::time::Day,
};
use leptos::{html, portal::Portal, prelude::*};
use leptos_use::use_document;
use time::OffsetDateTime;
use wasm_bindgen::JsCast;

use super::date_segments::{DateSegments, SegmentControls};

#[component]
pub fn DatePickerDemo() -> impl IntoView {
    let picker = use_date_picker(UseDatePickerInput {
        label: Some("Departure".to_owned()),
        ..Default::default()
    });
    let state = picker.state;

    // The field edits the picker's value.
    let field = use_date_field(UseDateFieldInput {
        value: state.value,
        on_change: Some(state.set_value),
        is_date_picker: true,
        ..Default::default()
    });

    let controls = SegmentControls::from(&field);

    let UsePopoverReturn {
        props,
        trigger_props,
        ..
    } = use_popover(UsePopoverInput {
        placement_x: Signal::stored(PlacementX::Start),
        placement_y: Signal::stored(PlacementY::Below),
        offset: 4.0.into(),
        cross_offset: 0.0.into(),
        container_padding: 12.0.into(),
        should_flip: true.into(),
        modality: PopoverModality::Modal,
        is_keyboard_dismiss_disabled: false,
        should_close_on_interact_outside: None,
        ..UsePopoverInput::new(OverlayTriggerState::from(ValueBinding::new(
            picker.is_open,
            Callback::new(move |open: bool| {
                if !open {
                    picker.close.run(());
                }
            }),
        )))
    });

    // `<Show>` may render its children more than once, so keep the attributes in stored values.
    let (popover_attrs, popover_styles) = props.into_parts();
    let popover_attrs = StoredValue::new(popover_attrs);
    let popover_styles = StoredValue::new(popover_styles);
    let dialog = StoredValue::new(picker.dialog_props);
    let UseDatePickerCalendarProps {
        value: calendar_value,
        on_change: calendar_on_change,
        min,
        max,
        default_focused_value,
        ..
    } = picker.calendar_props;

    // When the popover closed while it held the focus (the focus fell back to the body once it was removed), return
    // the focus to the button.
    let button = NodeRef::<html::Button>::new();
    let is_open = picker.is_open;
    Effect::new(move |was_open: Option<bool>| {
        let open = is_open.get();
        if was_open == Some(true) && !open {
            request_animation_frame(move || {
                let focus_lost = use_document()
                    .active_element()
                    .is_none_or(|active| active.tag_name().eq_ignore_ascii_case("body"));
                if focus_lost && let Some(button) = button.get_untracked() {
                    let _ = button.focus();
                }
            });
        }
        open
    });

    let value = picker.value;

    view! {
        <div {..trigger_props.into_attrs()} class="demo-date-picker">
            <div {..picker.group_props.into_attrs()} class="demo-date-picker-group">
                <span {..picker.label_props.into_attrs()} class="demo-date-field-label">"Departure"</span>
                // Both prop sets carry an `id` and a `role`: spread them on separate elements.
                <div {..picker.field_props.into_attrs()} class="demo-date-field-input demo-date-picker-input">
                    <div {..field.field_props.into_attrs()} class="demo-date-picker-segments">
                        <DateSegments controls is_disabled=false is_read_only=false is_invalid=picker.is_invalid/>
                    </div>
                    <button {..picker.button_props.into_attrs()} node_ref=button class="demo-date-picker-button">
                        "\u{1F4C5}"
                    </button>
                </div>
            </div>
        </div>

        <Portal>
            <Show when=move || is_open.get()>
                <div class="demo-popover-underlay"/>
                <div {..popover_attrs.get_value()} style=popover_styles.get_value() class="demo-date-picker-popover">
                    <div
                        id=dialog.with_value(|dialog| dialog.id.clone())
                        role=dialog.with_value(|dialog| dialog.role)
                        aria-modal=dialog.with_value(|dialog| dialog.aria_modal)
                        aria-labelledby=dialog.with_value(|dialog| dialog.aria_labelledby.clone())
                    >
                        <PickerCalendar
                            value=calendar_value
                            on_change=calendar_on_change
                            min
                            max
                            default_focused_value
                        />
                    </div>
                </div>
            </Show>
        </Portal>

        <p class="demo-mt-half">
            "Value: "
            <code>{move || value.get().map_or_else(|| "None".to_owned(), |date| date.date().to_string())}</code>
        </p>
    }
}

/// A single-month calendar built from the calendar hooks, configured from the picker's `calendar_props`.
#[component]
fn PickerCalendar(
    value: Signal<Option<OffsetDateTime>>,
    on_change: Callback<OffsetDateTime>,
    min: Option<OffsetDateTime>,
    max: Option<OffsetDateTime>,
    default_focused_value: Option<OffsetDateTime>,
) -> impl IntoView {
    let calendar = use_calendar_state(UseCalendarStateInput {
        default_value: value.get_untracked(),
        min,
        max,
        default_focused_value,
        on_change: Some(Callback::new(move |date: Option<OffsetDateTime>| {
            if let Some(date) = date {
                on_change.run(date);
            }
        })),
        ..Default::default()
    });
    let grid = use_calendar_grid(UseCalendarGridInput::from_calendar_state(calendar));

    // Focus the focused date when the calendar opens, and follow it while the grid has the focus.
    let grid_element = NodeRef::<html::Table>::new();
    Effect::new(move |previous: Option<()>| {
        calendar.focused_date.track();
        request_animation_frame(move || {
            let Some(grid) = grid_element.get_untracked() else {
                return;
            };
            let active = use_document().active_element();
            let grid_has_focus = active
                .as_ref()
                .is_some_and(|active| grid.contains(Some(active)))
                || active.is_none_or(|active| active.tag_name().eq_ignore_ascii_case("body"));
            if (previous.is_none() || grid_has_focus)
                && let Ok(Some(cell)) = grid.query_selector("[tabindex='0']")
                && let Some(cell) = cell.dyn_ref::<web_sys::HtmlElement>()
            {
                let _ = cell.focus();
            }
        });
    });

    // Rows are keyed by month and first day, so that the cells (and the focused one) survive moving the focus within
    // a month.
    let weeks = move || {
        let month = calendar.focused_date.get().month();
        calendar
            .weeks
            .get()
            .into_iter()
            .map(move |week| (month, week))
            .collect::<Vec<_>>()
    };
    let weekday_labels = grid.weekday_labels;

    view! {
        <div class="demo-date-picker-calendar-header">
            <Button
                on_press=move |_| calendar.focus_previous_page.run(())
                variant=ButtonVariant::Flat
                attr:aria-label="Previous month"
            >
                <Icon icon=icondata::BsChevronLeft/>
            </Button>
            <span class="demo-date-picker-month">
                {move || format!("{} {}", calendar.focused_month_name.get(), calendar.focused_year.get())}
            </span>
            <Button
                on_press=move |_| calendar.focus_next_page.run(())
                variant=ButtonVariant::Flat
                attr:aria-label="Next month"
            >
                <Icon icon=icondata::BsChevronRight/>
            </Button>
        </div>
        <table {..grid.grid_props.into_attrs()} node_ref=grid_element class="demo-date-picker-grid">
            <thead>
                <tr {..grid.header_props.into_attrs()}>
                    {weekday_labels.into_iter().map(|label| view! { <th>{label}</th> }).collect_view()}
                </tr>
            </thead>
            <tbody>
                <For
                    each=weeks
                    key=|(month, week)| (*month, week.days.first().map(|day| day.date_time.date()))
                    children=move |(_, week)| {
                        view! {
                            <tr>
                                {week.days.into_iter().map(|day| view! { <CalendarCell calendar day/> }).collect_view()}
                            </tr>
                        }
                    }
                />
            </tbody>
        </table>
    }
}

#[component]
fn CalendarCell(calendar: UseCalendarStateReturn, day: Day) -> impl IntoView {
    let date = day.date_time;
    let cell = use_calendar_cell(UseCalendarCellInput {
        day,
        is_focused: Signal::derive(move || calendar.is_cell_focused.run(date)),
        is_selected: Signal::derive(move || calendar.is_selected.run(date)),
        is_disabled: calendar.is_disabled,
        on_select: Some(Callback::new(move |day: Day| {
            calendar.select_date.run(day.date_time);
        })),
        on_focus: Some(Callback::new(move |day: Day| {
            calendar.set_focused_date.run(day.date_time);
        })),
    });

    view! {
        <td {..cell.cell_props.into_attrs()} class="demo-date-picker-cell">
            <span
                {..cell.button_props.into_attrs()}
                class="demo-date-picker-day"
                data-outside-month=cell.is_outside_month.then_some("")
                data-today=cell.is_today.then_some("")
            >
                {cell.formatted_date}
            </span>
        </td>
    }
}
