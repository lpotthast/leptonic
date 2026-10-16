//! The `PressResponder` alone, around a `Button` (`#perf-button`, its pressable child), for its
//! performance test (`leptonic/tests/perf_tests/atoms/press_responder.rs`): between two plain
//! buttons (`#perf-before`, `#perf-after`) to tab from and to, mounted at first. Its controls
//! ([`Control`]) mount and unmount it, change the props it gives the button (disabled, the forced
//! pressed state), and read how often the responder (`presses`) and the button
//! (`button-presses`) saw a press.

use leptonic::atoms::{button::Button, press::PressResponder};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let disabled = RwSignal::new(false);
    let forced = RwSignal::new(false);
    let press_count = StoredValue::new(0_u32);
    let button_press_count = StoredValue::new(0_u32);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
        Control::action("force-pressed", move || forced.set(true)),
        Control::action("unforce-pressed", move || forced.set(false)),
        Control::read("presses", move || f64::from(press_count.get_value())),
        Control::read("button-presses", move || {
            f64::from(button_press_count.get_value())
        }),
    ];
    view! {
        <h1>"PressResponder performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <PressResponder
                is_disabled=disabled
                force_is_pressed=forced
                on_press=move |_| press_count.update_value(|count| *count += 1)
            >
                <Button
                    id="perf-button"
                    on_press=move |_| button_press_count.update_value(|count| *count += 1)
                >
                    "Press me"
                </Button>
            </PressResponder>
        </Show>
        <button id="perf-after">"After"</button>
    }
}
