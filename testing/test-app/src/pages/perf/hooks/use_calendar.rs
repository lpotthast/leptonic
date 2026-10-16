//! `use_calendar` on plain elements, for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_calendar.rs`): a calendar of June 2024 (its state from
//! `use_calendar_state`, June 12 focused; fixed dates, never today), its element
//! (`#perf-calendar`), its title (`#perf-title`) and its previous and next buttons (`use_button`
//! with the inputs it returns), between two plain buttons (`#perf-before`, `#perf-after`),
//! mounted at first. Its controls ([`Control`]) mount and unmount it, select a date through its
//! state (the calendar announces the selection), and change its inputs (label, disabled,
//! maximum).
//!
//! The page creates the live announcer on load (`announce("")`): the calendar announces its
//! visible range and selection, and the first announcement of a page waits 100 ms for the live
//! regions.

use leptonic::{
    Assertiveness, IntoAttrs, announce,
    hooks::{
        button::use_button,
        calendar::{
            CalendarState, UseCalendarInput, UseCalendarReturn, UseCalendarStateInput,
            use_calendar, use_calendar_state,
        },
    },
    jiff::civil::{Date, date},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The calendar's inputs, app state the controls change.
#[derive(Clone, Copy)]
struct Inputs {
    label: RwSignal<String>,
    disabled: RwSignal<bool>,
    max_value: RwSignal<Option<Date>>,
}

/// The calendar: its state, `use_calendar`, and the elements it returns props for.
#[component]
fn HookCalendar(inputs: Inputs, state: StoredValue<Option<CalendarState>>) -> impl IntoView {
    let calendar_state = use_calendar_state(UseCalendarStateInput {
        default_focused_value: Some(date(2024, 6, 12)),
        is_disabled: inputs.disabled.into(),
        max_value: inputs.max_value.into(),
        ..UseCalendarStateInput::default()
    });
    state.set_value(Some(calendar_state));
    on_cleanup(move || state.set_value(None));
    let UseCalendarReturn {
        calendar_props,
        previous_button,
        next_button,
        title,
        ..
    } = use_calendar(UseCalendarInput {
        state: calendar_state,
        id: Some("perf-calendar".to_owned()),
        aria_label: inputs.label.into(),
        aria_labelledby: Signal::default(),
        aria_describedby: Signal::default(),
        aria_details: Signal::default(),
    });
    let (previous_attrs, previous_styles) = use_button(previous_button).props.into_parts();
    let (next_attrs, next_styles) = use_button(next_button).props.into_parts();
    view! {
        <div {..calendar_props.into_attrs()}>
            <button {..previous_attrs} style=previous_styles>
                "<"
            </button>
            <h2 id="perf-title">{title}</h2>
            <button {..next_attrs} style=next_styles>
                ">"
            </button>
        </div>
    }
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let state = StoredValue::new(None::<CalendarState>);
    let inputs = Inputs {
        label: RwSignal::new("Event date".to_owned()),
        disabled: RwSignal::new(false),
        max_value: RwSignal::new(None),
    };
    Effect::new(|| announce("", Assertiveness::Polite));
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("select-june-14", move || {
            if let Some(state) = state.get_value() {
                state.select_date(date(2024, 6, 14));
            }
        }),
        Control::action("relabel", move || {
            inputs.label.set("Delivery date".to_owned());
        }),
        Control::action("label-back", move || {
            inputs.label.set("Event date".to_owned());
        }),
        Control::action("disable", move || inputs.disabled.set(true)),
        Control::action("enable", move || inputs.disabled.set(false)),
        Control::action("set-max", move || {
            inputs.max_value.set(Some(date(2024, 6, 25)));
        }),
        Control::action("clear-max", move || inputs.max_value.set(None)),
    ];
    view! {
        <h1>"use_calendar performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <HookCalendar inputs=inputs state=state />
        </Show>
        <button id="perf-after">"After"</button>
    }
}
