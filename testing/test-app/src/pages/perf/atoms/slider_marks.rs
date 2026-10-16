//! The `SliderMarks` (`#perf-slider-marks`) of a `Slider` (0 to 100 in steps of 1, at 30) with a
//! 200px track and one thumb, for its performance test
//! (`leptonic/tests/perf_tests/atoms/slider_marks.rs`): automatic named marks (21: every fifth
//! step) or three custom ones (Low, Middle, High), each a `SliderMark` with its name; the slider's
//! values controlled by app state (`values` + `set_values`), the automatic marks mounted at first.
//! Its controls ([`Control`]) mount the marks (automatic or custom) and unmount them (the slider
//! stays), set the value (60, or 30 again), and change the slider's maximum (200, or 100 again)
//! and step (10, or 1 again).

use leptonic::{
    atoms::slider::{Slider, SliderMark, SliderMarks, SliderThumb, SliderTrack},
    hooks::slider::{CustomSliderMark, SliderMarkPlacement, SliderMarkValue},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The track (200 × 20 pixels), its thumb and the marks below it.
const STYLE: &str = "#perf-slider-track { position: relative; width: 200px; height: 20px; \
                     background: #ddd; margin: 20px; } \
                     .perf-thumb { width: 16px; height: 16px; background: #333; top: 50%; } \
                     #perf-slider-marks { position: relative; width: 200px; height: 20px; \
                     margin: 0 20px; font-size: 8px; }";

/// The placement of the mounted marks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Marks {
    /// Named marks along the steps.
    Automatic,
    /// Low (0), Middle (50) and High (100).
    Custom,
}

/// The placement of `marks`.
fn placement(marks: Marks) -> SliderMarkPlacement {
    match marks {
        Marks::Automatic => SliderMarkPlacement::Automatic { create_names: true },
        Marks::Custom => SliderMarkPlacement::Custom {
            marks: vec![
                CustomSliderMark {
                    value: SliderMarkValue::Value(0.0),
                    name: "Low".into(),
                },
                CustomSliderMark {
                    value: SliderMarkValue::Value(50.0),
                    name: "Middle".into(),
                },
                CustomSliderMark {
                    value: SliderMarkValue::Value(100.0),
                    name: "High".into(),
                },
            ],
        },
    }
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let marks = RwSignal::new(Some(Marks::Automatic));
    let values = RwSignal::new(vec![30.0_f64]);
    let max = RwSignal::new(100.0_f64);
    let step = RwSignal::new(1.0_f64);
    let controls = vec![
        Control::action("mount", move || marks.set(Some(Marks::Automatic))),
        Control::action("mount-custom", move || marks.set(Some(Marks::Custom))),
        Control::action("unmount", move || marks.set(None)),
        Control::action("set-value", move || values.set(vec![60.0])),
        Control::action("reset-value", move || values.set(vec![30.0])),
        Control::action("widen", move || max.set(200.0)),
        Control::action("narrow", move || max.set(100.0)),
        Control::action("coarsen", move || step.set(10.0)),
        Control::action("refine", move || step.set(1.0)),
    ];
    view! {
        <h1>"SliderMarks performance"</h1>
        <style>{STYLE}</style>
        <PerfControls controls />
        <Slider
            min_value=0.0
            max_value=max
            step=step
            values=values
            set_values=values
            aria_label="Volume"
        >
            <SliderTrack attr:id="perf-slider-track">
                <SliderThumb classes="perf-thumb" />
            </SliderTrack>
            {move || {
                marks
                    .get()
                    .map(|kind| {
                        view! {
                            <SliderMarks
                                attr:id="perf-slider-marks"
                                marks=placement(kind)
                                let:marks
                            >
                                <For
                                    each=move || marks.get()
                                    key=|mark| mark.value.to_bits()
                                    let:mark
                                >
                                    <SliderMark mark=mark
                                        .clone()>{mark.name.unwrap_or_default()}</SliderMark>
                                </For>
                            </SliderMarks>
                        }
                    })
            }}
        </Slider>
    }
}
