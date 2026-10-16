//! The `SliderOutput` (`#perf-slider-output`) of a `Slider` (0 to 100 in steps of 1, at 30) with a
//! 200px track and one thumb, for its performance test
//! (`leptonic/tests/perf_tests/atoms/slider_output.rs`): the slider's values controlled by app
//! state (`values` + `set_values`), the output mounted at first. Its controls ([`Control`]) mount
//! and unmount the output (the slider stays), set the value (60, or 30 again) and change the
//! slider's format (one fraction digit, or none again).

use leptonic::{
    NumberFormatOptions,
    atoms::slider::{Slider, SliderOutput, SliderThumb, SliderTrack},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The track (200 × 20 pixels) and its thumb.
const STYLE: &str = "#perf-slider-track { position: relative; width: 200px; height: 20px; \
                     background: #ddd; margin: 20px; } \
                     .perf-thumb { width: 16px; height: 16px; background: #333; top: 50%; }";

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let values = RwSignal::new(vec![30.0_f64]);
    let format = RwSignal::new(NumberFormatOptions::default());
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("set-value", move || values.set(vec![60.0])),
        Control::action("reset-value", move || values.set(vec![30.0])),
        Control::action("format-fraction", move || {
            format.set(NumberFormatOptions {
                minimum_fraction_digits: Some(1),
                ..NumberFormatOptions::default()
            });
        }),
        Control::action("format-integer", move || {
            format.set(NumberFormatOptions::default());
        }),
    ];
    view! {
        <h1>"SliderOutput performance"</h1>
        <style>{STYLE}</style>
        <PerfControls controls />
        <Slider
            min_value=0.0
            max_value=100.0
            values=values
            set_values=values
            format_options=format
            aria_label="Volume"
        >
            <Show when=move || mounted.get()>
                <SliderOutput attr:id="perf-slider-output" />
            </Show>
            <SliderTrack attr:id="perf-slider-track">
                <SliderThumb classes="perf-thumb" />
            </SliderTrack>
        </Slider>
    }
}
