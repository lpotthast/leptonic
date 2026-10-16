//! `DateSegment`s in a `DateField` (`#perf-field`, a date labelled "Birthday", its value app
//! state), for their performance test (`leptonic/tests/perf_tests/atoms/date_segment.rs`): a
//! segment per part of June 5, 2024 (month, day, year and the literals between them), between
//! two plain buttons (`#perf-before`, `#perf-after`) to tab from and to, mounted at first. Its
//! segments start from June 12, 2024 when stepped empty (never today). Its controls
//! ([`Control`]) mount and unmount the segments (the field and its input stay), clear the value
//! and set it again, disable and enable the field, and read its value (`value`: `yyyymmdd`, 0
//! without).
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

/// The field's date at first.
const VALUE: Date = date(2024, 6, 5);

/// `date` as `yyyymmdd`, for the read controls (0: none).
fn number(date: Option<Date>) -> f64 {
    date.map_or(0.0, |date| {
        f64::from(date.year()) * 10_000.0 + f64::from(date.month()) * 100.0 + f64::from(date.day())
    })
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let value = RwSignal::new(Some(VALUE));
    let disabled = RwSignal::new(false);
    Effect::new(|| announce("", Assertiveness::Polite));
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("clear-value", move || value.set(None)),
        Control::action("set-value", move || value.set(Some(VALUE))),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
        Control::read("value", move || number(value.get_untracked())),
    ];
    view! {
        <h1>"DateSegment performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <DateField
            id="perf-field"
            value=value
            set_value=value
            placeholder_value=date(2024, 6, 12)
            is_disabled=disabled
        >
            <Label>"Birthday"</Label>
            <DateInput children=move |segment| {
                view! {
                    <Show when=move || mounted.get()>
                        <DateSegment segment=segment />
                    </Show>
                }
            } />
        </DateField>
        <button id="perf-after">"After"</button>
    }
}
