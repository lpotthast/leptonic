//! The `MeterFill` (`#perf-meter-fill`) in a `Meter` (at 25 of 0 to 100), in a 200px track, for its
//! performance test (`leptonic/tests/perf_tests/atoms/meter_fill.rs`), mounted at first. Its
//! controls ([`Control`]) mount and unmount the fill (the `Meter` stays) and change the value (25
//! or 75).

use leptonic::atoms::meter::{Meter, MeterFill};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let value = RwSignal::new(25.0_f64);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("raise", move || value.set(75.0)),
        Control::action("lower", move || value.set(25.0)),
    ];
    view! {
        <h1>"MeterFill performance"</h1>
        <PerfControls controls />
        <Meter value=value aria_label="Progress">
            <div style="width: 200px; height: 8px;">
                <Show when=move || mounted.get()>
                    <MeterFill attr:id="perf-meter-fill" />
                </Show>
            </div>
        </Meter>
    }
}
