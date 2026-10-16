//! The `ToggleButtonGroup` alone: a single-selection group (`#perf-group`, a `radiogroup` of the
//! buttons `#perf-a`, `#perf-b`, `#perf-c`), uncontrolled, and a controlled multiple-selection
//! group (`#perf-multiple`, a `toolbar` of `#perf-multiple-a`, `#perf-multiple-b`,
//! `#perf-multiple-c`), for its performance test
//! (`leptonic/tests/perf_tests/atoms/toggle_button_group.rs`): between two plain buttons
//! (`#perf-before`, `#perf-after`) to tab from and to, both mounted at first. Its controls
//! ([`Control`]) mount and unmount the single-selection group, disable and enable it, change its
//! orientation, and change the multiple-selection group's value (`select-all`, `clear`).

use std::collections::HashSet;

use leptonic::{
    Orientation,
    atoms::toggle_button::{ToggleButton, ToggleButtonGroup},
    hooks::{button::ToggleGroupSelectionMode, collections::Key},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// No selected keys: the value type of the single-selection group.
fn no_keys() -> HashSet<Key> {
    HashSet::new()
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let disabled = RwSignal::new(false);
    let orientation = RwSignal::new(Orientation::Horizontal);
    let multiple = RwSignal::new(HashSet::<Key>::new());
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
        Control::action("vertical", move || orientation.set(Orientation::Vertical)),
        Control::action("horizontal", move || {
            orientation.set(Orientation::Horizontal);
        }),
        Control::action("select-all", move || {
            multiple.set(["a", "b", "c"].into_iter().map(Key::from).collect());
        }),
        Control::action("clear", move || multiple.set(HashSet::new())),
    ];
    view! {
        <h1>"ToggleButtonGroup performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <ToggleButtonGroup
                attr:id="perf-group"
                default_value=no_keys()
                aria_label="Single"
                is_disabled=disabled
                orientation=orientation
            >
                <ToggleButton value="a" attr:id="perf-a">
                    "A"
                </ToggleButton>
                <ToggleButton value="b" attr:id="perf-b">
                    "B"
                </ToggleButton>
                <ToggleButton value="c" attr:id="perf-c">
                    "C"
                </ToggleButton>
            </ToggleButtonGroup>
        </Show>
        <button id="perf-after">"After"</button>
        <ToggleButtonGroup
            attr:id="perf-multiple"
            aria_label="Multiple"
            selection_mode=ToggleGroupSelectionMode::Multiple
            value=multiple
            set_value=multiple
        >
            <ToggleButton value="a" attr:id="perf-multiple-a">
                "A"
            </ToggleButton>
            <ToggleButton value="b" attr:id="perf-multiple-b">
                "B"
            </ToggleButton>
            <ToggleButton value="c" attr:id="perf-multiple-c">
                "C"
            </ToggleButton>
        </ToggleButtonGroup>
    }
}
