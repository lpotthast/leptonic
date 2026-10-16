//! The `DateField` alone (`#perf-field`, a date labelled "Birthday", its value app state), for
//! its performance test (`leptonic/tests/perf_tests/atoms/date_field.rs`): its `Label` and
//! `DateInput` with a `DateSegment` per segment (month, day, year), between two plain buttons
//! (`#perf-before`, `#perf-after`) to tab from and to, mounted at first and empty. Its segments
//! start from June 12, 2024 when stepped (`placeholder_value`; never today, so that the counts
//! don't depend on the day the tests run). Its controls ([`Control`]) mount and unmount it,
//! change its reactive props, and read its value (`value`: `yyyymmdd`, 0 without).
//!
//! The page creates the live announcer on load (`announce("")`): a focused segment announces its
//! changes, and the first announcement of a page waits 100 ms for the live regions.

use leptonic::{
    Assertiveness, announce,
    atoms::{
        datepicker::{DateField, DateInput, DateSegment},
        field::Label,
    },
    jiff::civil::{Date, date},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// Where the segments start when stepped, and the placeholder after the change.
const PLACEHOLDER: Date = date(2024, 6, 12);
const OTHER_PLACEHOLDER: Date = date(2030, 1, 20);

/// `date` as `yyyymmdd`, for the read controls (0: none).
fn number(date: Option<Date>) -> f64 {
    date.map_or(0.0, |date| {
        f64::from(date.year()) * 10_000.0 + f64::from(date.month()) * 100.0 + f64::from(date.day())
    })
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let value = RwSignal::new(None::<Date>);
    let placeholder = RwSignal::new(PLACEHOLDER);
    let min_value = RwSignal::new(None::<Date>);
    let max_value = RwSignal::new(None::<Date>);
    let disabled = RwSignal::new(false);
    let read_only = RwSignal::new(false);
    let required = RwSignal::new(false);
    let invalid = RwSignal::new(false);
    let leading_zeros = RwSignal::new(false);
    Effect::new(|| announce("", Assertiveness::Polite));
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("set-value", move || value.set(Some(date(2024, 6, 5)))),
        Control::action("clear-value", move || value.set(None)),
        Control::action("move-placeholder", move || {
            placeholder.set(OTHER_PLACEHOLDER);
        }),
        Control::action("placeholder-back", move || placeholder.set(PLACEHOLDER)),
        Control::action("set-min", move || min_value.set(Some(date(2025, 1, 1)))),
        Control::action("set-max", move || max_value.set(Some(date(2024, 1, 1)))),
        Control::action("clear-limits", move || {
            min_value.set(None);
            max_value.set(None);
        }),
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
        <h1>"DateField performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <DateField
                id="perf-field"
                value=value
                set_value=value
                placeholder_value=placeholder
                min_value=min_value
                max_value=max_value
                is_disabled=disabled
                is_read_only=read_only
                is_required=required
                is_invalid=invalid
                should_force_leading_zeros=leading_zeros
            >
                <Label>"Birthday"</Label>
                <DateInput children=|segment| view! { <DateSegment segment=segment /> } />
            </DateField>
        </Show>
        <button id="perf-after">"After"</button>
    }
}
