use leptonic::{
    atoms::{
        calendar::{
            Calendar, CalendarCell, CalendarCellButton, CalendarGrid, CalendarGridBody,
            CalendarGridHeader, CalendarHeaderCell, CalendarHeaderRow, CalendarMonthPicker,
            CalendarNextButton, CalendarPreviousButton, CalendarWeek, CalendarYearPicker,
        },
        listbox::{ListBox, ListBoxItems},
        select::{Select, SelectPopover, SelectTrigger, SelectValue},
    },
    hooks::{
        calendar::{CalendarPickerItem, UseCalendarPickerReturn},
        collections::{Key, UseListCollectionInput, use_list_collection},
    },
    jiff::civil::date,
};
use leptos::prelude::*;

#[component]
pub fn CalendarPickersDemo() -> impl IntoView {
    let value = RwSignal::new(Some(date(1990, 6, 15)));

    view! {
        <Calendar value set_value=value aria_label="Birthday" classes="demo-calendar">
            <header class="demo-calendar-header">
                <CalendarPreviousButton classes="demo-calendar-nav">
                    <span aria-hidden="true">"\u{2039}"</span>
                </CalendarPreviousButton>
                // The pickers move the focused date to the chosen month or year.
                <CalendarMonthPicker children=|picker| view! { <PickerSelect picker/> }/>
                <CalendarYearPicker visible_years=60 children=|picker| view! { <PickerSelect picker/> }/>
                <CalendarNextButton classes="demo-calendar-nav">
                    <span aria-hidden="true">"\u{203a}"</span>
                </CalendarNextButton>
            </header>
            <CalendarGrid classes="demo-calendar-grid">
                <CalendarGridHeader>
                    <CalendarHeaderRow children=|day| view! { <CalendarHeaderCell>{day}</CalendarHeaderCell> }/>
                </CalendarGridHeader>
                <CalendarGridBody children=|week| {
                    view! {
                        <CalendarWeek
                            week
                            children=|date| {
                                view! {
                                    <CalendarCell date>
                                        <CalendarCellButton classes="demo-calendar-day"/>
                                    </CalendarCell>
                                }
                            }
                        />
                    }
                }/>
            </CalendarGrid>
        </Calendar>

        <p class="demo-status">
            {move || value.get().map_or_else(|| "No date selected.".to_owned(), |date| format!("Selected: {date}."))}
        </p>
    }
}

/// A month or year picker as a `Select`: its options are the picker's items, keyed by their ids.
#[component]
fn PickerSelect(picker: UseCalendarPickerReturn) -> impl IntoView {
    let UseCalendarPickerReturn {
        aria_label,
        value,
        items,
        on_change,
    } = picker;
    let options = use_list_collection(UseListCollectionInput {
        items,
        key: |item: &CalendarPickerItem| Key::from(item.id),
        text_value: |item: &CalendarPickerItem| item.formatted.clone(),
    });

    view! {
        <Select
            collection=options
            value={Signal::derive(move || Some(value.get()))}
            set_value={move |id: Option<i16>| {
                if let Some(id) = id {
                    on_change.run(id);
                }
            }}
            aria_label=aria_label
            classes="demo-calendar-picker"
        >
            <SelectTrigger classes="demo-sel-trigger">
                <SelectValue classes="demo-sel-value"/>
                <span class="demo-sel-caret" aria-hidden="true">"\u{25bc}"</span>
            </SelectTrigger>
            <SelectPopover classes="demo-sel-popover">
                <ListBox classes="demo-sel-listbox">
                    <ListBoxItems classes="demo-sel-item" let:node>{node.text_value.to_string()}</ListBoxItems>
                </ListBox>
            </SelectPopover>
        </Select>
    }
}
