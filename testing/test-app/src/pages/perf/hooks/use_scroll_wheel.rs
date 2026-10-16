//! `use_scroll_wheel` on a plain `<div>` (`#perf-wheel`), for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_scroll_wheel.rs`): mounted at first, counting scrolls
//! and adding up their vertical deltas. Its controls ([`Control`]) mount and unmount it,
//! disable and enable it, and read the count (`scrolls`) and the sum (`delta-y`).

use leptonic::{
    IntoAttrs,
    hooks::interactions::{ScrollEvent, UseScrollWheelInput, use_scroll_wheel},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let disabled = RwSignal::new(false);
    let scroll_count = StoredValue::new(0_u32);
    let delta_y = StoredValue::new(0.0_f64);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
        Control::read("scrolls", move || f64::from(scroll_count.get_value())),
        Control::read("delta-y", move || delta_y.get_value()),
    ];
    view! {
        <h1>"use_scroll_wheel performance"</h1>
        <PerfControls controls />
        <Show when=move || mounted.get()>
            <Wheeled disabled scroll_count delta_y />
        </Show>
    }
}

/// The element with `use_scroll_wheel`.
#[component]
fn Wheeled(
    disabled: RwSignal<bool>,
    scroll_count: StoredValue<u32>,
    delta_y: StoredValue<f64>,
) -> impl IntoView {
    let wheel = use_scroll_wheel(UseScrollWheelInput {
        is_disabled: disabled.into(),
        on_scroll: Some(Callback::new(move |e: ScrollEvent| {
            scroll_count.update_value(|count| *count += 1);
            delta_y.update_value(|sum| *sum += e.delta_y);
        })),
    });
    view! {
        <div id="perf-wheel" style="width: 200px; height: 100px" {..wheel.props.into_attrs()}>
            "Scroll here"
        </div>
    }
}
