//! The `Description` of a `TextField` ("Shown to others."), for its performance test
//! (`leptonic/tests/perf_tests/atoms/description.rs`): mounted at first, in a text field
//! labelled "Name" whose input (`#perf-input`) references it while it is rendered. Between two
//! plain buttons (`#perf-before`, `#perf-after`). Its controls ([`Control`]) mount and unmount
//! it.

use leptonic::atoms::{
    field::{Description, Label},
    input::Input,
    text_field::TextField,
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
    ];
    view! {
        <h1>"Description performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <TextField id="perf-input">
            <Label>"Name"</Label>
            <Input />
            <Show when=move || mounted.get()>
                <Description>"Shown to others."</Description>
            </Show>
        </TextField>
        <button id="perf-after">"After"</button>
    }
}
