//! The `SliderMark` (`#perf-slider-mark`, "Middle" at 50) of the custom `SliderMarks` of a
//! `Slider` (0 to 100 in steps of 1, at 30) with a 200px track and one thumb, for its performance
//! test (`leptonic/tests/perf_tests/atoms/slider_mark.rs`): the slider's values controlled by app
//! state (`values` + `set_values`), the mark mounted at first. Its controls ([`Control`]) mount
//! and unmount the mark (the marks stay), set the value (60: the mark in range, or 30 again), and
//! change the slider's orientation.

use leptonic::{
    Orientation,
    atoms::slider::{Slider, SliderMark, SliderMarks, SliderThumb, SliderTrack},
    hooks::slider::{CustomSliderMark, SliderMarkPlacement, SliderMarkValue},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The track (200 × 20 pixels), its thumb and the marks below it.
const STYLE: &str = "#perf-slider-track { position: relative; width: 200px; height: 20px; \
                     background: #ddd; margin: 20px; } \
                     .perf-thumb { width: 16px; height: 16px; background: #333; top: 50%; } \
                     .leptonic-SliderMarks { position: relative; width: 200px; height: 20px; \
                     margin: 0 20px; font-size: 8px; }";

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let values = RwSignal::new(vec![30.0_f64]);
    let orientation = RwSignal::new(Orientation::Horizontal);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("set-value", move || values.set(vec![60.0])),
        Control::action("reset-value", move || values.set(vec![30.0])),
        Control::action("make-vertical", move || {
            orientation.set(Orientation::Vertical);
        }),
        Control::action("make-horizontal", move || {
            orientation.set(Orientation::Horizontal);
        }),
    ];
    let middle = SliderMarkPlacement::Custom {
        marks: vec![CustomSliderMark {
            value: SliderMarkValue::Value(50.0),
            name: "Middle".into(),
        }],
    };
    view! {
        <h1>"SliderMark performance"</h1>
        <style>{STYLE}</style>
        <PerfControls controls />
        <Slider
            min_value=0.0
            max_value=100.0
            values=values
            set_values=values
            orientation=orientation
            aria_label="Volume"
        >
            <SliderTrack attr:id="perf-slider-track">
                <SliderThumb classes="perf-thumb" />
            </SliderTrack>
            <SliderMarks marks=middle let:marks>
                <For each=move || marks.get() key=|mark| mark.value.to_bits() let:mark>
                    <Show when=move || mounted.get()>
                        <SliderMark attr:id="perf-slider-mark" mark=mark.clone()>
                            "Middle"
                        </SliderMark>
                    </Show>
                </For>
            </SliderMarks>
        </Slider>
    }
}
