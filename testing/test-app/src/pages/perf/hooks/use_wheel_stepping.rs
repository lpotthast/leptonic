//! `use_wheel_stepping` on a plain field (`#perf-group` around the input `#perf-input`), for its
//! performance test (`leptonic/tests/perf_tests/hooks/use_wheel_stepping.rs`): between two plain
//! buttons (`#perf-before`, `#perf-after`) to tab from and to, mounted at first, its value a
//! count shown in the input. Its controls ([`Control`]) mount and unmount it, make it read-only
//! and editable again, and read the count (`count`).

use leptonic::hooks::interactions::{
    UseWheelSteppingInput, UseWheelSteppingReturn, use_wheel_stepping,
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let read_only = RwSignal::new(false);
    let count = RwSignal::new(0_i32);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("read-only", move || read_only.set(true)),
        Control::action("editable", move || read_only.set(false)),
        Control::read("count", move || f64::from(count.get_untracked())),
    ];
    view! {
        <h1>"use_wheel_stepping performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <Field read_only count />
        </Show>
        <button id="perf-after">"After"</button>
    }
}

/// The field with `use_wheel_stepping`.
#[component]
fn Field(read_only: RwSignal<bool>, count: RwSignal<i32>) -> impl IntoView {
    let UseWheelSteppingReturn { group, input } = use_wheel_stepping(UseWheelSteppingInput {
        is_read_only: read_only.into(),
        on_increment: Some(Callback::new(move |()| count.update(|c| *c += 1))),
        on_decrement: Some(Callback::new(move |()| count.update(|c| *c -= 1))),
        ..UseWheelSteppingInput::default()
    });
    view! {
        <div id="perf-group" {..group.attrs()}>
            <input id="perf-input" {..input.attrs()} prop:value=move || count.get().to_string() />
        </div>
    }
}
