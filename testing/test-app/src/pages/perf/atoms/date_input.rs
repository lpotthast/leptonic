//! A `DateInput` in a `DateField` (`#perf-field`, a date and time labelled "Departure"), for its
//! performance test (`leptonic/tests/perf_tests/atoms/date_input.rs`): its `DateSegment`s, on
//! June 5, 2024, 9:45 AM, between two plain buttons (`#perf-before`, `#perf-after`) to tab from
//! and to, mounted at first. A field's input can't mount again in the same field, so its
//! controls ([`Control`]) mount and unmount the field; they also switch the granularity between
//! days and minutes (the segments change) and change the states the input shows.

use leptonic::{
    atoms::{
        datepicker::{DateField, DateInput, DateSegment},
        field::Label,
    },
    hooks::datepicker::Granularity,
    jiff::civil::{DateTime, date},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let value = RwSignal::new(Some::<DateTime>(date(2024, 6, 5).at(9, 45, 0, 0)));
    let granularity = RwSignal::new(None::<Granularity>);
    let disabled = RwSignal::new(false);
    let read_only = RwSignal::new(false);
    let invalid = RwSignal::new(false);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("show-days", move || granularity.set(Some(Granularity::Day))),
        Control::action("show-minutes", move || granularity.set(None)),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
        Control::action("make-read-only", move || read_only.set(true)),
        Control::action("make-editable", move || read_only.set(false)),
        Control::action("invalidate", move || invalid.set(true)),
        Control::action("validate", move || invalid.set(false)),
    ];
    view! {
        <h1>"DateInput performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <DateField
                id="perf-field"
                value=value
                set_value=value
                granularity=granularity
                is_disabled=disabled
                is_read_only=read_only
                is_invalid=invalid
            >
                <Label>"Departure"</Label>
                <DateInput children=|segment| view! { <DateSegment segment=segment /> } />
            </DateField>
        </Show>
        <button id="perf-after">"After"</button>
    }
}
