//! The `TextArea` of a `TextField` (`#perf-textarea`, labelled "Notes", value "Ada") and a
//! standalone `TextArea` (class `perf-standalone`), for its performance test
//! (`leptonic/tests/perf_tests/atoms/text_area.rs`): both mounted at first, between two plain
//! buttons (`#perf-before`, `#perf-after`). Its controls ([`Control`]) mount and unmount each
//! (the field stays) and change the standalone textarea's props (disabled, invalid).

use leptonic::atoms::{field::Label, input::TextArea, text_field::TextField};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let standalone_mounted = RwSignal::new(true);
    let disabled = RwSignal::new(false);
    let invalid = RwSignal::new(false);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("mount-standalone", move || standalone_mounted.set(true)),
        Control::action("unmount-standalone", move || standalone_mounted.set(false)),
        Control::action("disable-standalone", move || disabled.set(true)),
        Control::action("enable-standalone", move || disabled.set(false)),
        Control::action("invalidate-standalone", move || invalid.set(true)),
        Control::action("uninvalidate-standalone", move || invalid.set(false)),
    ];
    view! {
        <h1>"TextArea performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <TextField id="perf-textarea" default_value="Ada">
            <Label>"Notes"</Label>
            <Show when=move || mounted.get()>
                <TextArea />
            </Show>
        </TextField>
        <Show when=move || standalone_mounted.get()>
            <TextArea classes="perf-standalone" is_disabled=disabled is_invalid=invalid />
        </Show>
        <button id="perf-after">"After"</button>
    }
}
