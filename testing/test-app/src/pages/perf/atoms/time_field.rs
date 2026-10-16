//! The `TimeField` alone (`#perf-field`, a time of day labelled "Pickup", its value app state),
//! for its performance test (`leptonic/tests/perf_tests/atoms/time_field.rs`): its `Label` and
//! `DateInput` with a `DateSegment` per segment (hour, minute, AM/PM in the test-app's en-US),
//! between two plain buttons (`#perf-before`, `#perf-after`) to tab from and to, mounted at first
//! and empty. Its controls ([`Control`]) mount and unmount it, change its reactive props, and read
//! its value (`value`: `hhmmss`, -1 without).
//!
//! The page creates the live announcer on load (`announce("")`): a focused segment announces its
//! changes, and the first announcement of a page waits 100 ms for the live regions.

use leptonic::{
    Assertiveness, HourCycle, announce,
    atoms::{
        datepicker::{DateInput, DateSegment, TimeField},
        field::Label,
    },
    hooks::datepicker::{Granularity, TimeBound},
    jiff::civil::{Time, time},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// `time` as `hhmmss`, for the read controls (-1: none).
fn number(time: Option<Time>) -> f64 {
    time.map_or(-1.0, |time| {
        f64::from(time.hour()) * 10_000.0
            + f64::from(time.minute()) * 100.0
            + f64::from(time.second())
    })
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let value = RwSignal::new(None::<Time>);
    let min_value = RwSignal::new(None::<TimeBound<Time>>);
    let max_value = RwSignal::new(None::<TimeBound<Time>>);
    let granularity = RwSignal::new(None::<Granularity>);
    let hour_cycle = RwSignal::new(None::<HourCycle>);
    let disabled = RwSignal::new(false);
    let read_only = RwSignal::new(false);
    let required = RwSignal::new(false);
    let invalid = RwSignal::new(false);
    let leading_zeros = RwSignal::new(false);
    Effect::new(|| announce("", Assertiveness::Polite));
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("set-value", move || value.set(Some(time(8, 30, 0, 0)))),
        Control::action("clear-value", move || value.set(None)),
        Control::action("set-min", move || {
            min_value.set(Some(TimeBound::TimeOfDay(time(9, 0, 0, 0))));
        }),
        Control::action("set-max", move || {
            max_value.set(Some(TimeBound::TimeOfDay(time(8, 0, 0, 0))));
        }),
        Control::action("clear-limits", move || {
            min_value.set(None);
            max_value.set(None);
        }),
        Control::action("show-seconds", move || {
            granularity.set(Some(Granularity::Second));
        }),
        Control::action("show-minutes", move || granularity.set(None)),
        Control::action("use-24-hours", move || hour_cycle.set(Some(HourCycle::H24))),
        Control::action("use-locale-hours", move || hour_cycle.set(None)),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
        Control::action("make-read-only", move || read_only.set(true)),
        Control::action("make-editable", move || read_only.set(false)),
        Control::action("require", move || required.set(true)),
        Control::action("make-optional", move || required.set(false)),
        Control::action("invalidate", move || invalid.set(true)),
        Control::action("validate", move || invalid.set(false)),
        Control::action("force-leading-zeros", move || leading_zeros.set(true)),
        Control::action("no-leading-zeros", move || leading_zeros.set(false)),
        Control::read("value", move || number(value.get_untracked())),
    ];
    view! {
        <h1>"TimeField performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <TimeField
                id="perf-field"
                value=value
                set_value=value
                min_value=min_value
                max_value=max_value
                granularity=granularity
                hour_cycle=hour_cycle
                is_disabled=disabled
                is_read_only=read_only
                is_required=required
                is_invalid=invalid
                should_force_leading_zeros=leading_zeros
            >
                <Label>"Pickup"</Label>
                <DateInput children=|segment| view! { <DateSegment segment=segment /> } />
            </TimeField>
        </Show>
        <button id="perf-after">"After"</button>
    }
}
