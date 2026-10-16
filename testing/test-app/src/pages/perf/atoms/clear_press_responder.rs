//! The `ClearPressResponder` alone, around a `Button` (`#perf-button`), inside a
//! `PressResponder` (whose own pressable child is `#perf-trigger`), for its performance test
//! (`leptonic/tests/perf_tests/atoms/clear_press_responder.rs`): between two plain buttons
//! (`#perf-before`, `#perf-after`), mounted at first. Its controls ([`Control`]) mount and
//! unmount it, and read how often the surrounding responder (`responder-presses`) and the button
//! (`presses`) saw a press.

use leptonic::atoms::{
    button::Button,
    press::{ClearPressResponder, PressResponder},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let press_count = StoredValue::new(0_u32);
    let responder_press_count = StoredValue::new(0_u32);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::read("presses", move || f64::from(press_count.get_value())),
        Control::read("responder-presses", move || {
            f64::from(responder_press_count.get_value())
        }),
    ];
    view! {
        <h1>"ClearPressResponder performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <PressResponder on_press=move |_| {
            responder_press_count.update_value(|count| *count += 1);
        }>
            <Button id="perf-trigger">"Trigger"</Button>
            <Show when=move || mounted.get()>
                <ClearPressResponder>
                    <Button
                        id="perf-button"
                        on_press=move |_| press_count.update_value(|count| *count += 1)
                    >
                        "Press me"
                    </Button>
                </ClearPressResponder>
            </Show>
        </PressResponder>
        <button id="perf-after">"After"</button>
    }
}
