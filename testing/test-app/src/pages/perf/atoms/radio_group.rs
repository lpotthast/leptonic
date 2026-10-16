//! The `RadioGroup` (`#perf-radio-group`) of 10 or 1,000 radios (`RadioField`s with their
//! `RadioButton`s, valued 0, 1, ...), for its performance test
//! (`leptonic/tests/perf_tests/atoms/radio_group.rs`): vertical, its value controlled by app
//! state (`value` + `set_value`, nothing selected at first), between two plain buttons
//! (`#perf-before`, `#perf-after`), mounted with 10 radios at first. Its controls ([`Control`])
//! mount it (with 10 or 1,000 radios) and unmount it, select the first or the last radio or none,
//! change its reactive props, and read the selected value (`selected`, `-1` for none).

use leptonic::{
    Orientation,
    atoms::radio::{RadioButton, RadioField, RadioGroup},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The radios of the small group.
const SMALL: usize = 10;
/// The radios of the large group.
const LARGE: usize = 1_000;

#[component]
pub fn PagePerf() -> impl IntoView {
    // The number of radios of the mounted group.
    let size = RwSignal::new(Some(SMALL));
    let value = RwSignal::new(None::<usize>);
    let orientation = RwSignal::new(Orientation::Vertical);
    let disabled = RwSignal::new(false);
    let read_only = RwSignal::new(false);
    let required = RwSignal::new(false);
    let invalid = RwSignal::new(false);
    let controls = vec![
        Control::action("mount", move || size.set(Some(SMALL))),
        Control::action("mount-large", move || size.set(Some(LARGE))),
        Control::action("unmount", move || size.set(None)),
        Control::action("select-first", move || value.set(Some(0))),
        Control::action("select-last", move || {
            value.set(size.get_untracked().map(|count| count - 1));
        }),
        Control::action("select-none", move || value.set(None)),
        Control::action("make-horizontal", move || {
            orientation.set(Orientation::Horizontal);
        }),
        Control::action("make-vertical", move || {
            orientation.set(Orientation::Vertical);
        }),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
        Control::action("make-read-only", move || read_only.set(true)),
        Control::action("make-editable", move || read_only.set(false)),
        Control::action("require", move || required.set(true)),
        Control::action("make-optional", move || required.set(false)),
        Control::action("invalidate", move || invalid.set(true)),
        Control::action("validate", move || invalid.set(false)),
        Control::read("selected", move || {
            value
                .get_untracked()
                .and_then(|value| u32::try_from(value).ok())
                .map_or(-1.0, f64::from)
        }),
    ];
    view! {
        <h1>"RadioGroup performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        {move || {
            size.get()
                .map(|count| {
                    view! {
                        <RadioGroup
                            id="perf-radio-group"
                            aria_label="Items"
                            value=value
                            set_value=value
                            orientation=orientation
                            is_disabled=disabled
                            is_read_only=read_only
                            is_required=required
                            is_invalid=invalid
                        >
                            {(0..count)
                                .map(|item| {
                                    view! {
                                        <RadioField value=item>
                                            <RadioButton>"Item"</RadioButton>
                                        </RadioField>
                                    }
                                })
                                .collect_view()}
                        </RadioGroup>
                    }
                })
        }}
        <button id="perf-after">"After"</button>
    }
}
