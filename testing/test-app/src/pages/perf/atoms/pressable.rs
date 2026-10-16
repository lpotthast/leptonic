//! The `Pressable` alone, on a `<span role="button">` (`#perf-pressable`), for its performance
//! test (`leptonic/tests/perf_tests/atoms/pressable.rs`): between two plain buttons
//! (`#perf-before`, `#perf-after`) to tab from and to, mounted at first, its pressed state (from
//! `on_press_change`) as `data-pressed` on the span. Its controls ([`Control`]) mount and
//! unmount it, disable and enable it, and read how often it was pressed (`presses`) and
//! double-pressed (`double-presses`).

use leptonic::atoms::press::Pressable;
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let disabled = RwSignal::new(false);
    let pressed = RwSignal::new(false);
    let press_count = StoredValue::new(0_u32);
    let double_press_count = StoredValue::new(0_u32);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
        Control::read("presses", move || f64::from(press_count.get_value())),
        Control::read("double-presses", move || {
            f64::from(double_press_count.get_value())
        }),
    ];
    view! {
        <h1>"Pressable performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <Pressable
                is_disabled=disabled
                on_press=move |_| press_count.update_value(|count| *count += 1)
                on_double_press=move |_| double_press_count.update_value(|count| *count += 1)
                on_press_change=move |is_pressed| pressed.set(is_pressed)
            >
                <span
                    id="perf-pressable"
                    role="button"
                    data-pressed=move || pressed.get().then_some("true")
                >
                    "Press me"
                </span>
            </Pressable>
        </Show>
        <button id="perf-after">"After"</button>
    }
}
