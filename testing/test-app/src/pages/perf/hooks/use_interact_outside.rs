//! `use_interact_outside` on a plain `<div>` (`#perf-inside`, with a button `#perf-inner`), for
//! its performance test (`leptonic/tests/perf_tests/hooks/use_interact_outside.rs`): next to a
//! plain button (`#perf-outside`) to click outside, mounted at first, counting interactions
//! outside that start and complete. Its controls ([`Control`]) mount and unmount it, disable and
//! enable it, and read the counts (`starts`, `interactions`).

use leptonic::hooks::interactions::{UseInteractOutsideInput, use_interact_outside};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The hook's input and counters.
#[derive(Clone, Copy)]
struct Inputs {
    disabled: RwSignal<bool>,
    start_count: StoredValue<u32>,
    interaction_count: StoredValue<u32>,
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let inputs = Inputs {
        disabled: RwSignal::new(false),
        start_count: StoredValue::new(0),
        interaction_count: StoredValue::new(0),
    };
    let Inputs {
        disabled,
        start_count,
        interaction_count,
    } = inputs;
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
        Control::read("starts", move || f64::from(start_count.get_value())),
        Control::read("interactions", move || {
            f64::from(interaction_count.get_value())
        }),
    ];
    view! {
        <h1>"use_interact_outside performance"</h1>
        <PerfControls controls />
        <button id="perf-outside">"Outside"</button>
        <Show when=move || mounted.get()>
            <Inside inputs />
        </Show>
    }
}

/// The element with `use_interact_outside`.
#[component]
fn Inside(inputs: Inputs) -> impl IntoView {
    let outside = use_interact_outside(UseInteractOutsideInput {
        is_disabled: inputs.disabled.into(),
        on_interact_outside_start: Some(Callback::new(move |_| {
            inputs.start_count.update_value(|count| *count += 1);
        })),
        on_interact_outside: Some(Callback::new(move |_| {
            inputs.interaction_count.update_value(|count| *count += 1);
        })),
        element: None,
    });
    view! {
        <div id="perf-inside" {..outside.target.attrs()}>
            <button id="perf-inner">"Inside"</button>
        </div>
    }
}
