//! The `SliderFill` (`#perf-slider-fill`) on the 200px `SliderTrack` of a `Slider` (0 to 100 in
//! steps of 1, at 30) with one thumb, for its performance test
//! (`leptonic/tests/perf_tests/atoms/slider_fill.rs`): the slider's values controlled by app state
//! (`values` + `set_values`), between two plain buttons (`#perf-before`, `#perf-after`), the fill
//! mounted at first, from the start. Its controls ([`Control`]) mount and unmount the fill (the
//! slider stays), set the value (60, or 30 again), set and unset the fill's `offset` (50), and
//! change the slider's orientation.

use leptonic::{
    Orientation,
    atoms::slider::{Slider, SliderFill, SliderThumb, SliderTrack},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The track (200 × 20 pixels), its fill and its thumb.
const STYLE: &str = "#perf-slider-track { position: relative; width: 200px; height: 20px; \
                     background: #ddd; margin: 20px; } \
                     #perf-slider-fill { height: 100%; background: #88f; } \
                     .perf-thumb { width: 16px; height: 16px; background: #333; top: 50%; }";

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let values = RwSignal::new(vec![30.0_f64]);
    let offset = RwSignal::new(None::<f64>);
    let orientation = RwSignal::new(Orientation::Horizontal);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("set-value", move || values.set(vec![60.0])),
        Control::action("reset-value", move || values.set(vec![30.0])),
        Control::action("set-offset", move || offset.set(Some(50.0))),
        Control::action("unset-offset", move || offset.set(None)),
        Control::action("make-vertical", move || {
            orientation.set(Orientation::Vertical);
        }),
        Control::action("make-horizontal", move || {
            orientation.set(Orientation::Horizontal);
        }),
    ];
    view! {
        <h1>"SliderFill performance"</h1>
        <style>{STYLE}</style>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Slider
            min_value=0.0
            max_value=100.0
            values=values
            set_values=values
            orientation=orientation
            aria_label="Volume"
        >
            <SliderTrack attr:id="perf-slider-track">
                <Show when=move || mounted.get()>
                    <SliderFill attr:id="perf-slider-fill" offset=offset />
                </Show>
                <SliderThumb classes="perf-thumb" />
            </SliderTrack>
        </Slider>
        <button id="perf-after">"After"</button>
    }
}
